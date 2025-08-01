use std::io::{self, Write};

use anyhow::Result;

use crate::cmd::{Init, InitShell, Run};
use crate::error::BrokenPipeHandler;
use crate::shell_gen;

impl Run for Init {
    fn run(&self) -> Result<()> {
        let cmd = if self.no_cmd { None } else { Some(self.cmd.as_str()) };
        let cmd_str = cmd.unwrap_or("z");

        let source = match self.shell {
            InitShell::Bash => shell_gen::generate_bash_init(cmd_str),
            InitShell::Fish => shell_gen::generate_fish_init(cmd_str),
            InitShell::Zsh => shell_gen::generate_zsh_init(cmd_str),
            InitShell::Powershell => shell_gen::generate_powershell_init(cmd_str),
            // Tier 3 shells - comment out per ROADMAP task 0.2a
            InitShell::Elvish => generate_placeholder("elvish", cmd),
            InitShell::Nushell => generate_placeholder("nushell", cmd),
            InitShell::Posix => generate_placeholder("posix", cmd),
            InitShell::Tcsh => generate_placeholder("tcsh", cmd),
            InitShell::Xonsh => generate_placeholder("xonsh", cmd),
        };

        writeln!(io::stdout(), "{source}").pipe_exit("stdout")
    }
}

// Placeholder function for Tier 3 shells (to be removed in Phase 0.2a)
fn generate_placeholder(shell: &str, cmd: Option<&str>) -> String {
    let cmd = cmd.unwrap_or("z");
    format!(
        r#"# Placeholder {shell} init for {cmd}
# TODO: Tier 3 shell support removed - use bash/zsh/fish/powershell instead
echo "zcd: {shell} shell not supported in zcd"
"#
    )
}
