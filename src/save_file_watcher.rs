use std::path::PathBuf;
use std::time::SystemTime;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::path::save_directory;
use crate::save_file_event_handler::SaveFileEventListener;
use crate::watcher_error::WatcherError;

pub type SaveFileUpdate = (PathBuf, SystemTime, Vec<u8>);

pub struct SaveFileWatcher {
    _watcher: RecommendedWatcher,
}

impl SaveFileWatcher {
    pub fn new() -> Result<Self, WatcherError> {
        let listener = SaveFileEventListener::new();
        let mut watcher = notify::recommended_watcher(listener)
            .map_err(|_| WatcherError::UnderlyingImplementationInitialization)?;

        let save_directory_path =
            save_directory().map_err(|_| WatcherError::SaveFileDirectoryNotExists)?;
        watcher
            .watch(&save_directory_path, RecursiveMode::NonRecursive)
            .map_err(|_| WatcherError::WatchInitialization)?;

        Ok(Self { _watcher: watcher })
    }
}
