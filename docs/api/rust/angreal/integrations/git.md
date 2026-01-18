# git


## Structs

### `struct Git`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Git integration using git2/libgit2 for reliability and self-contained operation

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `working_dir` | `PathBuf` |  |

#### Methods

##### `new`


```rust
fn new (working_dir : Option < & Path >) -> Result < Self >
```

Create a new Git instance

<details>
<summary>Source</summary>

```rust
    pub fn new(working_dir: Option<&Path>) -> Result<Self> {
        let working_dir = working_dir
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| std::env::current_dir().unwrap());

        if !working_dir.exists() {
            bail!(
                "Working directory does not exist: {}",
                working_dir.display()
            );
        }

        Ok(Self { working_dir })
    }
```

</details>



##### `is_available`


```rust
fn is_available () -> bool
```

Check if git2 is available (always true since it's built-in)

<details>
<summary>Source</summary>

```rust
    pub fn is_available() -> bool {
        true
    }
```

</details>



##### `working_dir`


```rust
fn working_dir (& self) -> & Path
```

Get the working directory path

<details>
<summary>Source</summary>

```rust
    pub fn working_dir(&self) -> &Path {
        &self.working_dir
    }
```

</details>



##### `get_repo`


```rust
fn get_repo (& self) -> Result < Repository >
```

Get or open repository

<details>
<summary>Source</summary>

```rust
    fn get_repo(&self) -> Result<Repository> {
        Repository::open(&self.working_dir).with_context(|| {
            format!(
                "Failed to open repository at {}",
                self.working_dir.display()
            )
        })
    }
```

</details>



##### `get_signature`


```rust
fn get_signature (& self) -> Result < Signature < '_ > >
```

Get default signature for commits

<details>
<summary>Source</summary>

```rust
    fn get_signature(&self) -> Result<Signature<'_>> {
        // Try to get from git config, fallback to defaults
        let repo = self.get_repo()?;
        let config = repo.config()?;

        let name = config
            .get_string("user.name")
            .unwrap_or_else(|_| "Angreal User".to_string());
        let email = config
            .get_string("user.email")
            .unwrap_or_else(|_| "angreal@localhost".to_string());

        Signature::now(&name, &email).context("Failed to create git signature")
    }
```

</details>



##### `init`


```rust
fn init (& self , bare : bool) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn init(&self, bare: bool) -> Result<()> {
        if bare {
            Repository::init_bare(&self.working_dir)?;
        } else {
            Repository::init(&self.working_dir)?;
        }
        Ok(())
    }
```

</details>



##### `clone`


```rust
fn clone (remote : & str , destination : Option < & Path >) -> Result < PathBuf >
```

<details>
<summary>Source</summary>

```rust
    pub fn clone(remote: &str, destination: Option<&Path>) -> Result<PathBuf> {
        let dest_path = if let Some(dest) = destination {
            dest.to_path_buf()
        } else {
            // Extract repo name from URL
            let repo_name = extract_repo_name(remote)?;
            PathBuf::from(repo_name)
        };

        Repository::clone(remote, &dest_path)
            .with_context(|| format!("Failed to clone repository from {}", remote))?;

        Ok(dest_path)
    }
```

</details>



##### `add`


```rust
fn add (& self , paths : & [& str]) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn add(&self, paths: &[&str]) -> Result<()> {
        let repo = self.get_repo()?;
        let mut index = repo.index()?;

        for path in paths {
            if *path == "." {
                // Add all files in the working directory
                index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            } else {
                index.add_path(Path::new(path))?;
            }
        }

        index.write()?;
        Ok(())
    }
```

</details>



##### `commit`


```rust
fn commit (& self , message : & str , all : bool) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn commit(&self, message: &str, all: bool) -> Result<()> {
        let repo = self.get_repo()?;
        let signature = self.get_signature()?;
        let mut index = repo.index()?;

        if all {
            // Add all modified files
            index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            index.write()?;
        }

        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let parent_commit = match repo.head() {
            Ok(head) => {
                let oid = head
                    .target()
                    .ok_or_else(|| anyhow::anyhow!("HEAD has no target"))?;
                Some(repo.find_commit(oid)?)
            }
            Err(_) => None, // First commit
        };

        let parents = if let Some(parent) = &parent_commit {
            vec![parent]
        } else {
            vec![]
        };

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(())
    }
```

</details>



##### `push`


```rust
fn push (& self , remote : Option < & str > , branch : Option < & str >) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn push(&self, remote: Option<&str>, branch: Option<&str>) -> Result<()> {
        let repo = self.get_repo()?;
        let remote_name = remote.unwrap_or("origin");
        let mut remote = repo.find_remote(remote_name)?;

        let branch_name = branch.unwrap_or("HEAD");
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);

        remote.push(&[&refspec], None)?;
        Ok(())
    }
```

</details>



##### `pull`


```rust
fn pull (& self , _remote : Option < & str > , _branch : Option < & str >) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn pull(&self, _remote: Option<&str>, _branch: Option<&str>) -> Result<()> {
        // Use the existing git_pull_ff implementation (fast-forward only)
        // Note: This ignores remote/branch parameters and uses "origin/main"
        git_pull_ff(&self.working_dir.to_string_lossy());
        Ok(())
    }
```

</details>



##### `status`


```rust
fn status (& self , short : bool) -> Result < String >
```

<details>
<summary>Source</summary>

```rust
    pub fn status(&self, short: bool) -> Result<String> {
        let repo = self.get_repo()?;

        // Create status options with workaround for 32-bit systems
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);

        // On 32-bit systems, we may need to disable certain checks that can overflow
        // when dealing with large file system values
        #[cfg(target_pointer_width = "32")]
        {
            // Disable checks that might cause issues with large inode/timestamp values
            opts.include_ignored(false);
            opts.recurse_untracked_dirs(false);
        }

        let statuses = match repo.statuses(Some(&mut opts)) {
            Ok(s) => s,
            Err(e) => {
                // If we get an overflow error on 32-bit systems, try a simpler status
                if e.message().contains("Value too large") {
                    let mut simple_opts = StatusOptions::new();
                    simple_opts.include_untracked(false);
                    simple_opts.include_ignored(false);
                    simple_opts.recurse_untracked_dirs(false);

                    match repo.statuses(Some(&mut simple_opts)) {
                        Ok(s) => s,
                        Err(_) => {
                            // If even simple status fails, return a basic clean status
                            return Ok(if short {
                                String::new()
                            } else {
                                "nothing to commit, working tree clean\n".to_string()
                            });
                        }
                    }
                } else {
                    return Err(e.into());
                }
            }
        };

        let mut output = String::new();

        for entry in statuses.iter() {
            let status = entry.status();
            let path = entry.path().unwrap_or("???");

            if short {
                let mut flags = String::new();
                if status.contains(git2::Status::INDEX_NEW) {
                    flags.push('A');
                } else if status.contains(git2::Status::INDEX_MODIFIED) {
                    flags.push('M');
                } else if status.contains(git2::Status::INDEX_DELETED) {
                    flags.push('D');
                } else {
                    flags.push(' ');
                }

                if status.contains(git2::Status::WT_NEW) {
                    flags.push('?');
                } else if status.contains(git2::Status::WT_MODIFIED) {
                    flags.push('M');
                } else if status.contains(git2::Status::WT_DELETED) {
                    flags.push('D');
                } else {
                    flags.push(' ');
                }

                output.push_str(&format!("{} {}\n", flags, path));
            } else if status.contains(git2::Status::INDEX_NEW) {
                output.push_str(&format!("new file:   {}\n", path));
            } else if status.contains(git2::Status::INDEX_MODIFIED) {
                output.push_str(&format!("modified:   {}\n", path));
            } else if status.contains(git2::Status::WT_NEW) {
                output.push_str(&format!("untracked:  {}\n", path));
            } else if status.contains(git2::Status::WT_MODIFIED) {
                output.push_str(&format!("modified:   {}\n", path));
            }
        }

        if output.is_empty() && !short {
            output = "nothing to commit, working tree clean\n".to_string();
        }

        Ok(output)
    }
```

</details>



##### `branch`


```rust
fn branch (& self , name : Option < & str > , delete : bool) -> Result < String >
```

<details>
<summary>Source</summary>

```rust
    pub fn branch(&self, name: Option<&str>, delete: bool) -> Result<String> {
        let repo = self.get_repo()?;

        if let Some(branch_name) = name {
            if delete {
                let mut branch = repo.find_branch(branch_name, git2::BranchType::Local)?;
                branch.delete()?;
                Ok(format!("Deleted branch {}\n", branch_name))
            } else {
                // Create new branch
                let head = repo.head()?;
                let commit = head.peel_to_commit()?;
                repo.branch(branch_name, &commit, false)?;
                Ok(format!("Created branch {}\n", branch_name))
            }
        } else {
            // List branches
            let branches = repo.branches(Some(git2::BranchType::Local))?;
            let mut output = String::new();

            for branch in branches {
                let (branch, _) = branch?;
                if let Some(name) = branch.name()? {
                    if branch.is_head() {
                        output.push_str(&format!("* {}\n", name));
                    } else {
                        output.push_str(&format!("  {}\n", name));
                    }
                }
            }

            Ok(output)
        }
    }
```

</details>



##### `checkout`


```rust
fn checkout (& self , branch : & str , create : bool) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn checkout(&self, branch: &str, create: bool) -> Result<()> {
        let repo = self.get_repo()?;

        if create {
            // Create and checkout new branch
            let head = repo.head()?;
            let commit = head.peel_to_commit()?;
            let branch = repo.branch(branch, &commit, false)?;

            // Set HEAD to the new branch
            repo.set_head(&format!("refs/heads/{}", branch.name()?.unwrap()))?;
        } else {
            // Checkout existing branch
            let obj = repo.revparse_single(&format!("refs/heads/{}", branch))?;
            repo.checkout_tree(&obj, None)?;
            repo.set_head(&format!("refs/heads/{}", branch))?;
        }

        Ok(())
    }
```

</details>



##### `remote_add`


```rust
fn remote_add (& self , name : & str , url : & str) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn remote_add(&self, name: &str, url: &str) -> Result<()> {
        let repo = self.get_repo()?;
        repo.remote(name, url)?;
        Ok(())
    }
```

</details>



##### `remote_remove`


```rust
fn remote_remove (& self , name : & str) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn remote_remove(&self, name: &str) -> Result<()> {
        let repo = self.get_repo()?;
        repo.remote_delete(name)?;
        Ok(())
    }
```

</details>



##### `tag`


```rust
fn tag (& self , name : & str , message : Option < & str >) -> Result < () >
```

<details>
<summary>Source</summary>

```rust
    pub fn tag(&self, name: &str, message: Option<&str>) -> Result<()> {
        let repo = self.get_repo()?;
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;

        if let Some(msg) = message {
            let signature = self.get_signature()?;
            repo.tag(name, commit.as_object(), &signature, msg, false)?;
        } else {
            repo.tag_lightweight(name, commit.as_object(), false)?;
        }

        Ok(())
    }
```

</details>



##### `execute`


```rust
fn execute (& self , subcommand : & str , args : & [& str]) -> Result < GitOutput >
```

Compatibility method for subprocess-style interface This maps git commands to git2 operations for backwards compatibility

<details>
<summary>Source</summary>

```rust
    pub fn execute(&self, subcommand: &str, args: &[&str]) -> Result<GitOutput> {
        match subcommand {
            "init" => {
                let bare = args.contains(&"--bare");
                self.init(bare)?;
                Ok(GitOutput {
                    success: true,
                    exit_code: 0,
                    stdout: if bare {
                        format!(
                            "Initialized empty Git repository in {}\n",
                            self.working_dir.display()
                        )
                    } else {
                        format!(
                            "Initialized empty Git repository in {}/.git/\n",
                            self.working_dir.display()
                        )
                    },
                    stderr: String::new(),
                })
            }
            "add" => {
                self.add(args)?;
                Ok(GitOutput {
                    success: true,
                    exit_code: 0,
                    stdout: String::new(),
                    stderr: String::new(),
                })
            }
            "commit" => {
                let message = args
                    .iter()
                    .position(|&x| x == "-m")
                    .and_then(|i| args.get(i + 1))
                    .ok_or_else(|| anyhow::anyhow!("No commit message provided"))?;
                let all = args.contains(&"-a");
                self.commit(message, all)?;
                Ok(GitOutput {
                    success: true,
                    exit_code: 0,
                    stdout: String::new(),
                    stderr: String::new(),
                })
            }
            "status" => {
                let short = args.contains(&"--short");
                let output = self.status(short)?;
                Ok(GitOutput {
                    success: true,
                    exit_code: 0,
                    stdout: output,
                    stderr: String::new(),
                })
            }
            "branch" => {
                let delete = args.contains(&"-d");
                let name = args.iter().find(|&&arg| !arg.starts_with('-')).copied();
                let output = self.branch(name, delete)?;
                Ok(GitOutput {
                    success: true,
                    exit_code: 0,
                    stdout: output,
                    stderr: String::new(),
                })
            }
            _ => {
                bail!(
                    "Git command '{}' not supported by git2 integration",
                    subcommand
                )
            }
        }
    }
```

</details>



##### `execute_with_options`


```rust
fn execute_with_options (& self , subcommand : & str , options : HashMap < & str , & str > , args : & [& str] ,) -> Result < GitOutput >
```

Compatibility method for options-based interface

<details>
<summary>Source</summary>

```rust
    pub fn execute_with_options(
        &self,
        subcommand: &str,
        options: HashMap<&str, &str>,
        args: &[&str],
    ) -> Result<GitOutput> {
        match subcommand {
            "init" => {
                let bare = options.contains_key("bare") || options.get("bare") == Some(&"");
                self.init(bare)?;
                Ok(GitOutput {
                    success: true,
                    exit_code: 0,
                    stdout: if bare {
                        format!(
                            "Initialized empty Git repository in {}\n",
                            self.working_dir.display()
                        )
                    } else {
                        format!(
                            "Initialized empty Git repository in {}/.git/\n",
                            self.working_dir.display()
                        )
                    },
                    stderr: String::new(),
                })
            }
            _ => {
                // For other commands, convert options to args and call execute
                let mut combined_args = Vec::new();
                for (key, value) in options {
                    if key.len() > 1 {
                        if value.is_empty() {
                            combined_args.push(format!("--{}", key));
                        } else {
                            combined_args.push(format!("--{}={}", key, value));
                        }
                    } else {
                        combined_args.push(format!("-{}", key));
                        if !value.is_empty() {
                            combined_args.push(value.to_string());
                        }
                    }
                }
                combined_args.extend(args.iter().map(|s| s.to_string()));

                let arg_refs: Vec<&str> = combined_args.iter().map(|s| s.as_str()).collect();
                self.execute(subcommand, &arg_refs)
            }
        }
    }
```

</details>





### `struct GitOutput`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `success` | `bool` |  |
| `exit_code` | `i32` |  |
| `stdout` | `String` |  |
| `stderr` | `String` |  |



## Functions

### `fn extract_repo_name`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_repo_name (url : & str) -> Result < String >
```

<details>
<summary>Source</summary>

```rust
fn extract_repo_name(url: &str) -> Result<String> {
    let name = if url.starts_with("git@") || url.ends_with(".git") {
        url.split('/')
            .next_back()
            .and_then(|s| s.strip_suffix(".git"))
            .or_else(|| {
                url.split(':')
                    .next_back()?
                    .split('/')
                    .next_back()?
                    .strip_suffix(".git")
            })
    } else {
        url.split('/').next_back()
    };

    name.map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Cannot extract repository name from URL: {}", url))
}
```

</details>
