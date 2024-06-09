use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::file_op::gather_file_data;
use crate::path::{save_data, save_directory};
use crate::save_file_watcher::SaveFileUpdate;

pub type InnerType = HashMap<PathBuf, HashMap<SystemTime, Vec<u8>>>;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Storage(InnerType);

impl Storage {
    fn empty() -> Self {
        Self(InnerType::new())
    }

    pub fn read_saves() -> Self {
        Self::read_saves_from_serialized()
            .or_else(|_| Self::read_saves_from_files())
            .unwrap_or_else(|_| Self::empty())
    }

    fn read_saves_from_serialized() -> Result<Self, std::io::Error> {
        let saves_path =
            save_data().map_err(|_| std::io::Error::from(std::io::ErrorKind::NotFound))?;
        let bytes = std::fs::read(saves_path)?;

        postcard::from_bytes(&bytes)
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))
    }

    fn read_saves_from_files() -> Result<Self, std::io::Error> {
        let mut storage = Self::empty();

        let path = save_directory()?;
        std::fs::read_dir(path)?
            .flatten()
            .map(|e| e.path())
            .flat_map(|path| gather_file_data(&path))
            .for_each(|update| storage.apply_update(update));

        Ok(storage)
    }

    pub fn apply_update(&mut self, save_file_update: SaveFileUpdate) {
        let (path, time, data) = save_file_update;

        self.0.entry(path).or_default().insert(time, data);
    }
}

impl Deref for Storage {
    type Target = InnerType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
