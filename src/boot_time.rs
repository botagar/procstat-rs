#[derive(Debug)]
pub struct BootTime {
    pub total_run_time: u64,
}

impl BootTime {
    pub fn empty() -> Self { Self { total_run_time: 0 } }

    pub fn from_vec(bt_val: Vec<u64>) -> Self {
        if bt_val.len() == 0 { return Self::empty() }
        
        Self {
            total_run_time: *bt_val.first().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::boot_time::BootTime;

    #[test]
    fn creates_context_from_vector() {
        let test_data: Vec<u64> = vec![1628859739];
        let boot_time: BootTime = BootTime::from_vec(test_data);
        
        assert_eq!(boot_time.total_run_time, 1628859739);
    }

    #[test]
    fn returns_default_if_vector_is_empty() {
        let boot_time: BootTime = BootTime::from_vec(Vec::new());
        
        assert_eq!(boot_time.total_run_time, 0);
    }
}