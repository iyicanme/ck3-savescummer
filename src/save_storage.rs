use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::{Duration, SystemTime};

use crate::CHANNEL;
use crate::path::save_data;
use crate::save_file::SaveFile;
use crate::save_file_watcher::SaveFileUpdate;
use crate::save_version::SaveVersion;
use crate::storage::Storage;
use crate::time_budget::TimeBudget;

pub struct SaveStorage {
    storage: Storage,
    receiver: &'static Receiver<SaveFileUpdate>,
    ignore_list: HashSet<PathBuf>,
}

impl SaveStorage {
    pub fn new() -> Self {
        Self {
            receiver: &unsafe { CHANNEL.get_or_init(mpsc::channel) }.1,
            storage: Storage::read_saves(),
            ignore_list: HashSet::default(),
        }
    }

    pub fn update(&mut self) {
        let time_budget = TimeBudget::new(Duration::from_millis(1));

        let mut any_updated = false;
        while let Ok(update) = self.receiver.try_recv() {
            if self.ignore_list.remove(&update.0) {
                continue;
            }

            any_updated = true;

            self.storage.apply_update(update);

            if time_budget.is_expired() {
                break;
            }
        }

        if any_updated {
            let _ = self.write_to_file();
        }
    }

    pub fn save_files(&self) -> impl Iterator<Item = SaveFile> + '_ {
        let mut files = self
            .storage
            .iter()
            .map(|(path, saves)| {
                SaveFile::new(
                    path.clone(),
                    saves
                        .keys()
                        .max()
                        .unwrap_or(&SystemTime::UNIX_EPOCH)
                        .to_owned(),
                )
            })
            .collect::<Vec<SaveFile>>();

        files.sort();

        files.into_iter().rev()
    }

    pub fn save_versions(&self, file_path: &PathBuf) -> impl Iterator<Item = SaveVersion> + '_ {
        let mut versions = self
            .storage
            .get(file_path)
            .into_iter()
            .flatten()
            .map(|(k, _)| SaveVersion::new(*k))
            .collect::<Vec<SaveVersion>>();

        versions.sort();

        versions.into_iter().rev()
    }

    pub fn data_of(&self, path: &PathBuf, time: &SystemTime) -> Option<&[u8]> {
        self.storage.get(path)?.get(time).map(Vec::as_slice)
    }

    pub fn add_ignore_record(&mut self, path: PathBuf) {
        self.ignore_list.insert(path);
    }

    pub fn write_to_file(&self) -> Result<(), std::io::Error> {
        let bytes = postcard::to_stdvec(&self.storage)
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))?;
        let saves_path =
            save_data().map_err(|_| std::io::Error::from(std::io::ErrorKind::NotFound))?;
        std::fs::write(saves_path, bytes)?;

        Ok(())
    }
}
