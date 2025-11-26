mod scanner;
mod git;
mod api;
mod config;

use clap::{Parser, Subcommand};
use colored::*;
use scanner::{SecretScanner, SecretDetection};
use git::GitHookManager;
use std::path::Path;

#[derive(Parser)]
#[command(name = "aegis")]
#[command(about = "Pre-commit secret scanning system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan files or directories for secrets
    Scan {
        /// Path to scan (file or directory)
        path: Option<String>,
        
        /// Scan specific file
        #[arg(short, long)]
        file: Option<String>,
        
        /// Scan staged git files
        #[arg(long)]
        staged: bool,
        
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    
    /// Install as pre-commit hook
    InstallHook,
    
    /// Show current configuration
    Config,
    
    /// Display system health
    Status,
    
    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, file, staged, format } => {
            let scanner = SecretScanner::new();
            let mut detections = Vec::new();

            if staged {
                println!("🔍 Scanning staged git files...");
                detections = GitHookManager::scan_staged_files()?;
            } else if let Some(file_path) = file {
                println!("🔍 Scanning file: {}", file_path);
                detections = scanner.scan_file(Path::new(&file_path));
            } else {
                let scan_path = path.as_deref().unwrap_or(".");
                println!("🔍 Scanning directory: {}", scan_path);
                detections = scan_directory(&scanner, scan_path);
            }

            handle_scan_results(detections, &format).await?;
        }
        
        Commands::InstallHook => {
            GitHookManager::install_pre_commit_hook()?;
        }
        
        Commands::Config => {
            show_configuration();
        }
        
        Commands::Status => {
            check_system_status().await?;
        }
        
        Commands::Version => {
            println!("Aegis CLI v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

fn scan_directory(scanner: &SecretScanner, path: &str) -> Vec<SecretDetection> {
    let mut all_detections = Vec::new();
    
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let detections = scanner.scan_file(entry.path());
        all_detections.extend(detections);
    }
    
    all_detections
}

async fn handle_scan_results(
    detections: Vec<SecretDetection>, 
    format: &str
) -> Result<(), Box<dyn std::error::Error>> {
    if detections.is_empty() {
        println!("✅ {}", "No secrets detected!".green());
        return Ok(());
    }

    match format {
        "json" => {
            let json_output = serde_json::to_string_pretty(&detections)?;
            println!("{}", json_output);
        }
        _ => {
            println!("❌ {}", "Secrets detected!".red().bold());
            for detection in &detections {
                println!("\n{}", "=".repeat(50).yellow());
                println!("{}: {}", "File".bold(), detection.file_path);
                println!("{}: {}", "Line".bold(), detection.line_number);
                println!("{}: {}", "Type".bold(), detection.secret_type);
                println!("{}: {:.2}%", "Confidence".bold(), detection.confidence_score * 100.0);
                println!("{}: {}", "Content".bold(), detection.content);
            }
            
            // Report to dashboard if configured
            // let api_client = create_api_client(); // Implement this
            // for detection in detections {
            //     api_client.report_scan_event(&detection, true).await?;
            // }
        }
    }

    if !detections.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}

fn show_configuration() {
    println!("{}", "Aegis Configuration".bold());
    println!("{}: v{}", "Version", env!("CARGO_PKG_VERSION"));
    println!("{}: {}", "Supported secret types", "AWS Keys, GitHub Tokens, API Keys, JWT, etc.");
    // Add more configuration details
}

async fn check_system_status() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "System Status".bold());
    println!("✅ CLI: Operational");
    println!("✅ Scanner: Ready");
    // Add more status checks
    Ok(())
}