use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{
    CrawlSessionStatus, JobType, LiveCrawlSession, LiveLog, 
    CrawlSessionRequest, CrawlSessionResponse
};
use crate::crawler_orchestrator::Priority;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::{Duration, Instant};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::learning_engine::LearningEngine;
use crate::crawler_orchestrator::CrawlStrategy;

#[derive(Debug, Clone)]
pub struct CrawlJob {
    pub id: Uuid,
    pub dno_name: String,
    pub dno_key: String,
    pub year: i32,
    pub priority: Priority,
    pub job_type: JobType,
    pub strategy: Option<CrawlStrategy>,
    pub constraints: CrawlConstraints,
    pub retry_count: u32,
    pub max_retries: u32,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub created_by_user: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlConstraints {
    pub max_time_minutes: Option<u32>,
    pub max_pages: Option<u32>,
    pub max_depth: Option<u32>,
    pub max_concurrent_downloads: Option<u32>,
    pub respect_robots_txt: bool,
    pub delay_between_requests_ms: u32,
    pub allowed_domains: Option<Vec<String>>,
    pub blocked_domains: Option<Vec<String>>,
}

impl Default for CrawlConstraints {
    fn default() -> Self {
        Self {
            max_time_minutes: Some(60),
            max_pages: Some(1000),
            max_depth: Some(10),
            max_concurrent_downloads: Some(5),
            respect_robots_txt: true,
            delay_between_requests_ms: 1000,
            allowed_domains: None,
            blocked_domains: None,
        }
    }
}

#[derive(Debug)]
pub struct WorkerHandle {
    pub worker_id: Uuid,
    pub session_id: Uuid,
    pub dno_key: String,
    pub started_at: Instant,
    pub status: WorkerStatus,
    pub progress: f64,
    pub current_url: Option<String>,
    pub cancel_sender: mpsc::Sender<()>,
}

#[derive(Debug, Clone)]
pub enum WorkerStatus {
    Initializing,
    Searching,
    Crawling,
    Extracting,
    Finalizing,
    Completed,
    Failed(String),
    Cancelled,
}

pub struct MasterCrawlerOrchestrator {
    job_queue: Arc<RwLock<VecDeque<CrawlJob>>>,
    priority_queues: Arc<RwLock<HashMap<Priority, VecDeque<CrawlJob>>>>,
    active_workers: Arc<RwLock<HashMap<Uuid, WorkerHandle>>>,
    resource_governor: ResourceGovernor,
    live_logger: LiveProgressLogger,
    learning_engine: Arc<Mutex<LearningEngine>>,
    performance_monitor: PerformanceMonitor,
    failure_recovery: FailureRecoveryEngine,
    scheduler: JobScheduler,
    max_concurrent_workers: usize,
    shutdown_signal: Arc<RwLock<bool>>,
}

impl MasterCrawlerOrchestrator {
    pub fn new(max_concurrent_workers: usize) -> Self {
        let mut priority_queues = HashMap::new();
        priority_queues.insert(Priority::Critical, VecDeque::new());
        priority_queues.insert(Priority::High, VecDeque::new());
        priority_queues.insert(Priority::Medium, VecDeque::new());
        priority_queues.insert(Priority::Low, VecDeque::new());

        Self {
            job_queue: Arc::new(RwLock::new(VecDeque::new())),
            priority_queues: Arc::new(RwLock::new(priority_queues)),
            active_workers: Arc::new(RwLock::new(HashMap::new())),
            resource_governor: ResourceGovernor::new(),
            live_logger: LiveProgressLogger::new(),
            learning_engine: Arc::new(Mutex::new(LearningEngine::new())),
            performance_monitor: PerformanceMonitor::new(),
            failure_recovery: FailureRecoveryEngine::new(),
            scheduler: JobScheduler::new(),
            max_concurrent_workers,
            shutdown_signal: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Master Crawler Orchestrator with {} max workers", self.max_concurrent_workers);

        // Start background tasks
        let orchestrator_clone = self.clone_for_background();
        tokio::spawn(async move {
            orchestrator_clone.job_processing_loop().await;
        });

        let orchestrator_clone = self.clone_for_background();
        tokio::spawn(async move {
            orchestrator_clone.resource_monitoring_loop().await;
        });

        let orchestrator_clone = self.clone_for_background();
        tokio::spawn(async move {
            orchestrator_clone.scheduled_job_processor().await;
        });

        let orchestrator_clone = self.clone_for_background();
        tokio::spawn(async move {
            orchestrator_clone.performance_monitoring_loop().await;
        });

        Ok(())
    }

    pub async fn submit_job(&mut self, request: CrawlSessionRequest) -> Result<CrawlSessionResponse> {
        let job_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        // Get recommended strategy from learning engine
        let strategy = self.learning_engine.lock().await
            .recommend_strategy(&request.dno_key, request.year)
            .await
            .unwrap_or_default();

        let job = CrawlJob {
            id: job_id,
            dno_name: request.dno_name.clone(),
            dno_key: request.dno_key.clone(),
            year: request.year,
            priority: request.priority.unwrap_or(Priority::Medium),
            job_type: JobType::UserRequest,
            strategy: Some(strategy),
            constraints: request.constraints.unwrap_or_default(),
            retry_count: 0,
            max_retries: 3,
            scheduled_for: request.scheduled_for,
            created_by_user: request.created_by_user,
            created_at: Utc::now(),
        };

        // Create live session
        let session = LiveCrawlSession {
            session_id,
            dno_name: job.dno_name.clone(),
            dno_key: job.dno_key.clone(),
            year: job.year,
            status: CrawlSessionStatus::Queued,
            priority: job.priority as i32,
            progress_percentage: 0.0,
            current_phase: Some("Queued".to_string()),
            current_url: None,
            pages_visited: 0,
            files_downloaded: 0,
            data_extracted: 0,
            errors_encountered: 0,
            estimated_completion: None,
            worker_thread_id: None,
            parent_session_id: None,
            created_by_user: job.created_by_user.clone(),
            started_at: None,
            completed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store session
        self.live_logger.create_session(session.clone()).await?;

        // Add job to appropriate priority queue
        self.add_job_to_queue(job.clone()).await?;

        // Log job submission
        self.live_logger.log(session_id, "info".to_string(), 
            format!("Job submitted for {} {}", job.dno_name, job.year),
            Some(serde_json::json!({
                "job_id": job_id,
                "priority": job.priority,
                "job_type": job.job_type
            }))
        ).await?;

        // Trigger job processing
        self.trigger_job_processing().await;

        Ok(CrawlSessionResponse {
            session_id,
            status: CrawlSessionStatus::Queued,
            estimated_start_time: self.estimate_start_time(&job).await,
            queue_position: self.get_queue_position(&job).await,
        })
    }

    pub async fn submit_automated_job(&mut self, dno_key: String, year: i32, job_type: JobType) -> Result<Uuid> {
        let job_id = Uuid::new_v4();

        let strategy = self.learning_engine.lock().await
            .recommend_strategy(&dno_key, year)
            .await
            .unwrap_or_default();

        let job = CrawlJob {
            id: job_id,
            dno_name: dno_key.clone(), // Will be resolved during processing
            dno_key: dno_key.clone(),
            year,
            priority: match job_type {
                JobType::AutomatedDiscovery => Priority::Low,
                JobType::HistoricalBackfill => Priority::Medium,
                JobType::Verification => Priority::High,
                JobType::UserRequest => Priority::Medium,
            },
            job_type,
            strategy: Some(strategy),
            constraints: CrawlConstraints::default(),
            retry_count: 0,
            max_retries: 2,
            scheduled_for: None,
            created_by_user: None,
            created_at: Utc::now(),
        };

        self.add_job_to_queue(job).await?;
        self.trigger_job_processing().await;

        Ok(job_id)
    }

    pub async fn schedule_job(&mut self, job: CrawlJob, scheduled_for: DateTime<Utc>) -> Result<Uuid> {
        let mut scheduled_job = job;
        scheduled_job.scheduled_for = Some(scheduled_for);

        self.scheduler.schedule_job(scheduled_job).await?;
        Ok(scheduled_job.id)
    }

    pub async fn cancel_job(&mut self, session_id: Uuid) -> Result<()> {
        // If job is running, cancel the worker
        if let Some(worker) = self.active_workers.read().await.get(&session_id) {
            if let Err(e) = worker.cancel_sender.send(()).await {
                warn!("Failed to send cancel signal to worker: {}", e);
            }
        }

        // Update session status
        self.live_logger.update_session_status(session_id, CrawlSessionStatus::Cancelled).await?;

        // Remove from queues
        self.remove_job_from_queues(session_id).await;

        info!("Job {} cancelled", session_id);
        Ok(())
    }

    pub async fn pause_job(&mut self, session_id: Uuid) -> Result<()> {
        if let Some(worker) = self.active_workers.write().await.get_mut(&session_id) {
            worker.status = WorkerStatus::Cancelled; // Temporarily use cancelled status
        }

        self.live_logger.update_session_status(session_id, CrawlSessionStatus::Paused).await?;
        info!("Job {} paused", session_id);
        Ok(())
    }

    pub async fn resume_job(&mut self, session_id: Uuid) -> Result<()> {
        self.live_logger.update_session_status(session_id, CrawlSessionStatus::Queued).await?;
        self.trigger_job_processing().await;
        info!("Job {} resumed", session_id);
        Ok(())
    }

    pub async fn get_active_sessions(&self) -> Result<Vec<LiveCrawlSession>> {
        self.live_logger.get_active_sessions().await
    }

    pub async fn get_session_status(&self, session_id: Uuid) -> Result<Option<LiveCrawlSession>> {
        self.live_logger.get_session(session_id).await
    }

    pub async fn get_session_logs(&self, session_id: Uuid, limit: Option<u32>) -> Result<Vec<LiveLog>> {
        self.live_logger.get_session_logs(session_id, limit).await
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down Master Crawler Orchestrator");

        // Set shutdown signal
        *self.shutdown_signal.write().await = true;

        // Cancel all active workers
        let workers = self.active_workers.read().await;
        for worker in workers.values() {
            if let Err(e) = worker.cancel_sender.send(()).await {
                warn!("Failed to send shutdown signal to worker {}: {}", worker.worker_id, e);
            }
        }

        // Wait for workers to finish (with timeout)
        let shutdown_timeout = Duration::from_secs(30);
        let start = Instant::now();

        while !self.active_workers.read().await.is_empty() && start.elapsed() < shutdown_timeout {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        if !self.active_workers.read().await.is_empty() {
            warn!("Some workers did not shutdown gracefully within timeout");
        }

        info!("Master Crawler Orchestrator shutdown complete");
        Ok(())
    }

    async fn job_processing_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_millis(500));

        loop {
            interval.tick().await;

            if *self.shutdown_signal.read().await {
                break;
            }

            if let Err(e) = self.process_job_queues().await {
                error!("Error in job processing loop: {}", e);
            }
        }
    }

    async fn process_job_queues(&self) -> Result<()> {
        let active_count = self.active_workers.read().await.len();
        
        if active_count >= self.max_concurrent_workers {
            return Ok(());
        }

        // Check if we can spawn more workers
        if !self.resource_governor.can_spawn_worker().await {
            return Ok(());
        }

        // Process priority queues in order
        let priorities = [Priority::Critical, Priority::High, Priority::Medium, Priority::Low];
        
        for priority in priorities {
            if let Some(job) = self.get_next_job_from_priority(priority).await {
                if let Err(e) = self.spawn_worker(job).await {
                    error!("Failed to spawn worker: {}", e);
                }
                break; // Only spawn one worker per iteration
            }
        }

        Ok(())
    }

    async fn get_next_job_from_priority(&self, priority: Priority) -> Option<CrawlJob> {
        let mut queues = self.priority_queues.write().await;
        queues.get_mut(&priority)?.pop_front()
    }

    async fn spawn_worker(&self, job: CrawlJob) -> Result<()> {
        let worker_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        // Create cancel channel
        let (cancel_sender, cancel_receiver) = mpsc::channel(1);

        // Create worker handle
        let worker_handle = WorkerHandle {
            worker_id,
            session_id,
            dno_key: job.dno_key.clone(),
            started_at: Instant::now(),
            status: WorkerStatus::Initializing,
            progress: 0.0,
            current_url: None,
            cancel_sender,
        };

        // Add to active workers
        self.active_workers.write().await.insert(session_id, worker_handle);

        // Create live session
        let session = LiveCrawlSession {
            session_id,
            dno_name: job.dno_name.clone(),
            dno_key: job.dno_key.clone(),
            year: job.year,
            status: CrawlSessionStatus::Initializing,
            priority: job.priority as i32,
            progress_percentage: 0.0,
            current_phase: Some("Initializing".to_string()),
            current_url: None,
            pages_visited: 0,
            files_downloaded: 0,
            data_extracted: 0,
            errors_encountered: 0,
            estimated_completion: None,
            worker_thread_id: Some(worker_id.to_string()),
            parent_session_id: None,
            created_by_user: job.created_by_user.clone(),
            started_at: Some(Utc::now()),
            completed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.live_logger.create_session(session).await?;

        // Spawn worker task
        let orchestrator_weak = Arc::downgrade(&self.active_workers);
        let live_logger = self.live_logger.clone();
        let learning_engine = Arc::clone(&self.learning_engine);

        tokio::spawn(async move {
            let result = Self::run_crawl_worker(
                job,
                session_id,
                worker_id,
                cancel_receiver,
                live_logger.clone(),
                learning_engine,
            ).await;

            // Clean up worker from active list
            if let Some(workers) = orchestrator_weak.upgrade() {
                workers.write().await.remove(&session_id);
            }

            // Update final status
            match result {
                Ok(_) => {
                    live_logger.update_session_status(session_id, CrawlSessionStatus::Completed).await.ok();
                }
                Err(e) => {
                    error!("Worker {} failed: {}", worker_id, e);
                    live_logger.update_session_status(session_id, CrawlSessionStatus::Failed).await.ok();
                }
            }
        });

        info!("Spawned worker {} for session {}", worker_id, session_id);
        Ok(())
    }

    async fn run_crawl_worker(
        job: CrawlJob,
        session_id: Uuid,
        worker_id: Uuid,
        mut cancel_receiver: mpsc::Receiver<()>,
        live_logger: LiveProgressLogger,
        learning_engine: Arc<Mutex<LearningEngine>>,
    ) -> Result<()> {
        info!("Worker {} starting crawl for {} {}", worker_id, job.dno_name, job.year);

        // Update status to searching
        live_logger.update_session_status(session_id, CrawlSessionStatus::Searching).await?;
        live_logger.log(session_id, "info".to_string(), 
            "Starting search phase".to_string(), None).await?;

        // Check for cancellation
        if cancel_receiver.try_recv().is_ok() {
            return Ok(());
        }

        // Simulate crawl phases
        let phases = [
            (CrawlSessionStatus::Searching, "Searching for DNO website", 20.0),
            (CrawlSessionStatus::Crawling, "Crawling website for data", 60.0),
            (CrawlSessionStatus::Extracting, "Extracting and processing data", 90.0),
        ];

        for (status, message, progress) in phases {
            live_logger.update_session_status(session_id, status).await?;
            live_logger.update_session_progress(session_id, progress).await?;
            live_logger.log(session_id, "info".to_string(), message.to_string(), None).await?;

            // Simulate work
            tokio::time::sleep(Duration::from_secs(2)).await;

            // Check for cancellation
            if cancel_receiver.try_recv().is_ok() {
                live_logger.log(session_id, "info".to_string(), 
                    "Crawl cancelled by user".to_string(), None).await?;
                return Ok(());
            }
        }

        // Complete the job
        live_logger.update_session_progress(session_id, 100.0).await?;
        live_logger.log(session_id, "info".to_string(), 
            "Crawl completed successfully".to_string(), 
            Some(serde_json::json!({
                "pages_visited": 42,
                "files_downloaded": 3,
                "data_extracted": 1
            }))
        ).await?;

        info!("Worker {} completed crawl for {} {}", worker_id, job.dno_name, job.year);
        Ok(())
    }

    // Helper methods and background tasks
    async fn add_job_to_queue(&self, job: CrawlJob) -> Result<()> {
        let mut queues = self.priority_queues.write().await;
        if let Some(queue) = queues.get_mut(&job.priority) {
            queue.push_back(job);
        }
        Ok(())
    }

    async fn remove_job_from_queues(&self, session_id: Uuid) {
        let mut queues = self.priority_queues.write().await;
        for queue in queues.values_mut() {
            queue.retain(|job| job.id != session_id);
        }
    }

    async fn trigger_job_processing(&self) {
        // This method triggers the job processing loop to check for new jobs
        // In a real implementation, this might use a condition variable or event
    }

    async fn estimate_start_time(&self, job: &CrawlJob) -> Option<DateTime<Utc>> {
        // Estimate when the job will start based on queue position and current load
        let queue_position = self.get_queue_position(job).await;
        let avg_job_time = Duration::from_secs(300); // 5 minutes average
        
        Some(Utc::now() + chrono::Duration::from_std(avg_job_time * queue_position as u32).ok()?)
    }

    async fn get_queue_position(&self, job: &CrawlJob) -> usize {
        let queues = self.priority_queues.read().await;
        let mut position = 0;

        // Count higher priority jobs
        let priorities = [Priority::Critical, Priority::High, Priority::Medium, Priority::Low];
        for priority in priorities {
            if priority == job.priority {
                break;
            }
            if let Some(queue) = queues.get(&priority) {
                position += queue.len();
            }
        }

        // Add position within same priority queue
        if let Some(queue) = queues.get(&job.priority) {
            for (i, queued_job) in queue.iter().enumerate() {
                if queued_job.id == job.id {
                    position += i;
                    break;
                }
            }
        }

        position
    }

    fn clone_for_background(&self) -> Self {
        // Create a lightweight clone for background tasks
        Self {
            job_queue: Arc::clone(&self.job_queue),
            priority_queues: Arc::clone(&self.priority_queues),
            active_workers: Arc::clone(&self.active_workers),
            resource_governor: self.resource_governor.clone(),
            live_logger: self.live_logger.clone(),
            learning_engine: Arc::clone(&self.learning_engine),
            performance_monitor: self.performance_monitor.clone(),
            failure_recovery: self.failure_recovery.clone(),
            scheduler: self.scheduler.clone(),
            max_concurrent_workers: self.max_concurrent_workers,
            shutdown_signal: Arc::clone(&self.shutdown_signal),
        }
    }

    async fn resource_monitoring_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;

            if *self.shutdown_signal.read().await {
                break;
            }

            self.resource_governor.update_resource_status().await;
        }
    }

