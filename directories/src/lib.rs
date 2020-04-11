#[cfg(test)]
mod tests;

use std::fs;
use std::io::Result;
use std::path::Path;

/// Checks if directory exists and attempts to create one
/// if it doesn't exist. Returns error if it was not able
/// to create one.
///
/// # Arguments
///
/// * `path` - the path that needs to exist or create.
pub fn must_exist(path: &str) -> Result<()> {
    let path = Path::new(path);

    if path.is_dir() {
        return Ok(());
    }

    fs::create_dir(path)?;

    Ok(())
}
