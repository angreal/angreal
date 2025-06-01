//! Zsh completion script generation for Angreal

/// Generate zsh completion script for Angreal
pub fn generate_completion_script() -> String {
    r#"#compdef angreal
# Angreal zsh completion script
# Auto-generated - do not edit manually

_angreal() {
    local context state line
    typeset -A opt_args

    # Get current completion context
    local completion_words=("${words[@]:2}")  # Remove 'angreal' from words

    # Call angreal to get completions
    local IFS=$'\n'
    local completions=($(angreal _complete "${completion_words[@]}" 2>/dev/null))

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
