use scraper::{Html, Selector};
use serde_json::{Value, Map};
use shared::AppError;

pub struct DnoDataExtractor;

impl DnoDataExtractor {
    pub fn new() -> Self {
        Self
    }
    
    pub fn extract_dno_data(&self, document: &Html, url: &str) -> Result<Value, AppError> {
        let mut data = Map::new();
        
        // Basic page info
        data.insert("source_url".to_string(), Value::String(url.to_string()));
        data.insert("extracted_at".to_string(), Value::String(chrono::Utc::now().to_rfc3339()));
        
        // Extract DNO name (usually in title or main heading)
        if let Some(name) = self.extract_dno_name(document) {
            data.insert("name".to_string(), Value::String(name));
        }
        
        // Extract contact information
        if let Some(contact) = self.extract_contact_info(document) {
            data.insert("contact".to_string(), contact);
        }
        
        // Extract service areas/regions
        if let Some(regions) = self.extract_regions(document) {
            data.insert("regions".to_string(), Value::Array(regions));
        }
        
        // Extract services offered
        if let Some(services) = self.extract_services(document) {
            data.insert("services".to_string(), Value::Array(services));
        }
        
        // Extract network data if available
        if let Some(network_data) = self.extract_network_data(document) {
            data.insert("network_data".to_string(), network_data);
        }
        
        Ok(Value::Object(data))
    }
    
    fn extract_dno_name(&self, document: &Html) -> Option<String> {
        // Try title first
        if let Some(title) = document.select(&Selector::parse("title").unwrap()).next() {
            let title_text = title.text().collect::<String>().trim().to_string();
            if !title_text.is_empty() && title_text.len() < 100 {
                return Some(title_text);
            }
        }
        
        // Try main heading
        if let Some(h1) = document.select(&Selector::parse("h1").unwrap()).next() {
            let h1_text = h1.text().collect::<String>().trim().to_string();
            if !h1_text.is_empty() && h1_text.len() < 100 {
                return Some(h1_text);
            }
        }
        
        None
    }
    
    fn extract_contact_info(&self, document: &Html) -> Option<Value> {
        let mut contact = Map::new();
        
        // Extract email addresses
        let text = document.root_element().text().collect::<String>();
        if let Ok(email_regex) = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b") {
            let emails: Vec<String> = email_regex
                .find_iter(&text)
                .map(|m| m.as_str().to_string())
                .collect();
            if !emails.is_empty() {
                contact.insert("emails".to_string(), Value::Array(
                    emails.into_iter().map(Value::String).collect()
                ));
            }
        }
        
        // Extract phone numbers
        if let Ok(phone_regex) = regex::Regex::new(r"\+49[\s\-]?[0-9\s\-\(\)]{8,}|0[0-9\s\-\(\)]{8,}") {
            let phones: Vec<String> = phone_regex
                .find_iter(&text)
                .map(|m| m.as_str().trim().to_string())
                .filter(|p| p.len() > 8)
                .collect();
            if !phones.is_empty() {
                contact.insert("phones".to_string(), Value::Array(
                    phones.into_iter().map(Value::String).collect()
                ));
            }
        }
        
        if contact.is_empty() {
            None
        } else {
            Some(Value::Object(contact))
        }
    }
    
    fn extract_regions(&self, document: &Html) -> Option<Vec<Value>> {
        let text = document.root_element().text().collect::<String>().to_lowercase();
        
        // Common German states and regions
        let regions = vec![
            "baden-württemberg", "bayern", "berlin", "brandenburg", "bremen",
            "hamburg", "hessen", "mecklenburg-vorpommern", "niedersachsen",
            "nordrhein-westfalen", "rheinland-pfalz", "saarland", "sachsen",
            "sachsen-anhalt", "schleswig-holstein", "thüringen"
        ];
        
        let found_regions: Vec<Value> = regions
            .iter()
            .filter(|region| text.contains(region))
            .map(|region| Value::String(region.to_string()))
            .collect();
            
        if found_regions.is_empty() {
            None
        } else {
            Some(found_regions)
        }
    }
    
    fn extract_services(&self, document: &Html) -> Option<Vec<Value>> {
        let text = document.root_element().text().collect::<String>().to_lowercase();
        
        // Common DNO services
        let services = vec![
            "netzanschluss", "störungsdienst", "zählerwesen", "netzbetrieb",
            "netzplanung", "messstellenbetrieb", "grundversorgung", "bilanzkreis",
            "einspeisemanagement", "redispatch", "blindleistung", "systemdienstleistung"
        ];
        
        let found_services: Vec<Value> = services
            .iter()
            .filter(|service| text.contains(service))
            .map(|service| Value::String(service.to_string()))
            .collect();
            
        if found_services.is_empty() {
            None
        } else {
            Some(found_services)
        }
    }
    
    fn extract_network_data(&self, document: &Html) -> Option<Value> {
        let mut network_data = Map::new();
        let text = document.root_element().text().collect::<String>();
        
        // Look for voltage levels
        if let Ok(voltage_regex) = regex::Regex::new(r"(\d+)\s?kV") {
            let voltages: Vec<String> = voltage_regex
                .find_iter(&text)
                .map(|m| m.as_str().to_string())
                .collect();
            if !voltages.is_empty() {
                network_data.insert("voltage_levels".to_string(), Value::Array(
                    voltages.into_iter().map(Value::String).collect()
                ));
            }
        }
        
        // Look for network length information
        if let Ok(length_regex) = regex::Regex::new(r"(\d+(?:,\d+)?)\s?km") {
            let lengths: Vec<String> = length_regex
                .find_iter(&text)
                .map(|m| m.as_str().to_string())
                .collect();
            if !lengths.is_empty() {
                network_data.insert("network_lengths".to_string(), Value::Array(
                    lengths.into_iter().map(Value::String).collect()
                ));
            }
        }
        
        if network_data.is_empty() {
            None
        } else {
            Some(Value::Object(network_data))
        }
    }
}

impl Default for DnoDataExtractor {
    fn default() -> Self {
        Self::new()
    }
}