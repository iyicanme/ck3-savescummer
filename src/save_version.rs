use std::cmp::Ordering;
use std::time::SystemTime;

#[derive(Eq, PartialEq)]
pub struct SaveVersion(SystemTime);

impl SaveVersion {
    pub const fn new(time: SystemTime) -> Self {
        Self(time)
    }

    pub const fn time(&self) -> &SystemTime {
        &self.0
    }
}

impl PartialOrd for SaveVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for SaveVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("Under no circumstances SaveVersion::cmp should throw")
    }
}
