//! Integration tests for shell completion functionality
//! Tests actual shell integration with temporary filesystem isolation

use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Use the Shell enum from the main project
use zcd::shell_gen::Shell;

/// Test framework for isolated zcd testing
struct ZcdTestFramework {
    temp_dir: TempDir,
    db_dir: PathBuf,
}

impl ZcdTestFramework {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create isolated database directory
        let db_dir = temp_dir.path().join(".local/share/zcd");
        fs::create_dir_all(&db_dir)?;

        Ok(ZcdTestFramework { temp_dir, db_dir })
    }

    /// Create test directories in the isolated environment
    fn create_dirs(&self, dirs: &[&str]) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut created = Vec::new();
        for dir in dirs {
            let path = self.temp_dir.path().join(dir);
            fs::create_dir_all(&path)?;
            created.push(path);
        }
        Ok(created)
    }

    /// Get a zcd command configured for this test environment
    fn zcd(&self) -> Command {
        let mut cmd = Command::cargo_bin("zcd").unwrap();
        cmd.env("_ZCD_DATA_DIR", &self.db_dir);
        cmd.current_dir(self.temp_dir.path());
        cmd
    }

    /// Add directories to the zcd database
    fn add_to_database(&self, dirs: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
        for dir in dirs {
            self.zcd().args(["add", dir.to_str().unwrap()]).assert().success();
        }
        Ok(())
    }

    /// Add a directory multiple times (for frequency testing)
    fn add_repeatedly(
        &self,
        dir: &PathBuf,
        count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..count {
            self.zcd().args(["add", dir.to_str().unwrap()]).assert().success();
        }
        Ok(())
    }

    /// Get completion results as a vector of strings
    fn get_completions(&self, query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output =
            self.zcd().args(["complete", query]).assert().success().get_output().stdout.clone();

        let completions = String::from_utf8(output)?;
        Ok(completions.lines().map(|s| s.to_string()).collect())
    }

    /// Get shell initialization script
    fn get_init_script(&self, shell: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output =
            self.zcd().args(["init", shell]).assert().success().get_output().stdout.clone();

        Ok(String::from_utf8(output)?)
    }

    /// Test that a shell is available on the system
    fn require_shell(shell: Shell) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("which").arg(&shell.to_string()).assert().success();
        Ok(())
    }

    /// Test completion workflow for a specific shell (reusable)
    fn test_shell_completion_workflow(
        &self,
        shell: Shell,
        prefix: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Test shell init generation works
        self.get_init_script(&shell.to_string())?;

        // Create test directories and add to database
        let test_dirs =
            self.create_dirs(&[&format!("{}_project1", prefix), &format!("{}_project2", prefix)])?;
        self.add_to_database(&test_dirs)?;

        // Test completion command works
        let completions = self.get_completions(&format!("{}_proj", prefix))?;

        // Should return project directories
        assert!(completions.len() >= 2, "Should find {} project directories", shell);
        assert!(
            completions.iter().any(|c| c.contains(&format!("{}_project1", prefix))),
            "Should include {}_project1",
            prefix
        );
        assert!(
            completions.iter().any(|c| c.contains(&format!("{}_project2", prefix))),
            "Should include {}_project2",
            prefix
        );

        Ok(())
    }

    /// Test basic completion functionality (reusable pattern)
    fn test_basic_completion(
        &self,
        dirs: &[&str],
        query: &str,
        expected_matches: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let test_dirs = self.create_dirs(dirs)?;
        self.add_to_database(&test_dirs)?;

        let completions = self.get_completions(query)?;

        assert!(
            completions.len() >= expected_matches.len(),
            "Should find at least {} directories",
            expected_matches.len()
        );

        for expected in expected_matches {
            assert!(
                completions.iter().any(|c| c.contains(expected)),
                "Should include {}",
                expected
            );
        }

        Ok(())
    }
}

#[test]
fn test_zsh_init_generation() -> Result<(), Box<dyn std::error::Error>> {
    ZcdTestFramework::require_shell(Shell::Zsh)?;

    let framework = ZcdTestFramework::new()?;

    // Test that zsh init generates without errors
    let init_script = framework.get_init_script(&Shell::Zsh.to_string())?;

    // Verify it contains expected zsh completion components
    assert!(init_script.contains("compdef"), "Should contain zsh compdef");
    assert!(init_script.contains("_z_complete"), "Should contain completion function");
    assert!(init_script.len() > 50, "Should generate meaningful content");
    assert!(init_script.len() < 1000, "Should be reasonably concise");

    Ok(())
}

