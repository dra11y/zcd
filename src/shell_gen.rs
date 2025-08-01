//! Static shell script generation module
//!
//! Replaces the askama template system with pure Rust string generation.
//! Generates minimal shell functions for zcd integration.

use std::fmt;

/// Supported shell types for initialization
#[derive(Debug, Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Power,
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shell::Bash => write!(f, "bash"),
            Shell::Zsh => write!(f, "zsh"),
            Shell::Fish => write!(f, "fish"),
            Shell::Power => write!(f, "powershell"),
        }
    }
}

/// Generate bash initialization script
pub fn generate_bash_init(cmd: &str) -> String {
    format!(
        r#"# zcd bash initialization
z() {{
    if [[ $# -eq 0 ]]; then
        cd ~
    elif [[ -d "$1" ]]; then
        cd "$1"
    else
        local result
        result="$(${{_ZCD_EXECUTABLE:-zcd}} query --exclude "$(pwd)" -- "$@")"
        [[ -n "$result" ]] && cd "$result"
    fi
}}

_z_complete() {{
    local candidates
    candidates="$(${{_ZCD_EXECUTABLE:-zcd}} complete "${{COMP_WORDS[COMP_CWORD]}}" 2>/dev/null)"
    COMPREPLY=($(compgen -W "$candidates" -- "${{COMP_WORDS[COMP_CWORD]}}"))
}}
complete -F _z_complete {cmd}"#
    )
}

/// Generate zsh initialization script
pub fn generate_zsh_init(cmd: &str) -> String {
    format!(
        r#"# zcd zsh initialization
z() {{
    if [[ $# -eq 0 ]]; then
        cd ~
    elif [[ -d "$1" ]]; then
        cd "$1"
    else
        local result
        result="$(${{_ZCD_EXECUTABLE:-zcd}} query --exclude "$(pwd)" -- "$@")"
        [[ -n "$result" ]] && cd "$result"
    fi
}}

_z_complete() {{
    local candidates
    candidates="$(${{_ZCD_EXECUTABLE:-zcd}} complete "${{words[CURRENT]}}" 2>/dev/null)"
    compadd -- ${{(f)candidates}}
}}
compdef _z_complete {cmd}"#
    )
}

/// Generate fish initialization script
pub fn generate_fish_init(cmd: &str) -> String {
    let _ = cmd; // Unused but kept for API consistency
    r#"# zcd fish initialization
function z
    if test (count $argv) -eq 0
        cd ~
    else if test -d "$argv[1]"
        cd "$argv[1]"
    else
        set result ((set -q _ZCD_EXECUTABLE; and echo $_ZCD_EXECUTABLE; or echo zcd) query --exclude (pwd) -- $argv)
        if test -n "$result"
            cd "$result"
        end
    end
end

function _z_complete
    (set -q _ZCD_EXECUTABLE; and echo $_ZCD_EXECUTABLE; or echo zcd) complete (commandline -ct) 2>/dev/null
end
complete -c z -f -a '(_z_complete)'"#.to_string()
}

/// Generate PowerShell initialization script
pub fn generate_powershell_init(cmd: &str) -> String {
    format!(
        "# zcd PowerShell initialization
function z {{
    param([Parameter(ValueFromPipeline = $true)] [string[]]$Path)

    $exe = if ($env:_ZCD_EXECUTABLE) {{ $env:_ZCD_EXECUTABLE }} else {{ 'zcd' }}

    if ($Path.Count -eq 0) {{
        Set-Location $env:USERPROFILE
    }} elseif (Test-Path $Path[0] -PathType Container) {{
        Set-Location $Path[0]
    }} else {{
        $result = & $exe query --exclude $PWD.Path -- @Path
        if ($result) {{ Set-Location $result }}
    }}
}}

function _z_complete {{
    param($wordToComplete, $commandAst, $cursorPosition)
    $exe = if ($env:_ZCD_EXECUTABLE) {{ $env:_ZCD_EXECUTABLE }} else {{ 'zcd' }}
    $result = & $exe complete $wordToComplete 2>$null
    if ($result) {{ $result -split [Environment]::NewLine }}
}}
Register-ArgumentCompleter -CommandName {cmd} -ScriptBlock {{ _z_complete @args }}"
    )
}

/// Generate shell initialization script for the given shell type
pub fn generate_init_script(shell: Shell, cmd: &str) -> String {
    match shell {
        Shell::Bash => generate_bash_init(cmd),
        Shell::Zsh => generate_zsh_init(cmd),
        Shell::Fish => generate_fish_init(cmd),
        Shell::Power => generate_powershell_init(cmd),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_display() {
        assert_eq!(Shell::Bash.to_string(), "bash");
        assert_eq!(Shell::Zsh.to_string(), "zsh");
        assert_eq!(Shell::Fish.to_string(), "fish");
        assert_eq!(Shell::Power.to_string(), "powershell");
    }

    #[test]
    fn test_bash_generation() {
        let script = generate_bash_init("z");
        assert!(script.contains("z() {"));
        assert!(script.contains("_z_complete() {"));
        assert!(script.contains("complete -F _z_complete z"));
        assert!(script.contains("${_ZCD_EXECUTABLE:-zcd} query"));
        assert!(script.contains("${_ZCD_EXECUTABLE:-zcd} complete"));
    }

    #[test]
    fn test_generated_scripts_reasonable_length() {
        let cmd = "zcd";
        // Scripts should be concise but functional - under 25 lines
        assert!(generate_bash_init(cmd).lines().count() < 25);
        assert!(generate_zsh_init(cmd).lines().count() < 25);
        assert!(generate_fish_init(cmd).lines().count() < 25);
        assert!(generate_powershell_init(cmd).lines().count() < 30); // PowerShell slightly longer
    }
}
