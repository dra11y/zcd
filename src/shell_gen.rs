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
    PowerShell,
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shell::Bash => write!(f, "bash"),
            Shell::Zsh => write!(f, "zsh"),
            Shell::Fish => write!(f, "fish"),
            Shell::PowerShell => write!(f, "powershell"),
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
        result="$({cmd} query --exclude "$(pwd)" -- "$@")"
        [[ -n "$result" ]] && cd "$result"
    fi
}}

_z_complete() {{
    local candidates
    candidates="$({cmd} complete "${{COMP_WORDS[COMP_CWORD]}}" 2>/dev/null)"
    COMPREPLY=($(compgen -W "$candidates" -- "${{COMP_WORDS[COMP_CWORD]}}"))
}}
complete -F _z_complete z"#,
        cmd = cmd
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
        result="$({cmd} query --exclude "$(pwd)" -- "$@")"
        [[ -n "$result" ]] && cd "$result"
    fi
}}

_z_complete() {{
    local candidates
    candidates="$({cmd} complete "${{words[CURRENT]}}" 2>/dev/null)"
    compadd -- ${{(f)candidates}}
}}
compdef _z_complete z"#,
        cmd = cmd
    )
}

/// Generate fish initialization script
pub fn generate_fish_init(cmd: &str) -> String {
    format!(
        r#"# zcd fish initialization
function z
    if test (count $argv) -eq 0
        cd ~
    else if test -d "$argv[1]"
        cd "$argv[1]"
    else
        set result ({cmd} query --exclude (pwd) -- $argv)
        if test -n "$result"
            cd "$result"
        end
    end
end

function _z_complete
    {cmd} complete (commandline -ct) 2>/dev/null
end
complete -c z -f -a '(_z_complete)'"#,
        cmd = cmd
    )
}

/// Generate PowerShell initialization script
pub fn generate_powershell_init(cmd: &str) -> String {
    format!(
        r#"# zcd PowerShell initialization
function z {{
    param([string]$Path)

    if (-not $Path) {{
        Set-Location ~
    }} elseif (Test-Path $Path -PathType Container) {{
        Set-Location $Path
    }} else {{
        $result = & {cmd} query --exclude (Get-Location).Path -- $Path
        if ($result) {{
            Set-Location $result
        }}
    }}
}}

Register-ArgumentCompleter -CommandName z -ScriptBlock {{
    param($commandName, $parameterName, $wordToComplete, $commandAst, $fakeBoundParameters)

    $candidates = & {cmd} complete $wordToComplete 2>$null
    if ($candidates) {{
        $candidates | ForEach-Object {{
            [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
        }}
    }}
}}"#,
        cmd = cmd
    )
}

/// Generate shell initialization script for the given shell type
pub fn generate_init_script(shell: Shell, cmd: &str) -> String {
    match shell {
        Shell::Bash => generate_bash_init(cmd),
        Shell::Zsh => generate_zsh_init(cmd),
        Shell::Fish => generate_fish_init(cmd),
        Shell::PowerShell => generate_powershell_init(cmd),
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
        assert_eq!(Shell::PowerShell.to_string(), "powershell");
    }

    #[test]
    fn test_bash_generation() {
        let script = generate_bash_init("zcd");
        assert!(script.contains("z() {"));
        assert!(script.contains("_z_complete() {"));
        assert!(script.contains("complete -F _z_complete z"));
        assert!(script.contains("zcd query"));
        assert!(script.contains("zcd complete"));
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
