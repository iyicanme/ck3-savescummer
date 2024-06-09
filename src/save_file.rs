use std::cmp::Ordering;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Eq, PartialEq)]
pub struct SaveFile(PathBuf, SystemTime);

impl SaveFile {
    pub const fn new(path_buf: PathBuf, time: SystemTime) -> Self {
        Self(path_buf, time)
    }

    pub const fn path(&self) -> &PathBuf {
        &self.0
    }

    pub const fn time(&self) -> &SystemTime {
        &self.1
    }
}

impl PartialOrd for SaveFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl Ord for SaveFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("Under no circumstances SaveFile::cmp should throw")
    }
}
