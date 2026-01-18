# templates


Template discovery for shell completion

Provides template suggestions for `angreal init` command from:
- Local cache (~/.angrealrc/)
- GitHub angreal organization repositories

## Structs

### `struct GitHubRepo`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


**Derives:** `Deserialize`

GitHub repository information

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `description` | `Option < String >` |  |
| `url` | `String` |  |



## Functions

### `fn get_template_suggestions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn get_template_suggestions () -> Result < Vec < String > >
```

Get template suggestions for completion

<details>
<summary>Source</summary>

```rust
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
```

</details>



### `fn get_local_templates`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn get_local_templates () -> Result < Vec < String > >
```

Get locally cached templates from ~/.angrealrc/

<details>
<summary>Source</summary>

```rust
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
```

</details>



### `fn get_github_templates`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn get_github_templates () -> Result < Vec < String > >
```

Get template repositories from GitHub angreal organization

<details>
<summary>Source</summary>

```rust
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
```

</details>



### `fn is_template_repo`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn is_template_repo (repo : & GitHubRepo) -> bool
```

Determine if a GitHub repository is a template (exclude meta repos)

<details>
<summary>Source</summary>

```rust
fn is_template_repo(repo: &GitHubRepo) -> bool {
    let name = repo.name.to_lowercase();

    // Skip anything that starts with "angreal"
    !name.starts_with("angreal")
}
```

</details>
