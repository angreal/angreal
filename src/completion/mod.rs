//! Shell completion support for Angreal
//!
//! Provides auto-installing shell completion for bash and zsh with:
//! - Real-time task discovery
//! - Template suggestions from GitHub
//! - Automatic setup on first run

use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub mod bash;
pub mod templates;
pub mod zsh;

/// Supported shells for completion
#[derive(Debug, Clone, PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
    Unknown(String),
}

impl Shell {
    /// Detect the current shell from environment
    pub fn detect() -> Self {
        // Check SHELL environment variable
        if let Ok(shell_path) = env::var("SHELL") {
            if shell_path.contains("bash") {
                return Shell::Bash;
            } else if shell_path.contains("zsh") {
                return Shell::Zsh;
            }
        }

        // Fallback: check parent process name
        if let Ok(output) = Command::new("ps")
            .args(["-p", &std::process::id().to_string(), "-o", "comm="])
            .output()
        {
            let comm = String::from_utf8_lossy(&output.stdout);
            if comm.contains("bash") {
                return Shell::Bash;
            } else if comm.contains("zsh") {
                return Shell::Zsh;
            }
        }

        Shell::Unknown("unknown".to_string())
    }

    /// Get the name of this shell
    pub fn name(&self) -> &str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Unknown(name) => name,
        }
    }
}

/// Configuration for shell completion
pub struct CompletionConfig {
    pub shell: Shell,
    pub install_path: PathBuf,
    pub completion_script: String,
}

impl CompletionConfig {
    /// Create completion config for detected shell
    pub fn for_current_shell() -> Result<Self> {
        let shell = Shell::detect();
        let home = env::var("HOME").context("HOME environment variable not set")?;

        let (install_path, completion_script) = match shell {
            Shell::Bash => {
                let path = PathBuf::from(&home)
                    .join(".bash_completion.d")
                    .join("angreal");
                let script = bash::generate_completion_script();
                (path, script)
            }
            Shell::Zsh => {
                // Try to find zsh completion directory
                let zsh_dir = find_zsh_completion_dir(&home)?;
                let path = zsh_dir.join("_angreal");
                let script = zsh::generate_completion_script();
                (path, script)
            }
            Shell::Unknown(_) => {
                anyhow::bail!("Unsupported shell for completion: {}", shell.name());
            }
        };

        Ok(CompletionConfig {
            shell,
            install_path,
            completion_script,
        })
    }

    /// Check if completion is already installed
    pub fn is_installed(&self) -> bool {
        self.install_path.exists()
    }

    /// Install completion script
    pub fn install(&self) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.install_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Write completion script
        fs::write(&self.install_path, &self.completion_script).with_context(|| {
            format!(
                "Failed to write completion script to: {}",
                self.install_path.display()
            )
        })?;

        // Add source line to shell rc file if needed
        self.ensure_sourced()?;

        Ok(())
    }

    /// Ensure completion script is sourced in shell rc file
    fn ensure_sourced(&self) -> Result<()> {
        let home = env::var("HOME").context("HOME environment variable not set")?;

        let rc_file = match self.shell {
            Shell::Bash => {
                // Try .bashrc first, then .bash_profile
                let bashrc = PathBuf::from(&home).join(".bashrc");
                if bashrc.exists() {
                    bashrc
                } else {
                    PathBuf::from(&home).join(".bash_profile")
                }
            }
            Shell::Zsh => PathBuf::from(&home).join(".zshrc"),
            Shell::Unknown(_) => return Ok(()), // Skip for unknown shells
        };

        // Check if sourcing is already present
        if rc_file.exists() {
            let content = fs::read_to_string(&rc_file)?;

            match self.shell {
                Shell::Bash => {
                    let source_line = format!("source {}", self.install_path.display());
                    if !content.contains(&source_line) {
                        // Add source line for bash
                        let mut new_content = content;
                        new_content.push('\n');
                        new_content
                            .push_str(&format!("# Angreal shell completion\n{}\n", source_line));
                        fs::write(&rc_file, new_content)?;
                    }
                }
                Shell::Zsh => {
                    // Check if we need to add custom completion directory to fpath
                    let completion_dir = self.install_path.parent().unwrap();
                    let is_custom_dir = !completion_dir.to_str().unwrap().contains("/usr/")
                        && !completion_dir.to_str().unwrap().contains("/etc/");

                    if is_custom_dir {
                        let fpath_line = format!("fpath=({} $fpath)", completion_dir.display());
                        if !content.contains(&fpath_line) {
                            let mut new_content = content;
                            new_content.push('\n');
                            new_content
                                .push_str(&format!("# Angreal shell completion\n{}\n", fpath_line));

                            // Also ensure compinit is called
                            if !new_content.contains("autoload -U compinit")
                                || !new_content.contains("compinit")
                            {
                                new_content.push_str("autoload -U compinit && compinit\n");
                            }

                            fs::write(&rc_file, new_content)?;
                        }
                    }
                }
                Shell::Unknown(_) => return Ok(()),
            }
        }

        Ok(())
    }
}

