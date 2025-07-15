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

/// Force install completion for specific shell or detected shell
pub fn force_install_completion(shell: Option<&str>) -> Result<()> {
    let config = if let Some(shell_name) = shell {
        // Create config for specific shell
        let shell_type = match shell_name {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            _ => anyhow::bail!("Unsupported shell: {}. Use 'bash' or 'zsh'", shell_name),
        };

        let home = env::var("HOME").context("HOME environment variable not set")?;
        let (install_path, completion_script) = match shell_type {
            Shell::Bash => {
                let path = PathBuf::from(&home)
                    .join(".bash_completion.d")
                    .join("angreal");
                let script = bash::generate_completion_script();
                (path, script)
            }
            Shell::Zsh => {
                let zsh_dir = find_zsh_completion_dir(&home)?;
                let path = zsh_dir.join("_angreal");
                let script = zsh::generate_completion_script();
                (path, script)
            }
            Shell::Unknown(_) => unreachable!(),
        };

        CompletionConfig {
            shell: shell_type,
            install_path,
            completion_script,
        }
    } else {
        // Use detected shell
        CompletionConfig::for_current_shell()
            .context("Failed to detect shell for completion setup")?
    };

    println!(
        "Installing {} completion{}...",
        config.shell.name(),
        if config.is_installed() {
            " (reinstalling)"
        } else {
            ""
        }
    );

    config
        .install()
        .with_context(|| format!("Failed to install {} completion", config.shell.name()))?;

    println!("✅ {} completion installed!", config.shell.name());
    println!("Restart your shell or run:");
    match config.shell {
        Shell::Bash => println!("   source ~/.bashrc"),
        Shell::Zsh => println!("   source ~/.zshrc"),
        Shell::Unknown(_) => {}
    }

    Ok(())
}

/// Uninstall completion for specific shell or detected shell
pub fn uninstall_completion(shell: Option<&str>) -> Result<()> {
    let configs = if let Some(shell_name) = shell {
        // Uninstall specific shell
        let shell_type = match shell_name {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            _ => anyhow::bail!("Unsupported shell: {}. Use 'bash' or 'zsh'", shell_name),
        };

        let home = env::var("HOME").context("HOME environment variable not set")?;
        let install_path = match shell_type {
            Shell::Bash => PathBuf::from(&home)
                .join(".bash_completion.d")
                .join("angreal"),
            Shell::Zsh => {
                let zsh_dir = find_zsh_completion_dir(&home)?;
                zsh_dir.join("_angreal")
            }
            Shell::Unknown(_) => unreachable!(),
        };

        vec![(shell_type, install_path)]
    } else {
        // Uninstall from all common locations
        let home = env::var("HOME").context("HOME environment variable not set")?;
        vec![
            (
                Shell::Bash,
                PathBuf::from(&home)
                    .join(".bash_completion.d")
                    .join("angreal"),
            ),
            (
                Shell::Zsh,
                PathBuf::from(&home)
                    .join(".zsh_completions")
                    .join("_angreal"),
            ),
            (
                Shell::Zsh,
                PathBuf::from(&home)
                    .join(".zsh")
                    .join("completions")
                    .join("_angreal"),
            ),
        ]
    };

    let mut removed_any = false;
    for (shell_type, path) in configs {
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("Failed to remove completion file: {}", path.display()))?;
            println!("✅ Removed {} completion", shell_type.name());
            removed_any = true;
        }
    }

    if !removed_any {
        println!("No completion files found to remove.");
    }

    Ok(())
}

/// Show completion installation status
pub fn show_completion_status() -> Result<()> {
    let home = env::var("HOME").context("HOME environment variable not set")?;

    // Check common completion locations
    let locations = vec![
        (
            "Bash",
            PathBuf::from(&home)
                .join(".bash_completion.d")
                .join("angreal"),
        ),
        (
            "Zsh (local)",
            PathBuf::from(&home)
                .join(".zsh_completions")
                .join("_angreal"),
        ),
        (
            "Zsh (oh-my-zsh)",
            PathBuf::from(&home)
                .join(".oh-my-zsh")
                .join("completions")
                .join("_angreal"),
        ),
        (
            "Zsh (system)",
            PathBuf::from("/usr/local/share/zsh/site-functions").join("_angreal"),
        ),
    ];

    println!("Shell completion status:");
    let mut found_any = false;

    for (name, path) in locations {
        if path.exists() {
            println!("  ✅ {} - installed at {}", name, path.display());
            found_any = true;
        } else {
            println!("  ❌ {} - not found", name);
        }
    }

    if !found_any {
        println!(
            "\nNo completion files found. Run 'angreal completion install' to set up completion."
        );
    }

    // Show current shell
    let current_shell = Shell::detect();
    println!("\nCurrent shell: {}", current_shell.name());

    Ok(())
}

