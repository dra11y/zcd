use std::io::{self, Write};

use anyhow::Result;

use crate::cmd::{Init, InitShell, Run};
use crate::config;
use crate::error::BrokenPipeHandler;

impl Run for Init {
    fn run(&self) -> Result<()> {
        let cmd = if self.no_cmd { None } else { Some(self.cmd.as_str()) };
        let echo = config::echo();
        let resolve_symlinks = config::resolve_symlinks();

        // Temporary placeholder - will be replaced with shell_gen module in Phase 1
        let source = match self.shell {
            InitShell::Bash => generate_bash_placeholder(cmd),
            InitShell::Fish => generate_fish_placeholder(cmd),
            InitShell::Zsh => generate_zsh_placeholder(cmd),
            InitShell::Powershell => generate_powershell_placeholder(cmd),
            // Tier 3 shells - comment out per ROADMAP task 0.2a (coming after 0.3c)
            InitShell::Elvish => generate_placeholder("elvish", cmd),
            InitShell::Nushell => generate_placeholder("nushell", cmd),
            InitShell::Posix => generate_placeholder("posix", cmd),
            InitShell::Tcsh => generate_placeholder("tcsh", cmd),
            InitShell::Xonsh => generate_placeholder("xonsh", cmd),
        };

        writeln!(io::stdout(), "{source}").pipe_exit("stdout")
    }
}

// Temporary placeholder functions - will be replaced with shell_gen module
fn generate_bash_placeholder(cmd: Option<&str>) -> String {
    let cmd = cmd.unwrap_or("z");
    format!(
        r#"# Placeholder bash init for {cmd}
# TODO: Replace with proper shell_gen implementation in Phase 1
echo "zcd init not yet implemented - Phase 1 pending"
"#
    )
}

fn generate_fish_placeholder(cmd: Option<&str>) -> String {
    let cmd = cmd.unwrap_or("z");
    format!(
        r#"# Placeholder fish init for {cmd}
# TODO: Replace with proper shell_gen implementation in Phase 1
echo "zcd init not yet implemented - Phase 1 pending"
"#
    )
}

fn generate_zsh_placeholder(cmd: Option<&str>) -> String {
    let cmd = cmd.unwrap_or("z");
    format!(
        r#"# Placeholder zsh init for {cmd}
# TODO: Replace with proper shell_gen implementation in Phase 1
echo "zcd init not yet implemented - Phase 1 pending"
"#
    )
}

fn generate_powershell_placeholder(cmd: Option<&str>) -> String {
    let cmd = cmd.unwrap_or("z");
    format!(
        r#"# Placeholder powershell init for {cmd}
# TODO: Replace with proper shell_gen implementation in Phase 1
Write-Output "zcd init not yet implemented - Phase 1 pending"
"#
    )
}

fn generate_placeholder(shell: &str, cmd: Option<&str>) -> String {
    let cmd = cmd.unwrap_or("z");
    format!(
        r#"# Placeholder {shell} init for {cmd}
# TODO: Replace with proper shell_gen implementation in Phase 1
echo "zcd init not yet implemented - Phase 1 pending"
"#
    )
}
