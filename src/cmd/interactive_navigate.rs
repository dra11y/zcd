use anyhow::Result;

use crate::cmd::complete::complete_paths;
use crate::cmd::{InteractiveNavigate, Run};

impl Run for InteractiveNavigate {
    fn run(&self) -> Result<()> {
        let current_dir = std::env::current_dir().ok();
        let partial = self.partial.as_deref().unwrap_or("");

        // Get initial completions to decide behavior
        let completions = if self.filesystem_only {
            super::complete::current_dir_subdirs(partial, current_dir.as_deref(), self.limit)?
        } else {
            complete_paths(partial, self.limit, current_dir.as_deref())?
        };

        match completions.len() {
            0 => {
                // No matches - could show error or just exit silently
                // For now, just exit silently like bash completion does
                Ok(())
            }
            1 => {
                // Exactly one match - auto-complete immediately
                println!("cd '{}'", completions[0]);
                Ok(())
            }
            _ => {
                // Multiple matches - show interactive menu
                self.show_interactive_menu(&completions, partial)
            }
        }
    }
}

impl InteractiveNavigate {
    fn show_interactive_menu(
        &self,
        _initial_completions: &[String],
        initial_partial: &str,
    ) -> Result<()> {
        let limit = self.limit;
        let filesystem_only = self.filesystem_only;

        let autocomplete_fn =
            move |input: &str| -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
                let current_dir = std::env::current_dir().ok();

                let completions = if filesystem_only {
                    super::complete::current_dir_subdirs(input, current_dir.as_deref(), limit)
                        .map_err(|e| {
                            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
                                as Box<dyn std::error::Error + Send + Sync>
                        })?
                } else {
                    complete_paths(input, limit, current_dir.as_deref()).map_err(|e| {
                        Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
                            as Box<dyn std::error::Error + Send + Sync>
                    })?
                };

                Ok(completions)
            };

        let prompt = if initial_partial.is_empty() {
            "Navigate to: ".to_string()
        } else {
            format!("Navigate to (starting with '{}'): ", initial_partial)
        };

        let selection = inquire::Text::new(&prompt)
            .with_autocomplete(autocomplete_fn)
            .with_initial_value(initial_partial)
            .with_help_message("Type to filter paths, TAB to complete, ESC to cancel")
            .prompt();

        match selection {
            Ok(path) => {
                // Output the shell command to execute
                println!("cd '{}'", path);
                Ok(())
            }
            Err(inquire::InquireError::OperationCanceled) => {
                // User pressed ESC - just exit cleanly
                Ok(())
            }
            Err(err) => anyhow::bail!("Interactive navigation failed: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_navigate_creation() {
        let cmd = InteractiveNavigate {
            partial: Some("/home".to_string()),
            limit: 10,
            filesystem_only: false,
        };

        assert_eq!(cmd.partial, Some("/home".to_string()));
        assert_eq!(cmd.limit, 10);
        assert!(!cmd.filesystem_only);
    }
}
