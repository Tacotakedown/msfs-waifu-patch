use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::time::SystemTime;

pub fn copy_and_replace_file(source_path: &str, destination_path: &str) -> io::Result<()> {
    let source_file = Path::new(source_path);

    if !source_file.exists() {
        return Err(Error::new(ErrorKind::NotFound, "Source file not found"));
    }

    let destination_file = Path::new(destination_path);
    if destination_file.exists() {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let backup_path = format!("{}.backup_{}", destination_path, timestamp);
        fs::copy(destination_path, &backup_path)?;
    }

    fs::copy(source_path, destination_path)?;

    Ok(())
}
