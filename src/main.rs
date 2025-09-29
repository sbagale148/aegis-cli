use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

mod scanner;
mod hook;

use crate::scanner::scan_path;
use crate::hook::install_hook;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("aegis")
        .version("0.1.0")
        .author("Your Name")
        .about("Scans code for secrets before commit")
        .subcommand_required(true)
        .subcommand(
            Command::new("scan")
                .about("Scans a file or directory for secrets")
                .arg(
                    Arg::new("path")
                        .default_value(".")
                        .help("The path to scan (file or directory)"),
                )
                .arg(
                    Arg::new("file")
                        .long("file")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .help("Treat the path as a single file"),
                ),
        )
        .subcommand(
            Command::new("install-hook")
                .about("Installs Aegis as a pre-commit hook in the current Git repository"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("scan", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").unwrap();
            let is_file = sub_matches.get_flag("file");
            let path_buf = PathBuf::from(path);

            if let Err(e) = scan_path(&path_buf, is_file) {
                eprintln!("Scan failed: {}", e);
                std::process::exit(1);
            }
            println!("✅ Scan completed. No secrets found.");
            Ok(())
        }
        Some(("install-hook", _)) => {
            install_hook()?;
            Ok(())
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}