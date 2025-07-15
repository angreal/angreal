//! Bash completion script generation for Angreal

/// Generate bash completion script for Angreal
pub fn generate_completion_script() -> String {
    r#"#!/bin/bash
# Angreal bash completion script
# Auto-generated - do not edit manually

_angreal_completion() {
    local cur prev words cword
    _get_comp_words_by_ref -n : cur prev words cword

    # Remove 'angreal' from words array for completion
    local completion_words=("${words[@]:1}")

    # Call angreal to get completions for current context
    local IFS=$'\n'
    local completions=($(angreal _complete "${completion_words[@]}" 2>/dev/null))

    if [ ${#completions[@]} -eq 0 ]; then
        # Fallback to file completion if no custom completions
        COMPREPLY=($(compgen -f -- "$cur"))
    else
        # Use custom completions
        COMPREPLY=($(compgen -W "${completions[*]}" -- "$cur"))
    fi
}

# Enable bash completion for angreal
complete -F _angreal_completion angreal

# Handle programmable completion availability
if ! declare -F _get_comp_words_by_ref >/dev/null 2>&1; then
    # Fallback implementation for systems without bash-completion
    _get_comp_words_by_ref() {
        local exclude flag i OPTIND=1
        local cur cword words=()

        while getopts "n:" flag "$@"; do
            case $flag in
                n) exclude=$OPTARG ;;
            esac
        done

        # Simple word splitting
        words=(${COMP_LINE})
        cur="${words[COMP_CWORD]}"
        cword=$COMP_CWORD

        # Export variables
        for i in "$@"; do
            case $i in
                cur) eval $i='$cur' ;;
                prev) eval $i='${words[COMP_CWORD-1]}' ;;
                words) eval $i='("${words[@]}")' ;;
                cword) eval $i='$cword' ;;
            esac
        done
    }
fi
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_completion_script() {
        let script = generate_completion_script();
        assert!(script.contains("_angreal_completion"));
        assert!(script.contains("complete -F _angreal_completion angreal"));
        assert!(!script.is_empty());
    }
}
