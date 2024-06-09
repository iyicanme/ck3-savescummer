use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum WatcherError {
    UnderlyingImplementationInitialization,
    SaveFileDirectoryNotExists,
    WatchInitialization,
}

impl Display for WatcherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnderlyingImplementationInitialization => "'notify' initialization failed",
            Self::SaveFileDirectoryNotExists => "Could not obtain save directory path",
            Self::WatchInitialization => "'watcher' initialization failed",
        };

        write!(f, "{message}")
    }
}

impl std::error::Error for WatcherError {}
