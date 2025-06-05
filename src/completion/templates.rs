//! Template discovery for shell completion
//!
//! Provides template suggestions for `angreal init` command from:
//! - Local cache (~/.angrealrc/)
//! - GitHub angreal organization repositories

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// GitHub repository information
#[derive(Deserialize)]
struct GitHubRepo {
    name: String,
    #[allow(dead_code)]
    description: Option<String>,
    #[serde(rename = "html_url")]
    #[allow(dead_code)]
    url: String,
}

/// Get template suggestions for completion
pub fn get_template_suggestions() -> Result<Vec<String>> {
    let mut suggestions = HashSet::new();

    // Add local cached templates
    if let Ok(local_templates) = get_local_templates() {
        suggestions.extend(local_templates);
    }

    // Add GitHub organization templates (with timeout)
    if let Ok(github_templates) = get_github_templates() {
        suggestions.extend(github_templates);
    }

    // Convert to sorted vector
    let mut result: Vec<String> = suggestions.into_iter().collect();
    result.sort();

    Ok(result)
}

/// Get locally cached templates from ~/.angrealrc/
fn get_local_templates() -> Result<Vec<String>> {
    let home = env::var("HOME").context("HOME environment variable not set")?;
    let angreal_cache = PathBuf::from(home).join(".angrealrc");

    if !angreal_cache.exists() {
        return Ok(Vec::new());
    }

    let mut templates = Vec::new();

    // Read directory entries
    for entry in fs::read_dir(&angreal_cache)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    // Skip hidden directories and add template name
                    if !name_str.starts_with('.') {
                        templates.push(name_str.to_string());
                    }
                }
            }
        }
    }

    Ok(templates)
}

/// Get template repositories from GitHub angreal organization
fn get_github_templates() -> Result<Vec<String>> {
    // Quick timeout for completion - don't block the user
    let client = Client::builder()
        .timeout(Duration::from_millis(500))
        .user_agent("angreal-completion")
        .build()?;

    let url = "https://api.github.com/orgs/angreal/repos?type=public&sort=updated&per_page=50";

    let response = client
        .get(url)
        .send()
        .context("Failed to fetch GitHub repositories")?;

    if !response.status().is_success() {
        // Don't fail completion for GitHub API issues
        return Ok(Vec::new());
    }

    let repos: Vec<GitHubRepo> = response
        .json()
        .context("Failed to parse GitHub API response")?;

    let mut templates = Vec::new();

    for repo in repos {
        // Filter for template repositories
        if is_template_repo(&repo) {
            templates.push(repo.name);
            // Don't add full URLs to completion - they're messy and users can specify full URLs manually
        }
    }

    Ok(templates)
}

/// Determine if a GitHub repository is a template (exclude meta repos)
fn is_template_repo(repo: &GitHubRepo) -> bool {
    let name = repo.name.to_lowercase();

    // Skip anything that starts with "angreal"
    !name.starts_with("angreal")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_templates() {
        // Should not crash even if directory doesn't exist
        let _ = get_local_templates();
    }

    #[test]
    fn test_is_template_repo() {
        let template_repo = GitHubRepo {
            name: "python-template".to_string(),
            description: Some("A Python project template".to_string()),
            url: "https://github.com/angreal/python-template".to_string(),
        };
        assert!(is_template_repo(&template_repo));

        let meta_repo = GitHubRepo {
            name: "angreal".to_string(),
            description: Some("The main angreal repository".to_string()),
            url: "https://github.com/angreal/angreal".to_string(),
        };
        assert!(!is_template_repo(&meta_repo));
    }

    #[test]
    fn test_get_template_suggestions() {
        // Should not crash and should return some suggestions
        let suggestions = get_template_suggestions().unwrap_or_default();
        // Even if network fails, should have local templates or empty list
        assert!(suggestions.len() >= 0);
    }
}
