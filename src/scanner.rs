use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SecretDetection {
    pub file_path: String,
    pub line_number: usize,
    pub secret_type: String,
    pub content: String,
    pub confidence_score: f64,
}

pub struct SecretScanner {
    patterns: HashMap<String, Regex>,
}

impl SecretScanner {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // AWS Access Key
        patterns.insert(
            "aws_access_key".to_string(),
            Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
        );
        
        // GitHub Personal Access Token
        patterns.insert(
            "github_token".to_string(),
            Regex::new(r"ghp_[a-zA-Z0-9]{36}").unwrap(),
        );
        
        // Generic API Key
        patterns.insert(
            "api_key".to_string(),
            Regex::new(r"(?i)(api[_-]?key|secret)[=\s:][\"\']?([a-zA-Z0-9]{32,})").unwrap(),
        );
        
        // JWT Token
        patterns.insert(
            "jwt_token".to_string(),
            Regex::new(r"eyJ[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+").unwrap(),
        );
        
        // Add more patterns as needed...
        
        SecretScanner { patterns }
    }

    pub fn scan_file(&self, file_path: &Path) -> Vec<SecretDetection> {
        let mut detections = Vec::new();
        
        if let Ok(content) = std::fs::read_to_string(file_path) {
            for (line_number, line) in content.lines().enumerate() {
                for (secret_type, pattern) in &self.patterns {
                    if let Some(captures) = pattern.captures(line) {
                        let matched_content = captures.get(0).map(|m| m.as_str()).unwrap_or("");
                        
                        // Calculate confidence score (regex match + entropy)
                        let confidence = self.calculate_confidence(matched_content, secret_type);
                        
                        if confidence > 0.7 { // Threshold for detection
                            detections.push(SecretDetection {
                                file_path: file_path.to_string_lossy().to_string(),
                                line_number: line_number + 1,
                                secret_type: secret_type.clone(),
                                content: matched_content.to_string(),
                                confidence_score: confidence,
                            });
                        }
                    }
                }
            }
        }
        
        detections
    }

    fn calculate_confidence(&self, content: &str, secret_type: &str) -> f64 {
        let mut confidence = 0.5; // Base confidence
        
        // Add pattern-specific confidence
        match secret_type {
            "aws_access_key" => confidence += 0.4,
            "github_token" => confidence += 0.4,
            "api_key" => confidence += 0.3,
            "jwt_token" => confidence += 0.2,
            _ => confidence += 0.1,
        }
        
        // Add entropy-based confidence
        let entropy = self.calculate_entropy(content);
        if entropy > 4.0 {
            confidence += 0.2;
        }
        
        confidence.min(1.0) // Cap at 1.0
    }

    fn calculate_entropy(&self, text: &str) -> f64 {
        let mut frequency_map = HashMap::new();
        let total_chars = text.len() as f64;
        
        for ch in text.chars() {
            *frequency_map.entry(ch).or_insert(0) += 1;
        }
        
        let mut entropy = 0.0;
        for &count in frequency_map.values() {
            let probability = count as f64 / total_chars;
            entropy -= probability * probability.log2();
        }
        
        entropy
    }
}