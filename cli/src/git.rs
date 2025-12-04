use anyhow::{Context, Result};
use git2::Repository;
use std::path::{Path, PathBuf};

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path).context("Failed to open git repository")?;
        Ok(GitRepo { repo })
    }

    pub fn get_staged_files(&self) -> Result<Vec<PathBuf>> {
        let mut staged_files = Vec::new();
        let index = self.repo.index().context("Failed to open git index")?;
        let workdir = self.repo.workdir().context("Repository has no workdir")?;

        // Get all files from the index (staged files)
        for entry in index.iter() {
            if let Some(path_bytes) = entry.path {
                let path_str = std::str::from_utf8(path_bytes)
                    .context("Invalid path in index")?;
                let full_path = workdir.join(path_str);
                
                // Only add if file exists
                if full_path.exists() && full_path.is_file() {
                    staged_files.push(full_path);
                }
            }
        }

        Ok(staged_files)
    }

    pub fn get_project_name(&self) -> Result<String> {
        let workdir = self.repo.workdir()
            .context("Repository has no workdir")?;
        
        let project_name = workdir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(project_name)
    }
}
