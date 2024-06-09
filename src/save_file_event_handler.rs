use std::sync::mpsc;
use std::sync::mpsc::Sender;

use log::{Level, log};
use notify::event::{CreateKind, ModifyKind};
use notify::EventKind;

use crate::CHANNEL;
use crate::file_op::gather_file_data;
use crate::save_file_watcher::SaveFileUpdate;

pub struct SaveFileEventListener {
    sender: &'static Sender<SaveFileUpdate>,
}

impl SaveFileEventListener {
    pub fn new() -> Self {
        Self {
            sender: &unsafe { CHANNEL.get_or_init(mpsc::channel) }.0,
        }
    }
}

impl notify::EventHandler for SaveFileEventListener {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        let Ok(event) = event else {
            log!(Level::Error, "Event receive failed: {event:?}");
            return;
        };

        let (EventKind::Create(CreateKind::Any | CreateKind::File)
        | EventKind::Modify(ModifyKind::Any | ModifyKind::Data(_))) = event.kind
        else {
            log!(Level::Info, "Received event of wrong kind: {event:?}");
            return;
        };

        event
            .paths
            .iter()
            .flat_map(gather_file_data)
            .for_each(|event| {
                let _ = self.sender.send(event);
            });
    }
}