/// Generate completions for current command line
pub fn generate_completions(args: &[String]) -> Result<Vec<String>> {
    let mut completions = Vec::new();

    // Filter out empty strings from args (shell completion often adds them)
    let filtered_args: Vec<String> = args.iter().filter(|s| !s.is_empty()).cloned().collect();

    // If we're completing the first argument after 'angreal'
    if filtered_args.is_empty() {
        // Always add built-in commands
        completions.push("alias".to_string());
        completions.push("tree".to_string());

        // Add 'init' command if not in angreal project
        if crate::utils::is_angreal_project().is_err() {
            completions.push("init".to_string());
        } else {
            // Add discovered tasks (top-level commands and groups)
            completions.extend(get_available_tasks()?);
        }
        return Ok(completions);
    }

    // Handle 'init' command completion
    if filtered_args.len() == 1 && filtered_args[0] == "init" {
        // Complete template names
        completions.extend(templates::get_template_suggestions()?);
        return Ok(completions);
    }

    // Handle 'alias' command completion
    if !filtered_args.is_empty() && filtered_args[0] == "alias" {
        if filtered_args.len() == 1 {
            // Complete subcommands for 'alias'
            completions.extend(vec![
                "create".to_string(),
                "remove".to_string(),
                "list".to_string(),
            ]);
            return Ok(completions);
        } else if filtered_args.len() == 2 && filtered_args[1] == "remove" {
            // Complete with existing aliases for 'alias remove'
            if let Ok(aliases) = crate::list_entrypoints() {
                completions.extend(aliases);
            }
            return Ok(completions);
        } else if filtered_args.len() >= 2 {
            // For 'alias create' or 'alias list', no further completion needed
            return Ok(completions);
        }
    }

    // Handle 'completion' command completion
    if !filtered_args.is_empty() && filtered_args[0] == "completion" {
        if filtered_args.len() == 1 {
            // Complete subcommands for 'completion'
            completions.extend(vec![
                "install".to_string(),
                "uninstall".to_string(),
                "status".to_string(),
            ]);
            return Ok(completions);
        } else if filtered_args.len() == 2
            && (filtered_args[1] == "install" || filtered_args[1] == "uninstall")
        {
            // Complete with shell options for 'completion install/uninstall'
            completions.extend(vec!["bash".to_string(), "zsh".to_string()]);
            return Ok(completions);
        } else if filtered_args.len() >= 2 {
            // For 'completion status' or other completed commands, no further completion needed
            return Ok(completions);
        }
    }

    // Handle nested command completion for angreal projects
    if crate::utils::is_angreal_project().is_ok() {
        // For any args, try to get nested completions
        // This will handle cases like "angreal test <TAB>" or "angreal group subgroup <TAB>"
        completions.extend(get_nested_command_completions(&filtered_args)?);
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
    for (_, task) in crate::task::ANGREAL_TASKS.lock().unwrap().iter() {
        if task.group.is_none() || task.group.as_ref().unwrap().is_empty() {
            // Top-level task - add the task name directly
            tasks.push(task.name.clone());
        } else {
            // Grouped task - add only the top-level group name for initial completion
            // The nested completion will handle deeper levels
            if let Some(groups) = &task.group {
                if let Some(first_group) = groups.first() {
                    tasks.push(first_group.name.clone());
                }
            }
        }
    }

    // Remove duplicates and sort
    tasks.sort();
    tasks.dedup();

    Ok(tasks)
}

/// Get completions for nested commands
fn get_nested_command_completions(args: &[String]) -> Result<Vec<String>> {
    use crate::builder::command_tree::CommandNode;

    let mut completions = Vec::new();

    // Build command tree from registered tasks
    let mut root = CommandNode::new_group("root".to_string(), None);

    // Load tasks
    let angreal_path = crate::utils::is_angreal_project()?;
    let task_files = crate::utils::get_task_files(angreal_path)?;

    // Load task files to register commands
    for task_file in task_files {
        let _ = crate::utils::load_python(task_file); // Ignore errors for completion
    }

    // Add all registered tasks to the command tree
    for (_, task) in crate::task::ANGREAL_TASKS.lock().unwrap().iter() {
        root.add_command(task.clone());
    }

    // Navigate the command tree based on the current args
    let mut current_node = &root;
    for arg in args {
        if let Some(child) = current_node.children.get(arg) {
            current_node = child;
        } else {
            // If we can't find this path, return empty completions
            return Ok(completions);
        }
    }

    // Return the names of all children at the current level
    for (name, child) in &current_node.children {
        // Only suggest groups if they have children, or commands if they're leaf nodes
        if !child.children.is_empty() || child.command.is_some() {
            completions.push(name.clone());
        }
    }

    // Sort for consistent output
    completions.sort();

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
