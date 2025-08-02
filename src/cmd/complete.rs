use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cmd::{Complete, Run};
use crate::config;
use crate::db::{Database, Stream, StreamOptions};
use crate::util;

impl Run for Complete {
    fn run(&self) -> Result<()> {
        // Get current directory from environment instead of requiring parameter
        let current_dir = std::env::current_dir().ok();
        let paths = complete_paths(&self.partial, self.limit, current_dir.as_deref())?;

        for path in paths {
            println!("{path}");
        }

        Ok(())
    }
}

/// Main completion logic - queries database for paths matching partial input
pub fn complete_paths(
    partial: &str,
    limit: usize,
    current_dir: Option<&Path>,
) -> Result<Vec<String>> {
    let mut db = Database::open()?;
    let now = util::current_time()?;

    // Create stream options for completion - we want existing paths only
    // Don't use keywords initially - we'll filter manually for better prefix matching
    let options = StreamOptions::new(now).with_exclude(config::exclude_dirs()?).with_exists(true);

    let mut stream = Stream::new(&mut db, options);
    let mut results = Vec::new();

    // Collect database paths that match the prefix
    while let Some(dir) = stream.next() {
        let path = dir.path.as_ref();

        // If partial is provided, only include paths that start with it
        if partial.is_empty() || path.starts_with(partial) {
            // Smart path formatting: relative for current subtree, absolute for others
            let formatted_path = format_path_for_completion(path, current_dir);
            results.push(formatted_path);

            if results.len() >= limit {
                break;
            }
        }
    }

    // If we have room for more results, add filesystem fallback
    if results.len() < limit {
        let fs_results = current_dir_subdirs(partial, current_dir, limit - results.len())?;
        for fs_path in fs_results {
            // Avoid duplicates from database
            if !results.contains(&fs_path) {
                results.push(fs_path);
                if results.len() >= limit {
                    break;
                }
            }
        }
    }

    Ok(results)
}

/// Format a path for completion based on current directory context
/// - Current subtree: return relative path (e.g., "subdir/target")
/// - Direct ancestry: return relative with .. (e.g., "..", "../..", "../sibling")  
/// - Complex paths: return absolute path for clarity
fn format_path_for_completion(path: &str, current_dir: Option<&Path>) -> String {
    let Some(current) = current_dir else {
        return path.to_string();
    };

    let path_buf = Path::new(path);
    let current_abs = current.canonicalize().unwrap_or_else(|_| current.to_path_buf());
    let target_abs = path_buf.canonicalize().unwrap_or_else(|_| path_buf.to_path_buf());

    // Case 1: Target is within current directory subtree
    if target_abs.starts_with(&current_abs) {
        if let Ok(relative) = target_abs.strip_prefix(&current_abs) {
            let relative_str = relative.to_string_lossy();
            return if relative_str.is_empty() {
                ".".to_string()
            } else {
                relative_str.to_string()
            };
        }
    }

    // Case 2: Check if we can create a reasonable relative path with ../
    // Find common ancestor
    let common_ancestor = find_common_ancestor(&current_abs, &target_abs);
    
    if let Some(ancestor) = common_ancestor {
        // Calculate depth from current to common ancestor
        let current_depth = current_abs.strip_prefix(&ancestor).map(|p| p.components().count()).unwrap_or(0);
        let target_relative = target_abs.strip_prefix(&ancestor).unwrap_or(&target_abs);
        
        // Only use relative path if it's reasonable (max 3 levels up)
        if current_depth <= 3 {
            let up_dirs = "../".repeat(current_depth);
            let target_path = target_relative.to_string_lossy();
            
            return if target_path.is_empty() {
                up_dirs.trim_end_matches('/').to_string()
            } else {
                format!("{}{}", up_dirs, target_path)
            };
        }
    }

    // Case 3: Use absolute path for complex cases
    path.to_string()
}

/// Find the common ancestor of two paths
fn find_common_ancestor(path1: &Path, path2: &Path) -> Option<PathBuf> {
    let components1: Vec<_> = path1.components().collect();
    let components2: Vec<_> = path2.components().collect();
    
    let mut common = PathBuf::new();
    
    for (c1, c2) in components1.iter().zip(components2.iter()) {
        if c1 == c2 {
            common.push(c1);
        } else {
            break;
        }
    }
    
    if common.as_os_str().is_empty() {
        None
    } else {
        Some(common)
    }
}

/// Get subdirectories from current directory that match the partial input
pub fn current_dir_subdirs(
    partial: &str,
    current_dir: Option<&Path>,
    limit: usize,
) -> Result<Vec<String>> {
    let dir = current_dir.unwrap_or_else(|| Path::new("."));

    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if results.len() >= limit {
                break;
            }

            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    // For filesystem completion, match against directory name, not full path
                    if partial.is_empty() || dir_name.starts_with(partial) {
                        // Use relative path for current directory subdirectories
                        results.push(dir_name.to_string());
                    }
                }
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_current_dir_subdirs_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let result = current_dir_subdirs("", Some(temp_dir.path()), 10).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_current_dir_subdirs_with_directories() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test directories
        fs::create_dir(dir_path.join("project1")).unwrap();
        fs::create_dir(dir_path.join("project2")).unwrap();
        fs::create_dir(dir_path.join("other")).unwrap();

        // Test empty prefix (should return all)
        let result = current_dir_subdirs("", Some(dir_path), 10).unwrap();
        assert_eq!(result.len(), 3);

        // Test prefix matching
        let result = current_dir_subdirs("proj", Some(dir_path), 10).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.contains("project1")));
        assert!(result.iter().any(|p| p.contains("project2")));

        // Test limit
        let result = current_dir_subdirs("", Some(dir_path), 2).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_current_dir_subdirs_prefix_matching() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        fs::create_dir(dir_path.join("alpha")).unwrap();
        fs::create_dir(dir_path.join("beta")).unwrap();
        fs::create_dir(dir_path.join("alphabet")).unwrap();

        let result = current_dir_subdirs("alph", Some(dir_path), 10).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.contains("alpha")));
        assert!(result.iter().any(|p| p.contains("alphabet")));
        assert!(!result.iter().any(|p| p.contains("beta")));
    }

    #[test]
    fn test_current_dir_subdirs_ignores_files() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create both directories and files
        fs::create_dir(dir_path.join("directory")).unwrap();
        fs::write(dir_path.join("file.txt"), "content").unwrap();

        let result = current_dir_subdirs("", Some(dir_path), 10).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("directory"));
    }
}
