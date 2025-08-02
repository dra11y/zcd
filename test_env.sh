#!/usr/bin/env zsh
# zsh testing setup for zcd completion development

# Guard: only load once per session
if (( ${+functions[_z_complete]} )); then
    return
fi

# Load zsh completion system
autoload -U compinit
compinit

# Define z function
z() {
    if [[ $# -eq 0 ]]; then
        cd ~
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
    local result
    result="$(${_ZCD_EXECUTABLE:-zcd} interactive "${words[CURRENT]}" 2>/dev/null)"
    if [[ -n "$result" ]]; then
        # Execute the returned command (cd 'path')
        eval "$result"
        # Trigger completion acceptance
        BUFFER="z ${words[CURRENT]}"
        CURSOR=${#BUFFER}
        zle accept-line
    fi
}

# Register the completion
compdef _z_complete z

echo "✅ zcd testing environment loaded!"
echo "   - Type 'z <partial><TAB>' to test TAB completion"
echo "   - Uses 'cargo run --' for all zcd calls"