    async fn scheduled_job_processor(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            if *self.shutdown_signal.read().await {
                break;
            }

            if let Ok(ready_jobs) = self.scheduler.get_ready_jobs().await {
                for job in ready_jobs {
                    if let Err(e) = self.add_job_to_queue(job).await {
                        error!("Failed to add scheduled job to queue: {}", e);
                    }
                }
            }
        }
    }

    async fn performance_monitoring_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(30));

        loop {
            interval.tick().await;

            if *self.shutdown_signal.read().await {
                break;
            }

            self.performance_monitor.collect_metrics(&self.active_workers).await;
        }
    }
}

// Supporting components
#[derive(Debug, Clone)]
struct ResourceGovernor {
    max_memory_mb: u64,
    max_cpu_percent: f64,
}

impl ResourceGovernor {
    fn new() -> Self {
        Self {
            max_memory_mb: 4096, // 4GB
            max_cpu_percent: 80.0,
        }
    }

    async fn can_spawn_worker(&self) -> bool {
        // Check system resources
        true // Simplified for now
    }

    async fn update_resource_status(&self) {
        // Monitor system resources
    }
}

#[derive(Debug, Clone)]
pub struct LiveProgressLogger {
    // WebSocket broadcaster and database logger would be implemented here
}

impl LiveProgressLogger {
    fn new() -> Self {
        Self {}
    }

