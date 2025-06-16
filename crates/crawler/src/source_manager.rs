use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use std::time::SystemTime;
use std::borrow::Cow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use thiserror::Error;
use tracing::{info, warn, error, debug, trace};
use tokio::fs as async_fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use shared::{
    DataSourceV2, SourceType, ExtractionMethod, AdminDataVerificationStatus,
    LiveCrawlSession, CrawlSessionStatus, LogLevel, LiveLog
};

/// Comprehensive source tracking and file management system
#[derive(Debug, Clone)]
pub struct SourceManager {
    /// Base directory for all DNO data (assets/dno-data)
    base_dir: PathBuf,
    /// In-memory cache of file metadata
    file_cache: HashMap<String, FileMetadata>,
    /// Audit trail of all operations
    audit_trail: Vec<AuditEntry>,
    /// Current session context
    current_session: Option<String>,
}

/// Detailed file metadata for tracking and integrity verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Unique identifier for this file entry
    pub id: String,
    /// DNO key (e.g., "netze-bw")
    pub dno_key: String,
    /// Year of the data
    pub year: i32,
    /// Relative path from base_dir
    pub relative_path: String,
    /// Absolute path to the file
    pub absolute_path: PathBuf,
    /// SHA-256 hash of file contents
    pub file_hash: String,
    /// File size in bytes
    pub file_size: u64,
    /// MIME type of the file
    pub content_type: String,
    /// Original source URL
    pub source_url: String,
    /// Final URL after redirects
    pub final_url: String,
    /// Source type classification
    pub source_type: SourceType,
    /// Extraction method used
    pub extraction_method: Option<ExtractionMethod>,
    /// Confidence score for data extraction
    pub extraction_confidence: Option<f64>,
    /// Extracted data structure
    pub extracted_data: Option<serde_json::Value>,
    /// File creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
    /// Last verification timestamp
    pub last_verified_at: Option<DateTime<Utc>>,
    /// Is this file currently active/valid
    pub is_active: bool,
    /// Admin verification status
    pub admin_verification_status: AdminDataVerificationStatus,
    /// Admin flagged for review
    pub admin_flagged: bool,
    /// Admin notes
    pub admin_notes: Option<String>,
    /// Session ID that created this file
    pub session_id: Option<String>,
    /// Provenance chain (how this file was discovered)
    pub provenance_chain: Vec<ProvenanceStep>,
    /// File integrity status
    pub integrity_status: FileIntegrityStatus,
    /// Deduplication references
    pub duplicate_references: Vec<String>,
}

/// Provenance tracking for complete audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceStep {
    /// Timestamp of this step
    pub timestamp: DateTime<Utc>,
    /// Type of action performed
    pub action: ProvenanceAction,
    /// Source of this action (crawler, admin, etc.)
    pub actor: String,
    /// Session ID if applicable
    pub session_id: Option<String>,
    /// Additional context
    pub context: Option<serde_json::Value>,
    /// URL or path related to this step
    pub source_reference: Option<String>,
}

/// Types of provenance actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProvenanceAction {
    /// File was discovered via web crawling
    Discovered,
    /// File was downloaded
    Downloaded,
    /// File was processed/analyzed
    Processed,
    /// File was verified by admin
    AdminVerified,
    /// File was flagged by admin
    AdminFlagged,
    /// File was moved or renamed
    Relocated,
    /// File was marked as duplicate
    Deduplicated,
    /// File integrity was verified
    IntegrityVerified,
    /// File was marked as corrupted
    CorruptionDetected,
    /// File was restored from backup
    Restored,
}

/// File integrity status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileIntegrityStatus {
    /// File is valid and intact
    Valid,
    /// File has been corrupted
    Corrupted { reason: String },
    /// File is missing
    Missing,
    /// File integrity is unknown
    Unknown,
}

/// Audit trail entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique identifier for this audit entry
    pub id: String,
    /// Timestamp of the operation
    pub timestamp: DateTime<Utc>,
    /// Operation type
    pub operation: AuditOperation,
    /// File or directory affected
    pub target: String,
    /// Actor performing the operation
    pub actor: String,
    /// Session ID if applicable
    pub session_id: Option<String>,
    /// Additional details
    pub details: Option<serde_json::Value>,
    /// Result of the operation
    pub result: OperationResult,
}

/// Types of auditable operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditOperation {
    /// File was created
    FileCreated,
    /// File was updated
    FileUpdated,
    /// File was deleted
    FileDeleted,
    /// File was moved
    FileMoved,
    /// Directory was created
    DirectoryCreated,
    /// Integrity check was performed
    IntegrityCheck,
    /// Deduplication was performed
    Deduplication,
    /// Admin review was performed
    AdminReview,
    /// Backup was created
    BackupCreated,
    /// Restoration was performed
    RestorationPerformed,
}

/// Result of an operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationResult {
    /// Operation succeeded
    Success,
    /// Operation failed
    Failed { error: String },
    /// Operation was skipped
    Skipped { reason: String },
}

