use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::{Context, Result, ensure};
use glob::Pattern;

use crate::db::Rank;

pub fn data_dir() -> Result<PathBuf> {
    let dir = match env::var_os("_ZCD_DATA_DIR") {
        Some(path) => PathBuf::from(path),
        None => dirs::data_local_dir()
            .context("could not find data directory, please set _ZCD_DATA_DIR manually")?
            .join("zcd"),
    };

    ensure!(dir.is_absolute(), "_ZCD_DATA_DIR must be an absolute path");
    Ok(dir)
}

pub fn echo() -> bool {
    env::var_os("_ZCD_ECHO").is_some_and(|var| var == "1")
}

pub fn exclude_dirs() -> Result<Vec<Pattern>> {
    match env::var_os("_ZCD_EXCLUDE_DIRS") {
        Some(paths) => env::split_paths(&paths)
            .map(|path| {
                let pattern = path.to_str().context("invalid unicode in _ZCD_EXCLUDE_DIRS")?;
                Pattern::new(pattern)
                    .with_context(|| format!("invalid glob in _ZCD_EXCLUDE_DIRS: {pattern}"))
            })
            .collect(),
        None => {
            let pattern = (|| {
                let home = dirs::home_dir()?;
                let home = Pattern::escape(home.to_str()?);
                Pattern::new(&home).ok()
            })();
            Ok(pattern.into_iter().collect())
        }
    }
}

pub fn fzf_opts() -> Option<OsString> {
    env::var_os("_ZCD_FZF_OPTS")
}

pub fn maxage() -> Result<Rank> {
    env::var_os("_ZCD_MAXAGE").map_or(Ok(10_000.0), |maxage| {
        let maxage = maxage.to_str().context("invalid unicode in _ZCD_MAXAGE")?;
        let maxage = maxage
            .parse::<u32>()
            .with_context(|| format!("unable to parse _ZCD_MAXAGE as integer: {maxage}"))?;
        Ok(maxage as Rank)
    })
}

pub fn resolve_symlinks() -> bool {
    env::var_os("_ZCD_RESOLVE_SYMLINKS").is_some_and(|var| var == "1")
}

pub fn executable() -> String {
    env::var("_ZCD_EXECUTABLE").unwrap_or_else(|_| "zcd".to_string())
}