#[test]
fn test_bash_init_generation() -> Result<(), Box<dyn std::error::Error>> {
    ZcdTestFramework::require_shell(Shell::Bash)?;

    let framework = ZcdTestFramework::new()?;

    // Test that bash init generates without errors
    let init_script = framework.get_init_script(&Shell::Bash.to_string())?;

    // Verify it contains expected bash completion components
    assert!(init_script.contains("complete -F"), "Should contain bash complete command");
    assert!(init_script.contains("_z_complete"), "Should contain completion function");
    assert!(init_script.contains("COMPREPLY"), "Should contain bash COMPREPLY");
    assert!(init_script.len() > 50, "Should generate meaningful content");
    assert!(init_script.len() < 1000, "Should be reasonably concise");

    Ok(())
}

#[test]
fn test_completion_basic_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let framework = ZcdTestFramework::new()?;

    framework.test_basic_completion(
        &["project1", "project2", "workspace"],
        "proj",
        &["project1", "project2"],
    )?;

    Ok(())
}

#[test]
fn test_completion_frequency_ordering() -> Result<(), Box<dyn std::error::Error>> {
    let framework = ZcdTestFramework::new()?;

    // Create test directories
    let test_dirs = framework.create_dirs(&["frequent", "rare", "medium"])?;

    // Add with different frequencies
    framework.add_repeatedly(&test_dirs[0], 10)?; // frequent - 10 times
    framework.add_repeatedly(&test_dirs[1], 1)?; // rare - 1 time
    framework.add_repeatedly(&test_dirs[2], 5)?; // medium - 5 times

    // Get all completions
    let completions = framework.get_completions("")?;

    // Should have results
    assert!(completions.len() >= 3, "Should find all directories");

    // Most frequent should be first (this tests the core frecency algorithm)
    assert!(
        completions[0].contains("frequent"),
        "Most frequent directory should be first: {}",
        completions[0]
    );

    Ok(())
}

#[test]
fn test_completion_filesystem_fallback() -> Result<(), Box<dyn std::error::Error>> {
    let framework = ZcdTestFramework::new()?;

    // Create directories but don't add to database - test filesystem fallback
    framework.create_dirs(&["fs_only1", "fs_only2"])?;

    let completions = framework.get_completions("fs_")?;

    assert!(completions.len() >= 2, "Should find filesystem directories");
    assert!(completions.iter().any(|c| c.contains("fs_only1")), "Should include fs_only1");
    assert!(completions.iter().any(|c| c.contains("fs_only2")), "Should include fs_only2");

    Ok(())
}

#[test]
fn test_completion_merging() -> Result<(), Box<dyn std::error::Error>> {
    let framework = ZcdTestFramework::new()?;

    // Create directories
    let db_dirs = framework.create_dirs(&["database_dir"])?;
    framework.create_dirs(&["filesystem_dir"])?;

    // Add only one to database
    framework.add_to_database(&db_dirs)?;

    // Test completion gets both database and filesystem results
    let completions = framework.get_completions("")?;

    assert!(
        completions.iter().any(|c| c.contains("database_dir")),
        "Should include database directory"
    );
    assert!(
        completions.iter().any(|c| c.contains("filesystem_dir")),
        "Should include filesystem directory"
    );

    Ok(())
}

#[test]
fn test_bash_completion_workflow() -> Result<(), Box<dyn std::error::Error>> {
    ZcdTestFramework::require_shell(Shell::Bash)?;

    let framework = ZcdTestFramework::new()?;
    framework.test_shell_completion_workflow(Shell::Bash, "bash")?;

    Ok(())
}

#[test]
fn test_zsh_completion_workflow() -> Result<(), Box<dyn std::error::Error>> {
    ZcdTestFramework::require_shell(Shell::Zsh)?;

    let framework = ZcdTestFramework::new()?;
    framework.test_shell_completion_workflow(Shell::Zsh, "zsh")?;

    Ok(())
}

#[test]
fn test_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let framework = ZcdTestFramework::new()?;

    // Test completion with non-existent path (should not crash)
    let _completions = framework.get_completions("/nonexistent/path")?;
    // Empty results are OK, just shouldn't crash

    // Test completion with empty database
    let _completions = framework.get_completions("anything")?;
    // Should work (might return filesystem results)

    Ok(())
}