/// Directory structure for DNO data
#[derive(Debug, Clone)]
pub struct DnoDirectory {
    /// DNO key
    pub dno_key: String,
    /// Year
    pub year: i32,
    /// Full path to directory
    pub path: PathBuf,
    /// Files in this directory
    pub files: Vec<FileMetadata>,
}

/// Deduplication result
#[derive(Debug, Clone)]
pub struct DeduplicationResult {
    /// Number of files analyzed
    pub files_analyzed: usize,
    /// Number of duplicates found
    pub duplicates_found: usize,
    /// Number of files deduplicated
    pub files_deduplicated: usize,
    /// Space saved in bytes
    pub space_saved: u64,
    /// Deduplication groups
    pub duplicate_groups: Vec<DuplicateGroup>,
}

/// Group of duplicate files
#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    /// Common hash
    pub hash: String,
    /// File size
    pub size: u64,
    /// Files in this group
    pub files: Vec<String>,
    /// Recommended action
    pub recommended_action: DeduplicationAction,
}

/// Recommended deduplication action
#[derive(Debug, Clone)]
pub enum DeduplicationAction {
    /// Keep the original, mark others as duplicates
    KeepOriginal { original: String },
    /// Manual review required
    ManualReview,
    /// All files are identical, safe to deduplicate
    SafeToDeduplicateAll,
}

/// Admin interface for source review
#[derive(Debug, Clone)]
pub struct AdminInterface {
    /// Reference to source manager
    source_manager: SourceManager,
}

/// Admin review result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminReviewResult {
    /// File ID being reviewed
    pub file_id: String,
    /// Review decision
    pub decision: AdminDecision,
    /// Reviewer notes
    pub notes: Option<String>,
    /// Timestamp of review
    pub reviewed_at: DateTime<Utc>,
    /// Reviewer ID
    pub reviewer_id: String,
}

/// Admin decision for file review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdminDecision {
    /// File is approved
    Approved,
    /// File is rejected
    Rejected { reason: String },
    /// File needs more review
    RequiresMoreReview { reason: String },
    /// File is flagged for special attention
    Flagged { reason: String },
}

