use std::sync::mpsc::{Receiver, Sender};
use std::sync::OnceLock;

use crate::save_file_watcher::{SaveFileUpdate, SaveFileWatcher};

mod context;
mod file_op;
mod path;
mod save_file;
mod save_file_event_handler;
mod save_file_watcher;
mod save_storage;
mod save_version;
mod state;
mod storage;
mod time_budget;
mod ui;
mod watcher_error;

static mut CHANNEL: OnceLock<(Sender<SaveFileUpdate>, Receiver<SaveFileUpdate>)> = OnceLock::new();

static WATCHER: OnceLock<SaveFileWatcher> = OnceLock::new();

fn main() {
    let watcher = SaveFileWatcher::new().expect("Save file watcher initialization failed");
    let _ = WATCHER.set(watcher);

    ui::run().expect("UI execution failed");
}
