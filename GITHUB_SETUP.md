# GitHub Setup Guide

## Step 1: Create a GitHub Repository

1. Go to [GitHub.com](https://github.com) and sign in
2. Click the "+" icon in the top right corner
3. Select "New repository"
4. Fill in the repository details:
   - **Repository name**: `aegis` (or your preferred name)
   - **Description**: "Aegis - Secret scanning system for pre-commit hooks"
   - **Visibility**: Choose Public or Private
   - **DO NOT** initialize with README, .gitignore, or license (we already have these)
5. Click "Create repository"

## Step 2: Add Remote and Push

After creating the repository, GitHub will show you commands. Use these commands in your terminal:

```bash
# Add the remote (replace YOUR_USERNAME with your GitHub username)
git remote add origin https://github.com/YOUR_USERNAME/aegis.git

# Push to GitHub
git branch -M main
git push -u origin main
```

Or if you prefer SSH:

```bash
git remote add origin git@github.com:YOUR_USERNAME/aegis.git
git branch -M main
git push -u origin main
```

## Step 3: Verify

After pushing, refresh your GitHub repository page. You should see all your files!

## Troubleshooting

If you get authentication errors:
- For HTTPS: You may need to use a Personal Access Token instead of password
- For SSH: Make sure your SSH key is added to GitHub

If the branch name is different:
- The commands above rename the branch to `main` (GitHub's default)
- If you prefer to keep `master`, just use: `git push -u origin master`