/// Errors that can occur in source management
#[derive(Error, Debug)]
pub enum SourceManagerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Path error: {0}")]
    Path(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Integrity check failed: {0}")]
    IntegrityCheckFailed(String),
    
    #[error("Duplicate file detected: {0}")]
    DuplicateFile(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Concurrent modification detected: {0}")]
    ConcurrentModification(String),
}

impl SourceManager {
    /// Create a new source manager with the specified base directory
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self, SourceManagerError> {
        let base_dir = base_dir.as_ref().to_path_buf();
        
        // Ensure base directory exists
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
            info!("Created base directory: {}", base_dir.display());
        }
        
        let mut manager = Self {
            base_dir,
            file_cache: HashMap::new(),
            audit_trail: Vec::new(),
            current_session: None,
        };
        
        // Initialize by scanning existing files
        manager.scan_existing_files()?;
        
        Ok(manager)
    }
    
    /// Set the current session context
    pub fn set_session(&mut self, session_id: String) {
        self.current_session = Some(session_id);
    }
    
    /// Get the path for a specific DNO and year
    pub fn get_dno_path(&self, dno_key: &str, year: i32) -> PathBuf {
        self.base_dir.join("dno-data").join(dno_key).join(year.to_string())
    }
    
    /// Ensure directory structure exists for a DNO and year
    pub async fn ensure_directory_structure(&mut self, dno_key: &str, year: i32) -> Result<PathBuf, SourceManagerError> {
        let path = self.get_dno_path(dno_key, year);
        
        if !path.exists() {
            async_fs::create_dir_all(&path).await?;
            info!("Created directory structure: {}", path.display());
            
            // Record audit entry
            self.add_audit_entry(
                AuditOperation::DirectoryCreated,
                path.to_string_lossy().to_string(),
                "system".to_string(),
                None,
                OperationResult::Success,
            );
        }
        
        Ok(path)
    }
    
    /// Store a file in the appropriate directory structure
    pub async fn store_file(
        &mut self,
        dno_key: &str,
        year: i32,
        filename: &str,
        content: &[u8],
        source_url: &str,
        final_url: &str,
        source_type: SourceType,
        content_type: &str,
    ) -> Result<FileMetadata, SourceManagerError> {
        // Ensure directory exists
        let dir_path = self.ensure_directory_structure(dno_key, year).await?;
        let file_path = dir_path.join(filename);
        
        // Calculate hash
        let file_hash = self.calculate_hash(content);
        
        // Check for duplicates
        if let Some(existing) = self.find_duplicate_by_hash(&file_hash) {
            warn!("Duplicate file detected: {} (original: {})", file_path.display(), existing.absolute_path.display());
            
            // Create reference to existing file instead of storing duplicate
            let mut metadata = existing.clone();
            metadata.id = Uuid::new_v4().to_string();
            metadata.duplicate_references.push(existing.id.clone());
            
            // Add provenance step
            metadata.provenance_chain.push(ProvenanceStep {
                timestamp: Utc::now(),
                action: ProvenanceAction::Deduplicated,
                actor: "system".to_string(),
                session_id: self.current_session.clone(),
                context: Some(serde_json::json!({
                    "original_file": existing.absolute_path.to_string_lossy(),
                    "duplicate_hash": file_hash
                })),
                source_reference: Some(source_url.to_string()),
            });
            
            return Ok(metadata);
        }
        
        // Write file to disk
        let mut file = async_fs::File::create(&file_path).await?;
        file.write_all(content).await?;
        file.flush().await?;
        
        // Create metadata
        let metadata = FileMetadata {
            id: Uuid::new_v4().to_string(),
            dno_key: dno_key.to_string(),
            year,
            relative_path: file_path.strip_prefix(&self.base_dir)
                .unwrap_or(&file_path)
                .to_string_lossy()
                .to_string(),
            absolute_path: file_path.clone(),
            file_hash: file_hash.clone(),
            file_size: content.len() as u64,
            content_type: content_type.to_string(),
            source_url: source_url.to_string(),
            final_url: final_url.to_string(),
            source_type,
            extraction_method: None,
            extraction_confidence: None,
            extracted_data: None,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            last_verified_at: Some(Utc::now()),
            is_active: true,
            admin_verification_status: AdminDataVerificationStatus::Pending,
            admin_flagged: false,
            admin_notes: None,
            session_id: self.current_session.clone(),
            provenance_chain: vec![
                ProvenanceStep {
                    timestamp: Utc::now(),
                    action: ProvenanceAction::Discovered,
                    actor: "crawler".to_string(),
                    session_id: self.current_session.clone(),
                    context: Some(serde_json::json!({
                        "source_url": source_url,
                        "final_url": final_url
                    })),
                    source_reference: Some(source_url.to_string()),
                },
                ProvenanceStep {
                    timestamp: Utc::now(),
                    action: ProvenanceAction::Downloaded,
                    actor: "system".to_string(),
                    session_id: self.current_session.clone(),
                    context: Some(serde_json::json!({
                        "file_size": content.len(),
                        "content_type": content_type
                    })),
                    source_reference: Some(file_path.to_string_lossy().to_string()),
                }
            ],
            integrity_status: FileIntegrityStatus::Valid,
            duplicate_references: Vec::new(),
        };
        
        // Store in cache
        self.file_cache.insert(metadata.id.clone(), metadata.clone());
        
        // Record audit entry
        self.add_audit_entry(
            AuditOperation::FileCreated,
            file_path.to_string_lossy().to_string(),
            "system".to_string(),
            Some(serde_json::json!({
                "dno_key": dno_key,
                "year": year,
                "file_size": content.len(),
                "hash": file_hash
            })),
            OperationResult::Success,
        );
        
        info!("Stored file: {} ({})", file_path.display(), file_hash);
        
        Ok(metadata)
    }
    
    /// Update file metadata with extraction results
    pub async fn update_extraction_results(
        &mut self,
        file_id: &str,
        extraction_method: ExtractionMethod,
        extracted_data: serde_json::Value,
        confidence: Option<f64>,
    ) -> Result<(), SourceManagerError> {
        let metadata = self.file_cache.get_mut(file_id)
            .ok_or_else(|| SourceManagerError::FileNotFound(file_id.to_string()))?;
        
        let absolute_path = metadata.absolute_path.clone();
        
        metadata.extraction_method = Some(extraction_method.clone());
        metadata.extracted_data = Some(extracted_data.clone());
        metadata.extraction_confidence = confidence;
        metadata.modified_at = Utc::now();
        
        // Add provenance step
        metadata.provenance_chain.push(ProvenanceStep {
            timestamp: Utc::now(),
            action: ProvenanceAction::Processed,
            actor: "ai_processor".to_string(),
            session_id: self.current_session.clone(),
            context: Some(serde_json::json!({
                "extraction_method": extraction_method,
                "confidence": confidence,
                "data_size": extracted_data.to_string().len()
            })),
            source_reference: Some(absolute_path.to_string_lossy().to_string()),
        });
        
        // Record audit entry
        self.add_audit_entry(
            AuditOperation::FileUpdated,
            absolute_path.to_string_lossy().to_string(),
            "ai_processor".to_string(),
            Some(serde_json::json!({
                "extraction_method": extraction_method,
                "confidence": confidence
            })),
            OperationResult::Success,
        );
        
        debug!("Updated extraction results for file: {}", file_id);
        
        Ok(())
    }
    
    /// Verify file integrity
    pub async fn verify_file_integrity(&mut self, file_id: &str) -> Result<FileIntegrityStatus, SourceManagerError> {
        // Get file info first 
        let (absolute_path, expected_hash) = {
            let metadata = self.file_cache.get(file_id)
                .ok_or_else(|| SourceManagerError::FileNotFound(file_id.to_string()))?;
            (metadata.absolute_path.clone(), metadata.file_hash.clone())
        };
        
        // Check if file exists
        if !absolute_path.exists() {
            let metadata = self.file_cache.get_mut(file_id).unwrap();
            metadata.integrity_status = FileIntegrityStatus::Missing;
            error!("File missing: {}", absolute_path.display());
            return Ok(FileIntegrityStatus::Missing);
        }
        
        // Read file and calculate hash
        let mut file = async_fs::File::open(&absolute_path).await?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;
        
        let current_hash = self.calculate_hash(&content);
        
        if current_hash != expected_hash {
            let reason = format!("Hash mismatch: expected {}, got {}", expected_hash, current_hash);
            
            // Update metadata
            let metadata = self.file_cache.get_mut(file_id).unwrap();
            metadata.integrity_status = FileIntegrityStatus::Corrupted { reason: reason.clone() };
            
            // Add provenance step
            metadata.provenance_chain.push(ProvenanceStep {
                timestamp: Utc::now(),
                action: ProvenanceAction::CorruptionDetected,
                actor: "system".to_string(),
                session_id: self.current_session.clone(),
                context: Some(serde_json::json!({
                    "expected_hash": expected_hash,
                    "actual_hash": current_hash,
                    "reason": reason
                })),
                source_reference: Some(absolute_path.to_string_lossy().to_string()),
            });
            
            error!("File corruption detected: {}", absolute_path.display());
            return Ok(FileIntegrityStatus::Corrupted { reason });
        }
        
        // Update verification timestamp
        let metadata = self.file_cache.get_mut(file_id).unwrap();
        metadata.integrity_status = FileIntegrityStatus::Valid;
        metadata.last_verified_at = Some(Utc::now());
        
        // Add provenance step
        metadata.provenance_chain.push(ProvenanceStep {
            timestamp: Utc::now(),
            action: ProvenanceAction::IntegrityVerified,
            actor: "system".to_string(),
            session_id: self.current_session.clone(),
            context: Some(serde_json::json!({
                "hash": current_hash,
                "file_size": content.len()
            })),
            source_reference: Some(absolute_path.to_string_lossy().to_string()),
        });
        
        // Record audit entry
        self.add_audit_entry(
            AuditOperation::IntegrityCheck,
            absolute_path.to_string_lossy().to_string(),
            "system".to_string(),
            Some(serde_json::json!({
                "hash": current_hash,
                "status": "valid"
            })),
            OperationResult::Success,
        );
        
        debug!("File integrity verified: {}", file_id);
        
        Ok(FileIntegrityStatus::Valid)
    }
    
    /// Perform deduplication across all files
    pub async fn perform_deduplication(&mut self) -> Result<DeduplicationResult, SourceManagerError> {
        let mut hash_groups: HashMap<String, Vec<String>> = HashMap::new();
        let files_analyzed = self.file_cache.len();
        
        // Group files by hash
        for (file_id, metadata) in &self.file_cache {
            hash_groups.entry(metadata.file_hash.clone())
                .or_insert_with(Vec::new)
                .push(file_id.clone());
        }
        
        let mut duplicate_groups = Vec::new();
        let mut duplicates_found = 0;
        let mut files_deduplicated = 0;
        let mut space_saved = 0u64;
        
        // Process each hash group
        for (hash, file_ids) in hash_groups {
            if file_ids.len() > 1 {
                duplicates_found += file_ids.len() - 1;
                
                // Get file size (all files in group have same size)
                let file_size = self.file_cache.get(&file_ids[0])
                    .map(|m| m.file_size)
                    .unwrap_or(0);
                
                space_saved += file_size * (file_ids.len() - 1) as u64;
                
                // Determine recommended action
                let recommended_action = self.analyze_duplicate_group(&file_ids);
                
                duplicate_groups.push(DuplicateGroup {
                    hash: hash.clone(),
                    size: file_size,
                    files: file_ids.clone(),
                    recommended_action: recommended_action.clone(),
                });
                
                // Apply deduplication based on recommendation
                match recommended_action {
                    DeduplicationAction::KeepOriginal { original } => {
                        for file_id in &file_ids {
                            if file_id != &original {
                                self.mark_as_duplicate(file_id, &original)?;
                                files_deduplicated += 1;
                            }
                        }
                    }
                    DeduplicationAction::SafeToDeduplicateAll => {
                        // Keep the first file, mark others as duplicates
                        let original = &file_ids[0];
                        for file_id in file_ids.iter().skip(1) {
                            self.mark_as_duplicate(file_id, original)?;
                            files_deduplicated += 1;
                        }
                    }
                    DeduplicationAction::ManualReview => {
                        // Flag all files for manual review
                        for file_id in &file_ids {
                            if let Some(metadata) = self.file_cache.get_mut(file_id) {
                                metadata.admin_flagged = true;
                                metadata.admin_notes = Some("Duplicate detected - manual review required".to_string());
                            }
                        }
                    }
                }
            }
        }
        
        // Record audit entry
        self.add_audit_entry(
            AuditOperation::Deduplication,
            "system_wide".to_string(),
            "system".to_string(),
            Some(serde_json::json!({
                "files_analyzed": files_analyzed,
                "duplicates_found": duplicates_found,
                "files_deduplicated": files_deduplicated,
                "space_saved": space_saved
            })),
            OperationResult::Success,
        );
        
        info!("Deduplication complete: {} files analyzed, {} duplicates found, {} files deduplicated, {} bytes saved",
              files_analyzed, duplicates_found, files_deduplicated, space_saved);
        
        Ok(DeduplicationResult {
            files_analyzed,
            duplicates_found,
            files_deduplicated,
            space_saved,
            duplicate_groups,
        })
    }
    
    /// Get all files for a specific DNO and year
    pub fn get_files_for_dno_year(&self, dno_key: &str, year: i32) -> Vec<&FileMetadata> {
        self.file_cache.values()
            .filter(|metadata| metadata.dno_key == dno_key && metadata.year == year)
            .collect()
    }
    
    /// Get files requiring admin review
    pub fn get_files_requiring_review(&self) -> Vec<&FileMetadata> {
        self.file_cache.values()
            .filter(|metadata| {
                metadata.admin_verification_status == AdminDataVerificationStatus::Pending ||
                metadata.admin_flagged ||
                metadata.integrity_status != FileIntegrityStatus::Valid
            })
            .collect()
    }
    
    /// Get audit trail entries
    pub fn get_audit_trail(&self, limit: Option<usize>) -> Vec<&AuditEntry> {
        let mut entries: Vec<&AuditEntry> = self.audit_trail.iter().collect();
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            entries.truncate(limit);
        }
        
        entries
    }
    
    /// Get file metadata by ID
    pub fn get_file_metadata(&self, file_id: &str) -> Option<&FileMetadata> {
        self.file_cache.get(file_id)
    }
    
    /// Get directory structure for a DNO
    pub fn get_dno_directory(&self, dno_key: &str, year: i32) -> Option<DnoDirectory> {
        let path = self.get_dno_path(dno_key, year);
        
        if !path.exists() {
            return None;
        }
        
        let files = self.get_files_for_dno_year(dno_key, year)
            .into_iter()
            .cloned()
            .collect();
        
        Some(DnoDirectory {
            dno_key: dno_key.to_string(),
            year,
            path,
            files,
        })
    }
    
    /// Export metadata to JSON
    pub fn export_metadata(&self) -> Result<String, SourceManagerError> {
        let export_data = serde_json::json!({
            "files": self.file_cache.values().collect::<Vec<_>>(),
            "audit_trail": self.audit_trail,
            "export_timestamp": Utc::now(),
            "version": "1.0"
        });
        
        Ok(serde_json::to_string_pretty(&export_data)?)
    }
    
    /// Import metadata from JSON
    pub fn import_metadata(&mut self, json_data: &str) -> Result<(), SourceManagerError> {
        let import_data: serde_json::Value = serde_json::from_str(json_data)?;
        
        // Import files
        if let Some(files) = import_data.get("files") {
            if let Ok(files_vec) = serde_json::from_value::<Vec<FileMetadata>>(files.clone()) {
                for file_metadata in files_vec {
                    self.file_cache.insert(file_metadata.id.clone(), file_metadata);
                }
            }
        }
        
        // Import audit trail
        if let Some(audit_trail) = import_data.get("audit_trail") {
            if let Ok(audit_vec) = serde_json::from_value::<Vec<AuditEntry>>(audit_trail.clone()) {
                self.audit_trail.extend(audit_vec);
            }
        }
        
        info!("Imported metadata: {} files, {} audit entries", 
              self.file_cache.len(), self.audit_trail.len());
        
        Ok(())
    }
    
    // Private helper methods
    
    /// Calculate SHA-256 hash of content
    fn calculate_hash(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
    
    /// Find duplicate file by hash
    fn find_duplicate_by_hash(&self, hash: &str) -> Option<&FileMetadata> {
        self.file_cache.values()
            .find(|metadata| metadata.file_hash == hash && metadata.is_active)
    }
    
    /// Analyze a group of duplicate files to determine recommended action
    fn analyze_duplicate_group(&self, file_ids: &[String]) -> DeduplicationAction {
        let mut files_with_data = Vec::new();
        let mut files_without_data = Vec::new();
        
        for file_id in file_ids {
            if let Some(metadata) = self.file_cache.get(file_id) {
                if metadata.extracted_data.is_some() {
                    files_with_data.push(file_id.clone());
                } else {
                    files_without_data.push(file_id.clone());
                }
            }
        }
        
        // If some files have extracted data and others don't, prefer the ones with data
        if !files_with_data.is_empty() && !files_without_data.is_empty() {
            return DeduplicationAction::KeepOriginal { 
                original: files_with_data[0].clone() 
            };
        }
        
        // If files are from different sessions, require manual review
        let mut sessions = std::collections::HashSet::new();
        for file_id in file_ids {
            if let Some(metadata) = self.file_cache.get(file_id) {
                sessions.insert(metadata.session_id.clone());
            }
        }
        
        if sessions.len() > 1 {
            return DeduplicationAction::ManualReview;
        }
        
        // Otherwise, safe to deduplicate
        DeduplicationAction::SafeToDeduplicateAll
    }
    
    /// Mark a file as duplicate of another
    fn mark_as_duplicate(&mut self, duplicate_id: &str, original_id: &str) -> Result<(), SourceManagerError> {
        let duplicate_metadata = self.file_cache.get_mut(duplicate_id)
            .ok_or_else(|| SourceManagerError::FileNotFound(duplicate_id.to_string()))?;
        
        duplicate_metadata.is_active = false;
        duplicate_metadata.duplicate_references.push(original_id.to_string());
        duplicate_metadata.modified_at = Utc::now();
        
        // Add provenance step
        duplicate_metadata.provenance_chain.push(ProvenanceStep {
            timestamp: Utc::now(),
            action: ProvenanceAction::Deduplicated,
            actor: "system".to_string(),
            session_id: self.current_session.clone(),
            context: Some(serde_json::json!({
                "original_file_id": original_id,
                "deduplication_reason": "identical_hash"
            })),
            source_reference: Some(duplicate_metadata.absolute_path.to_string_lossy().to_string()),
        });
        
        debug!("Marked file {} as duplicate of {}", duplicate_id, original_id);
        
        Ok(())
    }
    
    /// Scan existing files and populate cache
    fn scan_existing_files(&mut self) -> Result<(), SourceManagerError> {
        let dno_data_dir = self.base_dir.join("dno-data");
        
        if !dno_data_dir.exists() {
            return Ok(());
        }
        
        // Recursively scan directory structure
        self.scan_directory_recursive(&dno_data_dir)?;
        
        info!("Scanned existing files: {} files found", self.file_cache.len());
        
        Ok(())
    }
    
    /// Recursively scan directory for files
    fn scan_directory_recursive(&mut self, dir: &Path) -> Result<(), SourceManagerError> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.scan_directory_recursive(&path)?;
            } else if path.is_file() {
                // Try to reconstruct metadata for existing files
                if let Ok(metadata) = self.reconstruct_file_metadata(&path) {
                    self.file_cache.insert(metadata.id.clone(), metadata);
                }
            }
        }
        
        Ok(())
    }
    
    /// Reconstruct file metadata for existing files
    fn reconstruct_file_metadata(&self, file_path: &Path) -> Result<FileMetadata, SourceManagerError> {
        let metadata = fs::metadata(file_path)?;
        let file_size = metadata.len();
        
        // Read file to calculate hash
        let content = fs::read(file_path)?;
        let file_hash = self.calculate_hash(&content);
        
        // Extract DNO key and year from path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .map_err(|_| SourceManagerError::Path("Invalid file path".to_string()))?;
        
        let path_components: Vec<&str> = relative_path.iter()
            .map(|os_str| os_str.to_str().unwrap_or(""))
            .collect();
        
        let (dno_key, year) = if path_components.len() >= 3 && path_components[0] == "dno-data" {
            let dno_key = path_components[1].to_string();
            let year = path_components[2].parse::<i32>()
                .map_err(|_| SourceManagerError::Path("Invalid year in path".to_string()))?;
            (dno_key, year)
        } else {
            return Err(SourceManagerError::Path("Invalid path structure".to_string()));
        };
        
        // Guess content type from extension
        let content_type = self.guess_content_type(file_path);
        
        // Guess source type from content type
        let source_type = match content_type.as_str() {
            "application/pdf" => SourceType::Pdf,
            "image/png" | "image/jpeg" | "image/gif" => SourceType::Image,
            "text/plain" | "text/html" => SourceType::Text,
            _ => SourceType::Webpage,
        };
        
        let created_at = metadata.created()
            .map(|t| DateTime::<Utc>::from(t))
            .unwrap_or_else(|_| Utc::now());
        
        let modified_at = metadata.modified()
            .map(|t| DateTime::<Utc>::from(t))
            .unwrap_or_else(|_| Utc::now());
        
        Ok(FileMetadata {
            id: Uuid::new_v4().to_string(),
            dno_key,
            year,
            relative_path: relative_path.to_string_lossy().to_string(),
            absolute_path: file_path.to_path_buf(),
            file_hash,
            file_size,
            content_type,
            source_url: "unknown".to_string(),
            final_url: "unknown".to_string(),
            source_type,
            extraction_method: None,
            extraction_confidence: None,
            extracted_data: None,
            created_at,
            modified_at,
            last_verified_at: None,
            is_active: true,
            admin_verification_status: AdminDataVerificationStatus::Pending,
            admin_flagged: false,
            admin_notes: None,
            session_id: None,
            provenance_chain: vec![
                ProvenanceStep {
                    timestamp: created_at,
                    action: ProvenanceAction::Discovered,
                    actor: "file_scanner".to_string(),
                    session_id: None,
                    context: Some(serde_json::json!({
                        "scan_type": "existing_file_reconstruction"
                    })),
                    source_reference: Some(file_path.to_string_lossy().to_string()),
                }
            ],
            integrity_status: FileIntegrityStatus::Unknown,
            duplicate_references: Vec::new(),
        })
    }
    
    /// Guess content type from file extension
    fn guess_content_type(&self, file_path: &Path) -> String {
        match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("pdf") => "application/pdf".to_string(),
            Some("png") => "image/png".to_string(),
            Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
            Some("gif") => "image/gif".to_string(),
            Some("html") | Some("htm") => "text/html".to_string(),
            Some("txt") => "text/plain".to_string(),
            Some("json") => "application/json".to_string(),
            Some("xml") => "application/xml".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }
    
    /// Add an audit entry
    fn add_audit_entry(
        &mut self,
        operation: AuditOperation,
        target: String,
        actor: String,
        details: Option<serde_json::Value>,
        result: OperationResult,
    ) {
        let entry = AuditEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            operation,
            target,
            actor,
            session_id: self.current_session.clone(),
            details,
            result,
        };
        
        self.audit_trail.push(entry);
        
        // Keep audit trail size reasonable
        if self.audit_trail.len() > 10000 {
            self.audit_trail.drain(0..1000);
        }
    }
}

