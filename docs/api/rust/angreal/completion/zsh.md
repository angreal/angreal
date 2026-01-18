# zsh


Zsh completion script generation for Angreal

## Functions

### `fn generate_completion_script`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn generate_completion_script () -> String
```

Generate zsh completion script for Angreal

<details>
<summary>Source</summary>

```rust
pub fn generate_completion_script() -> String {
    r#"#compdef angreal
# Angreal zsh completion script
# Auto-generated - do not edit manually

_angreal() {
    local context state line
    typeset -A opt_args

    # Build arguments to pass to completion (exclude 'angreal' and current incomplete word)
    local completion_args=()
    local i
    for (( i=2; i < CURRENT; i++ )); do
        completion_args+=("${words[i]}")
    done

    # Call angreal to get completions
    local IFS=$'\n'
    local completions=($(angreal _complete "${completion_args[@]}" 2>/dev/null))

    if (( ${#completions[@]} > 0 )); then
        # Use custom completions
        _describe 'angreal commands' completions
    else
        # Fallback to file completion
        _files
    fi
}

# Enable zsh completion for angreal
_angreal "$@"
"#
    .to_string()
}
```

</details>
