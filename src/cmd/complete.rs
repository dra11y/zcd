use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::cmd::{Complete, Run};
use crate::config;
use crate::db::{Database, Stream, StreamOptions};
use crate::util;

impl Run for Complete {
    fn run(&self) -> Result<()> {
        let paths = complete_paths(&self.partial, self.limit, self.current_dir.as_deref())?;

        for path in paths {
            println!("{}", path);
        }

        Ok(())
    }
}

/// Main completion logic - queries database for paths matching partial input
fn complete_paths(partial: &str, limit: usize, current_dir: Option<&Path>) -> Result<Vec<String>> {
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
            results.push(path.to_string());

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

/// Get subdirectories from current directory that match the partial input
fn current_dir_subdirs(
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
                        // Convert to absolute path for consistency with database results
                        if let Ok(abs_path) = path.canonicalize() {
                            if let Some(abs_str) = abs_path.to_str() {
                                results.push(abs_str.to_string());
                            }
                        } else if let Some(path_str) = path.to_str() {
                            // Fallback to relative path if canonicalize fails
                            results.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(results)
}
