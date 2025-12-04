use crate::entropy::calculate_shannon_entropy;
use crate::ScanResult;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub struct Scanner {
    patterns: Vec<SecretPattern>,
}

struct SecretPattern {
    name: String,
    regex: Regex,
    min_entropy: f64,
}

impl Scanner {
    pub fn new() -> Self {
        let patterns = vec![
            SecretPattern {
                name: "AWS Access Key ID".to_string(),
                regex: Regex::new(r#"AKIA[0-9A-Z]{16}"#).unwrap(),
                min_entropy: 0.0,
            },
            SecretPattern {
                name: "AWS Secret Access Key".to_string(),
                regex: Regex::new(r#"(?i)(aws_secret_access_key|aws_secret_key)\s*[=:]\s*['"]?([A-Za-z0-9/+=]{40})['"]?"#).unwrap(),
                min_entropy: 4.0,
            },
            SecretPattern {
                name: "GitHub Token".to_string(),
                regex: Regex::new(r#"(?i)(ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{36,255}"#).unwrap(),
                min_entropy: 0.0,
            },
            SecretPattern {
                name: "Slack Token".to_string(),
                regex: Regex::new(r#"(?i)(xox[baprs]-[0-9a-zA-Z-]{10,})"#).unwrap(),
                min_entropy: 0.0,
            },
            SecretPattern {
                name: "Generic API Key".to_string(),
                regex: Regex::new(r#"(?i)(api[_-]?key|apikey)\s*[=:]\s*['"]?([A-Za-z0-9_\-]{20,})['"]?"#).unwrap(),
                min_entropy: 3.5,
            },
            SecretPattern {
                name: "JWT Token".to_string(),
                regex: Regex::new(r#"eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}"#).unwrap(),
                min_entropy: 0.0,
            },
            SecretPattern {
                name: "Private Key".to_string(),
                regex: Regex::new(r#"-----BEGIN (RSA |DSA |EC )?PRIVATE KEY-----"#).unwrap(),
                min_entropy: 0.0,
            },
            SecretPattern {
                name: "Database Connection String".to_string(),
                regex: Regex::new(r#"(?i)(mongodb|postgres|mysql|redis)://[^\s'"]+"#).unwrap(),
                min_entropy: 3.0,
            },
            SecretPattern {
                name: "Password in Config".to_string(),
                regex: Regex::new(r#"(?i)(password|passwd|pwd)\s*[=:]\s*['"]?([^\s'"]{8,})['"]?"#).unwrap(),
                min_entropy: 3.5,
            },
        ];

        Scanner { patterns }
    }

    pub fn scan_file(&self, content: &str, file_path: &Path) -> Vec<ScanResult> {
        let mut findings = Vec::new();
        let file_name = file_path.to_string_lossy().to_string();

        for pattern in &self.patterns {
            for (line_num, line) in content.lines().enumerate() {
                if let Some(captures) = pattern.regex.captures(line) {
                    // Extract the actual secret value for entropy analysis
                    let secret_value = captures.get(2)
                        .map(|m| m.as_str())
                        .or_else(|| captures.get(0).map(|m| m.as_str()))
                        .unwrap_or("");

                    // Skip empty matches or too short values
                    if secret_value.is_empty() || secret_value.len() < 8 {
                        continue;
                    }

                    // Calculate entropy if required
                    let entropy = calculate_shannon_entropy(secret_value);
                    
                    if entropy < pattern.min_entropy {
                        continue;
                    }

                    // Calculate confidence score (0.0 to 1.0)
                    let confidence = self.calculate_confidence(&pattern.name, entropy, secret_value.len());

                    // Extract preview (truncate if too long)
                    let preview = if line.len() > 80 {
                        format!("{}...", &line[..77])
                    } else {
                        line.to_string()
                    };

                    findings.push(ScanResult {
                        file: file_name.clone(),
                        line: line_num + 1,
                        secret_type: pattern.name.clone(),
                        confidence,
                        preview,
                    });
                }
            }
        }

        findings
    }

    fn calculate_confidence(&self, secret_type: &str, entropy: f64, length: usize) -> f64 {
        let mut confidence = 0.5; // Base confidence

        // Adjust based on entropy
        if entropy > 4.0 {
            confidence += 0.3;
        } else if entropy > 3.0 {
            confidence += 0.2;
        }

        // Adjust based on length
        if length > 32 {
            confidence += 0.1;
        } else if length > 16 {
            confidence += 0.05;
        }

        // Type-specific adjustments
        if secret_type.contains("AWS") || secret_type.contains("GitHub") || secret_type.contains("Slack") {
            confidence += 0.1;
        }

        confidence.min(1.0)
    }
}

