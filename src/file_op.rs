use std::path::PathBuf;

use crate::save_file_watcher::SaveFileUpdate;

pub fn gather_file_data(path: &PathBuf) -> Result<SaveFileUpdate, std::io::Error> {
    let modified = std::fs::metadata(path)?.modified()?;
    let save_data = std::fs::read(path)?;

    Ok((path.clone(), modified, save_data))
}