    async fn create_session(&self, _session: LiveCrawlSession) -> Result<()> {
        Ok(())
    }

    async fn update_session_status(&self, _session_id: Uuid, _status: CrawlSessionStatus) -> Result<()> {
        Ok(())
    }

    async fn update_session_progress(&self, _session_id: Uuid, _progress: f64) -> Result<()> {
        Ok(())
    }

    async fn log(&self, _session_id: Uuid, _level: String, _message: String, _context: Option<serde_json::Value>) -> Result<()> {
        Ok(())
    }

    async fn get_active_sessions(&self) -> Result<Vec<LiveCrawlSession>> {
        Ok(vec![])
    }

    async fn get_session(&self, _session_id: Uuid) -> Result<Option<LiveCrawlSession>> {
        Ok(None)
    }

    async fn get_session_logs(&self, _session_id: Uuid, _limit: Option<u32>) -> Result<Vec<LiveLog>> {
        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
struct PerformanceMonitor;

impl PerformanceMonitor {
    fn new() -> Self { Self }
    async fn collect_metrics(&self, _workers: &Arc<RwLock<HashMap<Uuid, WorkerHandle>>>) {}
}

#[derive(Debug, Clone)]
struct FailureRecoveryEngine;

impl FailureRecoveryEngine {
    fn new() -> Self { Self }
}

#[derive(Debug, Clone)]
struct JobScheduler;

impl JobScheduler {
    fn new() -> Self { Self }
    async fn schedule_job(&self, _job: CrawlJob) -> Result<()> { Ok(()) }
    async fn get_ready_jobs(&self) -> Result<Vec<CrawlJob>> { Ok(vec![]) }
}