/// Find zsh completion directory
fn find_zsh_completion_dir(home: &str) -> Result<PathBuf> {
    // Common zsh completion directories in order of preference
    let candidates = vec![
        PathBuf::from(home).join(".zsh").join("completions"),
        PathBuf::from(home).join(".oh-my-zsh").join("completions"),
        PathBuf::from("/usr/local/share/zsh/site-functions"),
        PathBuf::from("/usr/share/zsh/site-functions"),
        // Fallback: create in home directory
        PathBuf::from(home).join(".zsh_completions"),
    ];

    for candidate in &candidates {
        if candidate.exists() && candidate.is_dir() {
            // Check if we can write to this directory
            let test_file = candidate.join(".angreal_test_write");
            if fs::write(&test_file, "test").is_ok() {
                // Clean up test file
                let _ = fs::remove_file(&test_file);
                return Ok(candidate.clone());
            }
        }
    }

    // Create the fallback directory
    let fallback = &candidates[candidates.len() - 1];
    fs::create_dir_all(fallback).with_context(|| {
        format!(
            "Failed to create zsh completion directory: {}",
            fallback.display()
        )
    })?;

    Ok(fallback.clone())
}

/// Check if completion should be auto-installed
pub fn should_auto_install() -> bool {
    // Check if user has explicitly disabled auto-install
    if env::var("ANGREAL_NO_AUTO_COMPLETION").is_ok() {
        return false;
    }

    // Check if completion is already installed
    if let Ok(config) = CompletionConfig::for_current_shell() {
        !config.is_installed()
    } else {
        false
    }
}

/// Auto-install completion if appropriate
pub fn auto_install_completion() -> Result<()> {
    if !should_auto_install() {
        return Ok(());
    }

    let config = CompletionConfig::for_current_shell()
        .context("Failed to detect shell for completion setup")?;

    println!(
        "🚀 Setting up shell completion for {}...",
        config.shell.name()
    );

    config
        .install()
        .with_context(|| format!("Failed to install {} completion", config.shell.name()))?;

    println!("✅ Shell completion installed! Restart your shell or run:");
    match config.shell {
        Shell::Bash => println!("   source ~/.bashrc"),
        Shell::Zsh => println!("   source ~/.zshrc"),
        Shell::Unknown(_) => {}
    }

    Ok(())
}

/// Generate completions for current command line
pub fn generate_completions(args: &[String]) -> Result<Vec<String>> {
    let mut completions = Vec::new();

    // If we're completing the first argument after 'angreal'
    if args.len() <= 1 {
        // Add 'init' command if not in angreal project
        if crate::utils::is_angreal_project().is_err() {
            completions.push("init".to_string());
        } else {
            // Add discovered tasks
            completions.extend(get_available_tasks()?);
        }
        return Ok(completions);
    }

    // Handle 'init' command completion
    if args[0] == "init" && args.len() == 2 {
        // Complete template names
        completions.extend(templates::get_template_suggestions()?);
        return Ok(completions);
    }

    // Handle nested command completion
    if crate::utils::is_angreal_project().is_ok() {
        completions.extend(get_nested_command_completions(args)?);
    }

    Ok(completions)
}

/// Get available tasks in current project
fn get_available_tasks() -> Result<Vec<String>> {
    let mut tasks = Vec::new();

    // Load tasks (this triggers the same discovery as normal angreal execution)
    let angreal_path = crate::utils::is_angreal_project()?;
    let task_files = crate::utils::get_task_files(angreal_path)?;

    // Load task files to register commands
    for task_file in task_files {
        let _ = crate::utils::load_python(task_file); // Ignore errors for completion
    }

    // Get registered tasks
    for task in crate::task::ANGREAL_TASKS.lock().unwrap().iter() {
        // Add top-level task names
        if task.group.is_none() || task.group.as_ref().unwrap().is_empty() {
            tasks.push(task.name.clone());
        } else {
            // Add group names
            for group in task.group.as_ref().unwrap() {
                tasks.push(group.name.clone());
            }
        }
    }

    // Remove duplicates and sort
    tasks.sort();
    tasks.dedup();

    Ok(tasks)
}

/// Get completions for nested commands
fn get_nested_command_completions(_args: &[String]) -> Result<Vec<String>> {
    let mut completions = Vec::new();

    // This would need to match the command tree logic from builder/command_tree.rs
    // For now, return basic task names
    completions.extend(get_available_tasks()?);

    Ok(completions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_detection() {
        let shell = Shell::detect();
        // Should detect some shell or unknown
        match shell {
            Shell::Bash | Shell::Zsh | Shell::Unknown(_) => {}
        }
    }

    #[test]
    fn test_should_auto_install() {
        // Should not crash
        let _ = should_auto_install();
    }
}
