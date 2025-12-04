use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Instant;
use chrono::Utc;

mod scanner;
mod entropy;
mod git;
mod api;

use scanner::Scanner;
use git::GitRepo;
use api::ApiClient;

#[derive(Parser)]
#[command(name = "aegis")]
#[command(about = "Aegis - Secret scanning for pre-commit hooks")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan staged files for secrets
    Scan {
        /// API endpoint URL for reporting (optional)
        #[arg(long, env = "AEGIS_API_URL")]
        api_url: Option<String>,
        /// Skip API reporting
        #[arg(long)]
        no_report: bool,
    },
    /// Install pre-commit hook
    Install,
    /// Uninstall pre-commit hook
    Uninstall,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScanResult {
    file: String,
    line: usize,
    secret_type: String,
    confidence: f64,
    preview: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanEvent {
    pub timestamp: String,
    pub project_name: String,
    pub file_path: String,
    pub secret_type: String,
    pub confidence: f64,
    pub line_number: usize,
    pub preview: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { api_url, no_report } => {
            let start = Instant::now();
            
            println!("🔍 Aegis: Scanning staged files for secrets...\n");
            
            let repo = GitRepo::open(".")?;
            let staged_files = repo.get_staged_files()?;
            
            if staged_files.is_empty() {
                println!("✓ No staged files to scan.");
                return Ok(());
            }

            let scanner = Scanner::new();
            let mut findings: Vec<ScanResult> = Vec::new();

            for file_path in &staged_files {
                if let Ok(content) = fs::read_to_string(file_path) {
                    let file_findings = scanner.scan_file(&content, file_path);
                    findings.extend(file_findings);
                }
            }

            let duration = start.elapsed();

            if findings.is_empty() {
                println!("✓ Scan completed in {:.2}ms - No secrets detected!", duration.as_secs_f64() * 1000.0);
                return Ok(());
            }

            println!("❌ Found {} potential secret(s):\n", findings.len());
            
            for finding in &findings {
                println!("  File: {}", finding.file);
                println!("  Line: {}", finding.line);
                println!("  Type: {}", finding.secret_type);
                println!("  Confidence: {:.1}%", finding.confidence * 100.0);
                println!("  Preview: {}", finding.preview);
                println!();
            }

            // Report to API if configured
            if !no_report {
                if let Some(url) = api_url {
                    let api_client = ApiClient::new(&url);
                    let project_name = repo.get_project_name().unwrap_or_else(|_| "unknown".to_string());
                    
                    // Report all events asynchronously (fire and forget)
                    let url_clone = url.clone();
                    for finding in &findings {
                        let event = ScanEvent {
                            timestamp: Utc::now().to_rfc3339(),
                            project_name: project_name.clone(),
                            file_path: finding.file.clone(),
                            secret_type: finding.secret_type.clone(),
                            confidence: finding.confidence,
                            line_number: finding.line,
                            preview: finding.preview.clone(),
                        };
                        
                        // Async, non-blocking report - spawn and forget
                        let client = ApiClient::new(&url_clone);
                        let event_clone = event.clone();
                        tokio::spawn(async move {
                            if let Err(e) = client.report_event(&event_clone).await {
                                eprintln!("Warning: Failed to report event: {}", e);
                            }
                        });
                    }
                }
            }

            println!("⚠️  Commit blocked. Please remove secrets before committing.");
            std::process::exit(1);
        }
        Commands::Install => {
            install_hook()?;
        }
        Commands::Uninstall => {
            uninstall_hook()?;
        }
    }

    Ok(())
}

fn install_hook() -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        anyhow::bail!("Not a git repository. Please run this command from the root of a git repo.");
    }

    let hooks_dir = git_dir.join("hooks");
    fs::create_dir_all(&hooks_dir)?;

    let hook_path = hooks_dir.join("pre-commit");
    
    // Get the absolute path to the aegis binary
    let current_exe = std::env::current_exe()
        .context("Failed to get current executable path")?;
    let exe_path = current_exe.to_string_lossy().replace("\\", "/");

    let hook_content = format!(
        r#"#!/bin/sh
# Aegis pre-commit hook
{exe_path} scan
"#,
        exe_path = exe_path
    );

    fs::write(&hook_path, hook_content)?;
    
    // Make it executable on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms)?;
    }

    println!("✓ Pre-commit hook installed at: {}", hook_path.display());
    Ok(())
}

fn uninstall_hook() -> Result<()> {
    let hook_path = Path::new(".git/hooks/pre-commit");
    
    if hook_path.exists() {
        fs::remove_file(hook_path)?;
        println!("✓ Pre-commit hook removed.");
    } else {
        println!("No pre-commit hook found.");
    }
    
    Ok(())
}
