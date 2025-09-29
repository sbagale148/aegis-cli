use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use anyhow::{Context, Result};

pub fn install_hook() -> Result<()> {
    // Find the .git directory
    let git_dir = find_git_dir().context("Could not find .git directory. Are you in a Git repository?")?;
    let hooks_dir = git_dir.join("hooks");
    let pre_commit_hook = hooks_dir.join("pre-commit");

    // Create the hooks directory if it doesn't exist
    if !hooks_dir.exists() {
        fs::create_dir(&hooks_dir)?;
    }

    // Get the absolute path to the current aegis binary
    let current_exe = env::current_exe().context("Failed to get the path of the current executable")?;
    let current_exe_str = current_exe.to_str().context("Executable path is not valid UTF-8")?;

    // The content of the pre-commit hook script
    let script_content = format!(
        "#!/bin/sh\n{} scan\n",
        current_exe_str
    );

    // Write the script
    fs::write(&pre_commit_hook, script_content)?;

    // Make the script executable (Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&pre_commit_hook)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&pre_commit_hook, perms)?;
    }

    println!("✅ Pre-commit hook installed successfully at: {}", pre_commit_hook.display());
    Ok(())
}

fn find_git_dir() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    let mut dir = current_dir.as_path();

    loop {
        let git_dir = dir.join(".git");
        if git_dir.exists() {
            return Some(git_dir);
        }
        dir = dir.parent()?;
    }
}