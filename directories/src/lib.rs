use std::fs;
use std::io::Result;
use std::path::Path;

pub fn must_exist(path: &str) -> Result<()> {
    let path = Path::new(path);

    if path.is_dir() {
        return Ok(());
    }

    fs::create_dir(path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn must_exist_suceeds_with_existing() -> Result<()> {
        const DIR: &str = "preexisting_dir";

        fs::create_dir(DIR)?;

        let result = must_exist(DIR);

        fs::remove_dir(DIR)?;

        result
    }

    #[test]
    fn must_exist_creates_if_able() -> Result<()> {
        const DIR: &str = "inexisting_dir";

        let result = must_exist(&DIR);

        if Path::new(DIR).is_dir() {
            fs::remove_dir(DIR)?;
        } else {
            panic!("Dir was not created");
        }

        result
    }

    #[test]
    fn must_exist_fails_on_file() -> Result<()> {
        const DIR: &str = "existing_file";

        File::create(DIR)?;
        let result = must_exist(DIR);

        fs::remove_file(DIR)?;

        if result.is_err() {
            Ok(())
        } else {
            panic!("Did not fail with existing file");
        }
    }
}
