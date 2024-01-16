use std::{
    fs,
    path::{Path, PathBuf},
};

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::WalkDir;

use crate::Result;

// region:			--- Dir utils
/// Returns true if  one or more dir was created
pub fn ensure_dir(dir: &Path) -> Result<bool> {
    if dir.is_dir() {
        Ok(false)
    } else {
        fs::create_dir_all(dir)?;
        Ok(true)
    }
}

pub fn list_files(
    dir: &Path,
    include_globs: Option<&[&str]>,
    exclude_globs: Option<&[&str]>,
) -> Result<Vec<PathBuf>> {
    let base_dir_exclude = base_dir_exclude_globs()?;

    // recursive depth
    let depth = include_globs
        .map(|globs| globs.iter().any(|&g| g.contains("**")))
        .map(|v| if v { 100 } else { 1 })
        .unwrap_or(1);

    // prep globs
    let include_globs = include_globs.map(get_glob_set).transpose()?;
    let exclude_globs = exclude_globs.map(get_glob_set).transpose()?;

    // build file iterator
    let walk_dir_it = WalkDir::new(dir)
        .max_depth(depth)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                !base_dir_exclude.is_match(e.path())
            } else {
                if let Some(exclude_globs) = exclude_globs.as_ref() {
                    if exclude_globs.is_match(e.path()) {
                        return false;
                    }
                }
                match include_globs.as_ref() {
                    Some(globs) => globs.is_match(e.path()),
                    None => true,
                }
            }
        });

    todo!()
}

fn base_dir_exclude_globs() -> Result<GlobSet> {
    get_glob_set(&[".git", "target"])
}

pub fn get_glob_set(globs: &[&str]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for glob in globs {
        builder.add(Glob::new(glob)?);
    }
    Ok(builder.build()?)
}
// endregion:		--- Dir utils
