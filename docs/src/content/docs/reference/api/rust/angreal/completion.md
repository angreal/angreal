---
title: "angreal::completion"
description: "Shell completion support for Angreal"
editUrl: false
sidebar:
  order: 13
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Shell completion support for Angreal

Provides auto-installing shell completion for bash and zsh with:
- Real-time task discovery
- Template suggestions from GitHub
- Automatic setup on first run

## Structs

### <span id="struct.CompletionConfig"></span><code style="font-size: 0.85em">struct CompletionConfig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Configuration for shell completion

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `shell` | `Shell` |  |
| `install_path` | `PathBuf` |  |
| `completion_script` | `String` |  |

#### Methods

##### <span id="CompletionConfig.for_current_shell"></span><code>for_​current_​shell</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn for_current_shell() -> Result<Self>
```

Create completion config for detected shell

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="CompletionConfig.is_installed"></span><code>is_​installed</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn is_installed(&self) -> bool
```

Check if completion is already installed

<details>
<summary>Source</summary>

```rust
    pub fn is_installed(&self) -> bool {
        self.install_path.exists()
    }
```

</details>

##### <span id="CompletionConfig.install"></span><code>install</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn install(&self) -> Result<()>
```

Install completion script

<details>
<summary>Source</summary>

```rust
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
```

</details>

## Enums

### <span id="enum.Shell"></span><code style="font-size: 0.85em">enum Shell</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Supported shells for completion

#### Variants

- **`Bash`**
- **`Zsh`**
- **`Unknown`**

#### Methods

##### <span id="Shell.detect"></span><code>detect</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn detect() -> Self
```

Detect the current shell from environment

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="Shell.name"></span><code>name</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn name(&self) -> &str
```

Get the name of this shell

<details>
<summary>Source</summary>

```rust
    pub fn name(&self) -> &str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Unknown(name) => name,
        }
    }
```

</details>

## Functions

### <span id="fn.should_auto_install"></span><code style="font-size: 0.85em">fn should_​auto_​install</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn should_auto_install() -> bool
```

Check if completion should be auto-installed

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="fn.auto_install_completion"></span><code style="font-size: 0.85em">fn auto_​install_​completion</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn auto_install_completion() -> Result<()>
```

Auto-install completion if appropriate

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="fn.force_install_completion"></span><code style="font-size: 0.85em">fn force_​install_​completion</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn force_install_completion(shell: Option<&str>) -> Result<()>
```

Force install completion for specific shell or detected shell

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="fn.uninstall_completion"></span><code style="font-size: 0.85em">fn uninstall_​completion</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn uninstall_completion(shell: Option<&str>) -> Result<()>
```

Uninstall completion for specific shell or detected shell

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="fn.show_completion_status"></span><code style="font-size: 0.85em">fn show_​completion_​status</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn show_completion_status() -> Result<()>
```

Show completion installation status

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="fn.generate_completions"></span><code style="font-size: 0.85em">fn generate_​completions</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn generate_completions(args: &[String]) -> Result<Vec<String>>
```

Generate completions for current command line

<details>
<summary>Source</summary>

```rust
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
```

</details>
