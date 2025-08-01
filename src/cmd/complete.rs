use anyhow::Result;
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
fn complete_paths(partial: &str, limit: usize, _current_dir: Option<&Path>) -> Result<Vec<String>> {
    let mut db = Database::open()?;
    let now = util::current_time()?;

    // Create stream options for completion - we want existing paths only
    // Don't use keywords initially - we'll filter manually for better prefix matching
    let options = StreamOptions::new(now).with_exclude(config::exclude_dirs()?).with_exists(true);

    let mut stream = Stream::new(&mut db, options);
    let mut results = Vec::new();

    // Collect all paths, then filter by prefix
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

    Ok(results)
}
