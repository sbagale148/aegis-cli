use std::fs;
use std::path::Path;
use crate::scanner::{SecretScanner, SecretDetection};

pub struct GitHookManager;

impl GitHookManager {
    pub fn install_pre_commit_hook() -> Result<(), Box<dyn std::error::Error>> {
        let hook_content = r#"#!/bin/sh
# Aegis Pre-commit Hook
echo "🔍 Scanning staged files with Aegis..."
aegis scan --staged

if [ $? -ne 0 ]; then
    echo "❌ Commit blocked: Secrets detected"
    exit 1
else
    echo "✅ No secrets detected"
    exit 0
fi
"#;

        let hook_path = Path::new(".git/hooks/pre-commit");
        
        // Create hooks directory if it doesn't exist
        if let Some(parent) = hook_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(hook_path, hook_content)?;
        
        // Make the hook executable on Unix-like systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(hook_path, perms)?;
        }
        
        println!("✅ Pre-commit hook installed successfully");
        Ok(())
    }

    pub fn scan_staged_files() -> Result<Vec<SecretDetection>, Box<dyn std::error::Error>> {
        let output = std::process::Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .output()?;
            
        let staged_files = String::from_utf8(output.stdout)?;
        let scanner = SecretScanner::new();
        let mut all_detections = Vec::new();
        
        for file_path in staged_files.lines() {
            if !file_path.trim().is_empty() {
                let detections = scanner.scan_file(Path::new(file_path));
                all_detections.extend(detections);
            }
        }
        
        Ok(all_detections)
    }
}