impl AdminInterface {
    /// Create a new admin interface
    pub fn new(source_manager: SourceManager) -> Self {
        Self { source_manager }
    }
    
    /// Get files requiring admin review
    pub fn get_pending_reviews(&self) -> Vec<&FileMetadata> {
        self.source_manager.get_files_requiring_review()
    }
    
    /// Perform admin review of a file
    pub async fn review_file(
        &mut self,
        file_id: &str,
        decision: AdminDecision,
        reviewer_id: &str,
        notes: Option<String>,
    ) -> Result<AdminReviewResult, SourceManagerError> {
        // Get absolute path first
        let absolute_path = {
            let metadata = self.source_manager.file_cache.get(file_id)
                .ok_or_else(|| SourceManagerError::FileNotFound(file_id.to_string()))?;
            metadata.absolute_path.clone()
        };
        
        // Update metadata based on decision
        let metadata = self.source_manager.file_cache.get_mut(file_id).unwrap();
        match &decision {
            AdminDecision::Approved => {
                metadata.admin_verification_status = AdminDataVerificationStatus::Verified;
                metadata.admin_flagged = false;
            }
            AdminDecision::Rejected { .. } => {
                metadata.admin_verification_status = AdminDataVerificationStatus::Rejected;
                metadata.is_active = false;
            }
            AdminDecision::RequiresMoreReview { .. } => {
                metadata.admin_flagged = true;
            }
            AdminDecision::Flagged { .. } => {
                metadata.admin_flagged = true;
            }
        }
        
        metadata.admin_notes = notes.clone();
        metadata.modified_at = Utc::now();
        
        // Add provenance step
        metadata.provenance_chain.push(ProvenanceStep {
            timestamp: Utc::now(),
            action: ProvenanceAction::AdminVerified,
            actor: reviewer_id.to_string(),
            session_id: self.source_manager.current_session.clone(),
            context: Some(serde_json::json!({
                "decision": decision,
                "notes": notes
            })),
            source_reference: Some(absolute_path.to_string_lossy().to_string()),
        });
        
        // Record audit entry
        self.source_manager.add_audit_entry(
            AuditOperation::AdminReview,
            absolute_path.to_string_lossy().to_string(),
            reviewer_id.to_string(),
            Some(serde_json::json!({
                "decision": decision,
                "file_id": file_id
            })),
            OperationResult::Success,
        );
        
        let result = AdminReviewResult {
            file_id: file_id.to_string(),
            decision,
            notes,
            reviewed_at: Utc::now(),
            reviewer_id: reviewer_id.to_string(),
        };
        
        info!("Admin review completed for file: {} by {}", file_id, reviewer_id);
        
        Ok(result)
    }
    
