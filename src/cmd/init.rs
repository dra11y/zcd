use std::io::{self, Write};

use anyhow::Result;

use crate::cmd::{Init, InitShell, Run};
use crate::error::BrokenPipeHandler;
use crate::shell_gen::{self, Shell};

impl TryFrom<InitShell> for Shell {
    type Error = &'static str;

    fn try_from(init_shell: InitShell) -> std::result::Result<Self, Self::Error> {
        match init_shell {
            InitShell::Bash => Ok(Shell::Bash),
            InitShell::Zsh => Ok(Shell::Zsh),
            InitShell::Fish => Ok(Shell::Fish),
            InitShell::Powershell => Ok(Shell::Power),
            // Tier 3 shells not supported in our Shell enum
            InitShell::Elvish => Err("elvish not supported"),
            InitShell::Nushell => Err("nushell not supported"),
            InitShell::Posix => Err("posix not supported"),
            InitShell::Tcsh => Err("tcsh not supported"),
            InitShell::Xonsh => Err("xonsh not supported"),
        }
    }
}

impl Run for Init {
    fn run(&self) -> Result<()> {
        let cmd = if self.no_cmd { None } else { Some(self.cmd.as_str()) };
        let cmd_str = cmd.unwrap_or("z");

        let source = match Shell::try_from(self.shell) {
            Ok(shell) => shell_gen::generate_init_script(shell, cmd_str),
            Err(_) => {
                // Tier 3 shell - generate placeholder
                let shell_name = match self.shell {
                    InitShell::Elvish => "elvish",
                    InitShell::Nushell => "nushell",
                    InitShell::Posix => "posix",
                    InitShell::Tcsh => "tcsh",
                    InitShell::Xonsh => "xonsh",
                    _ => unreachable!(), // All supported shells handled above
                };
                generate_placeholder(shell_name, cmd)
            }
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
