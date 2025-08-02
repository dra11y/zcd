#!/usr/bin/env zsh
# zsh testing setup for zcd completion development

# Guard: only load once per session
if (( ${+functions[_z_complete]} )); then
    return
fi

# Load zsh completion system
autoload -U compinit
compinit

# Define zcd
zcd() {
    cargo run -- "$@"
}

# Define z function
z() {
    if [[ $# -eq 0 ]]; then
        cd ~
    elif [[ "$1" == "--help" ]]; then
        ${_ZCD_EXECUTABLE:-zcd} --help
    elif [[ -d "$1" ]]; then
        cd "$1"
    else
        local result
        result="$(${_ZCD_EXECUTABLE:-zcd} query --exclude "$(pwd)" -- "$@")"
        [[ -n "$result" ]] && cd "$result"
    fi
}

# Define completion function
_z_complete() {
    local -a completions
    local result
    
    # Get completions from zcd complete command (Rust gets current directory automatically)
    result="$(${_ZCD_EXECUTABLE:-zcd} complete "${words[CURRENT]}" 2>/dev/null)"
    
    if [[ -n "$result" ]]; then
        # Split result into array and add to completions
        completions=(${(f)result})
        # Use compadd to add completions without immediate replacement
        compadd -a completions
    fi
}

# Register the completion
compdef _z_complete z

echo "✅ zcd testing environment loaded!"
echo "   - Type 'z <partial><TAB>' to test TAB completion"
echo "   - Uses 'cargo run --' for all zcd calls"