    /// Flag a file for special attention
    pub async fn flag_file(
        &mut self,
        file_id: &str,
        reason: &str,
        reviewer_id: &str,
    ) -> Result<(), SourceManagerError> {
        let metadata = self.source_manager.file_cache.get_mut(file_id)
            .ok_or_else(|| SourceManagerError::FileNotFound(file_id.to_string()))?;
        
        metadata.admin_flagged = true;
        metadata.admin_notes = Some(reason.to_string());
        metadata.modified_at = Utc::now();
        
        // Add provenance step
        metadata.provenance_chain.push(ProvenanceStep {
            timestamp: Utc::now(),
            action: ProvenanceAction::AdminFlagged,
            actor: reviewer_id.to_string(),
            session_id: self.source_manager.current_session.clone(),
            context: Some(serde_json::json!({
                "reason": reason
            })),
            source_reference: Some(metadata.absolute_path.to_string_lossy().to_string()),
        });
        
        info!("File flagged: {} by {} (reason: {})", file_id, reviewer_id, reason);
        
        Ok(())
    }
    
    /// Get comprehensive audit report
    pub fn get_audit_report(&self, days: Option<i64>) -> serde_json::Value {
        let cutoff_date = days.map(|d| Utc::now() - chrono::Duration::days(d));
        
        let filtered_entries: Vec<&AuditEntry> = self.source_manager.audit_trail.iter()
            .filter(|entry| {
                cutoff_date.map_or(true, |cutoff| entry.timestamp >= cutoff)
            })
            .collect();
        
        let mut operation_counts = HashMap::new();
        let mut actor_counts = HashMap::new();
        let mut daily_activity = HashMap::new();
        
        for entry in &filtered_entries {
            *operation_counts.entry(format!("{:?}", entry.operation)).or_insert(0) += 1;
            *actor_counts.entry(entry.actor.clone()).or_insert(0) += 1;
            
            let date = entry.timestamp.date_naive();
            *daily_activity.entry(date).or_insert(0) += 1;
        }
        
        serde_json::json!({
            "summary": {
                "total_entries": filtered_entries.len(),
                "date_range": {
                    "from": cutoff_date,
                    "to": Utc::now()
                }
            },
            "operation_breakdown": operation_counts,
            "actor_activity": actor_counts,
            "daily_activity": daily_activity,
            "recent_entries": filtered_entries.iter().take(50).collect::<Vec<_>>()
        })
    }
    
