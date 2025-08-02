#!/usr/bin/env zsh
# zsh testing setup for zcd completion development

# Guard: only load once per session
if (( ${+functions[_z_complete]} )); then
    return
fi

export _ZCD_EXECUTABLE=./target/debug/zcd

# Load zsh completion system
autoload -U compinit
compinit

cargo build

# Define zcd
zcd() {
    $_ZCD_EXECUTABLE "$@"
}

# Define z function
z() {
    if [[ $# -eq 0 ]]; then
        cd ~
    elif [[ "$1" == "--help" ]]; then
        $_ZCD_EXECUTABLE --help
    elif [[ -d "$1" ]]; then
        cd "$1"
    else
        local result
        result="$($_ZCD_EXECUTABLE query --exclude "$(pwd)" -- "$@")"
        [[ -n "$result" ]] && cd "$result"
    fi
}

# Define completion function
_z_complete() {
    [[ -n "$_ZCD_DEBUG" ]] && echo "DEBUG: _z_complete called with words[CURRENT]='${words[CURRENT]}'" >&2
    local -a completions
    local result

    # Get completions from zcd complete command - use fast binary
    result="$($_ZCD_EXECUTABLE complete "${words[CURRENT]}" 2>/dev/null)"
    [[ -n "$_ZCD_DEBUG" ]] && echo "DEBUG: result='$result'" >&2

    if [[ -n "$result" ]]; then
        # Split result into array and add to completions
        completions=(${(f)result})
        [[ -n "$_ZCD_DEBUG" ]] && echo "DEBUG: completions=(${completions[@]})" >&2
        # Use compadd to add completions without immediate replacement
        compadd -a completions
        [[ -n "$_ZCD_DEBUG" ]] && echo "DEBUG: compadd executed" >&2
    else
        [[ -n "$_ZCD_DEBUG" ]] && echo "DEBUG: No result from completion" >&2
    fi
}

# Register the completion
compdef _z_complete z

echo "✅ zcd testing environment loaded!"
echo "   - Type 'z <partial><TAB>' to test TAB completion"
[[ -t "$_ZCD_DEBUG" ]] && echo "   - Set _ZCD_DEBUG=1 to enable completion debug output"
[[ -n "$_ZCD_DEBUG" ]] && echo "   - Uses '$_ZCD_EXECUTABLE' for all zcd calls"
[[ -n "$_ZCD_DEBUG" ]] && echo "   - Completion function: $(type _z_complete)"
[[ -n "$_ZCD_DEBUG" ]] && echo "   - Z function registered: $(compdef | grep ' z$')"
