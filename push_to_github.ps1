# PowerShell script to push Aegis to GitHub
# Run this AFTER creating a GitHub repository

param(
    [Parameter(Mandatory=$true)]
    [string]$GitHubUsername,
    
    [Parameter(Mandatory=$false)]
    [string]$RepoName = "aegis"
)

Write-Host "Setting up GitHub remote..." -ForegroundColor Green

# Add remote
$remoteUrl = "https://github.com/$GitHubUsername/$RepoName.git"
git remote add origin $remoteUrl

if ($LASTEXITCODE -ne 0) {
    Write-Host "Remote might already exist. Checking..." -ForegroundColor Yellow
    git remote set-url origin $remoteUrl
}

# Rename branch to main (GitHub standard)
git branch -M main

Write-Host "Pushing to GitHub..." -ForegroundColor Green
git push -u origin main

if ($LASTEXITCODE -eq 0) {
    Write-Host "Successfully pushed to GitHub!" -ForegroundColor Green
    Write-Host "Repository URL: https://github.com/$GitHubUsername/$RepoName" -ForegroundColor Cyan
} else {
    Write-Host "Push failed. You may need to authenticate." -ForegroundColor Red
    Write-Host "If using HTTPS, you'll need a Personal Access Token." -ForegroundColor Yellow
}