    /// Get file statistics
    pub fn get_file_statistics(&self) -> serde_json::Value {
        let total_files = self.source_manager.file_cache.len();
        let active_files = self.source_manager.file_cache.values()
            .filter(|f| f.is_active)
            .count();
        let flagged_files = self.source_manager.file_cache.values()
            .filter(|f| f.admin_flagged)
            .count();
        let pending_review = self.source_manager.file_cache.values()
            .filter(|f| f.admin_verification_status == AdminDataVerificationStatus::Pending)
            .count();
        
        let total_size: u64 = self.source_manager.file_cache.values()
            .map(|f| f.file_size)
            .sum();
        
        let mut dno_breakdown = HashMap::new();
        let mut year_breakdown = HashMap::new();
        let mut type_breakdown = HashMap::new();
        
        for metadata in self.source_manager.file_cache.values() {
            *dno_breakdown.entry(metadata.dno_key.clone()).or_insert(0) += 1;
            *year_breakdown.entry(metadata.year).or_insert(0) += 1;
            *type_breakdown.entry(format!("{:?}", metadata.source_type)).or_insert(0) += 1;
        }
        
        serde_json::json!({
            "totals": {
                "total_files": total_files,
                "active_files": active_files,
                "flagged_files": flagged_files,
                "pending_review": pending_review,
                "total_size_bytes": total_size
            },
            "breakdown": {
                "by_dno": dno_breakdown,
                "by_year": year_breakdown,
                "by_type": type_breakdown
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_source_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let source_manager = SourceManager::new(temp_dir.path()).unwrap();
        
        assert_eq!(source_manager.base_dir, temp_dir.path());
        assert!(source_manager.file_cache.is_empty());
        assert!(source_manager.audit_trail.is_empty());
    }
    
    #[tokio::test]
    async fn test_file_storage() {
        let temp_dir = TempDir::new().unwrap();
        let mut source_manager = SourceManager::new(temp_dir.path()).unwrap();
        
        let content = b"test content";
        let metadata = source_manager.store_file(
            "test-dno",
            2024,
            "test.pdf",
            content,
            "https://example.com/test.pdf",
            "https://example.com/test.pdf",
            SourceType::Pdf,
            "application/pdf",
        ).await.unwrap();
        
        assert_eq!(metadata.dno_key, "test-dno");
        assert_eq!(metadata.year, 2024);
        assert_eq!(metadata.file_size, content.len() as u64);
        assert!(metadata.absolute_path.exists());
    }
    
    #[tokio::test]
    async fn test_duplicate_detection() {
        let temp_dir = TempDir::new().unwrap();
        let mut source_manager = SourceManager::new(temp_dir.path()).unwrap();
        
        let content = b"duplicate content";
        
        // Store first file
        let metadata1 = source_manager.store_file(
            "test-dno",
            2024,
            "test1.pdf",
            content,
            "https://example.com/test1.pdf",
            "https://example.com/test1.pdf",
            SourceType::Pdf,
            "application/pdf",
        ).await.unwrap();
        
        // Store duplicate file
        let metadata2 = source_manager.store_file(
            "test-dno",
            2024,
            "test2.pdf",
            content,
            "https://example.com/test2.pdf",
            "https://example.com/test2.pdf",
            SourceType::Pdf,
            "application/pdf",
        ).await.unwrap();
        
        // Second file should be marked as duplicate
        assert!(!metadata2.duplicate_references.is_empty());
        assert_eq!(metadata2.duplicate_references[0], metadata1.id);
    }
    
    #[tokio::test]
    async fn test_integrity_verification() {
        let temp_dir = TempDir::new().unwrap();
        let mut source_manager = SourceManager::new(temp_dir.path()).unwrap();
        
        let content = b"test content for integrity";
        let metadata = source_manager.store_file(
            "test-dno",
            2024,
            "integrity_test.pdf",
            content,
            "https://example.com/test.pdf",
            "https://example.com/test.pdf",
            SourceType::Pdf,
            "application/pdf",
        ).await.unwrap();
        
        // Verify integrity
        let status = source_manager.verify_file_integrity(&metadata.id).await.unwrap();
        assert!(matches!(status, FileIntegrityStatus::Valid));
    }
    
    #[tokio::test]
    async fn test_admin_interface() {
        let temp_dir = TempDir::new().unwrap();
        let source_manager = SourceManager::new(temp_dir.path()).unwrap();
        let mut admin_interface = AdminInterface::new(source_manager);
        
        // This test would require more setup to be meaningful
        let stats = admin_interface.get_file_statistics();
        assert!(stats.get("totals").is_some());
    }
}