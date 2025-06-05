//! Zsh completion script generation for Angreal

/// Generate zsh completion script for Angreal
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_completion_script() {
        let script = generate_completion_script();
        assert!(script.contains("#compdef angreal"));
        assert!(script.contains("_angreal"));
        assert!(!script.is_empty());
    }
}
