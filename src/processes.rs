#[derive(Debug)]
pub struct Processes {
    pub forks_since_boot: u64,
    pub running_processes: u64,
    pub blocked_processes: u64,
}

impl Processes {
    pub fn empty() -> Self {
        Self {
            forks_since_boot: 0,
            running_processes: 0,
            blocked_processes: 0,
        }
    }
}
