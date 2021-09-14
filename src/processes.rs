#[derive(Debug)]
pub struct Processes {
    pub forks_since_boot: u64,
    pub running_processes: u64,
    pub blocked_processes: u64,
}

impl Processes {
    pub fn new() -> Self {
        Self {
            forks_since_boot: 0,
            running_processes: 0,
            blocked_processes: 0,
        }
    }

    pub fn from_string(line_in_file: String) -> Self {
        if !line_in_file.starts_with("btime") { return Self::new() }

        let mut tokens = line_in_file.split_whitespace();

        let context_switches = u64::from_str_radix(tokens.nth(1).unwrap(), 10).unwrap();

        Self {
            forks_since_boot: context_switches,
            running_processes: context_switches,
            blocked_processes: context_switches,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::processes::Processes;

    #[test]
    fn creates_context_from_string() {
        let boot_time: Processes = Processes::from_string(String::from_str("btime 1628859739").unwrap());
        
        assert_eq!(boot_time.forks_since_boot, 1628859739);
    }

    #[test]
    fn returns_default_if_string_is_malformed() {
        let boot_time: Processes = Processes::from_string(String::from_str("Not a boot time string").unwrap());
        
        assert_eq!(boot_time.forks_since_boot, 0);
    }
}