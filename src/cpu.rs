#[derive(Debug)]
pub struct CPU {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
    pub guest_nice: u64,
}

impl CPU {
    pub fn empty() -> CPU {
        CPU {
            user: 0,
            nice: 0,
            system: 0,
            idle: 0,
            iowait: 0,
            irq: 0,
            softirq: 0,
            steal: 0,
            guest: 0,
            guest_nice: 0,
        }
    }

    pub fn from_vec(cpu_vals: Vec<u64>) -> CPU {
        CPU {
            user: unwrap_or_0(cpu_vals.get(0)),
            nice: unwrap_or_0(cpu_vals.get(1)),
            system: unwrap_or_0(cpu_vals.get(2)),
            idle: unwrap_or_0(cpu_vals.get(3)),
            iowait: unwrap_or_0(cpu_vals.get(4)),
            irq: unwrap_or_0(cpu_vals.get(5)),
            softirq: unwrap_or_0(cpu_vals.get(6)),
            steal: unwrap_or_0(cpu_vals.get(7)),
            guest: unwrap_or_0(cpu_vals.get(8)),
            guest_nice: unwrap_or_0(cpu_vals.get(9)),
        }
    }
}

fn unwrap_or_0(wrapped: Option<&u64>) -> u64 {
    match wrapped {
        Some(value) => *value,
        None => 0,
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    #[test]
    fn given_10_columns_it_populates_all_fields() {
        let test_data: Vec<u64> = vec![46044323, 3582636, 101626186, 3138393496, 2100903, 1, 224268, 2, 3, 4];
        let cpu: CPU = CPU::from_vec(test_data);
        
        assert_eq!(cpu.user, 46044323);
        assert_eq!(cpu.nice, 3582636);
        assert_eq!(cpu.system, 101626186);
        assert_eq!(cpu.idle, 3138393496);
        assert_eq!(cpu.iowait, 2100903);
        assert_eq!(cpu.irq, 1);
        assert_eq!(cpu.softirq, 224268);
        assert_eq!(cpu.steal, 2);
        assert_eq!(cpu.guest, 3);
        assert_eq!(cpu.guest_nice, 4);
    }

    #[test]
    fn given_6_columns_it_populates_remaining_fields_with_0() {
        let test_data: Vec<u64> = vec![46044323, 3582636, 101626186, 3138393496, 2100903, 1];
        let cpu: CPU = CPU::from_vec(test_data);
        
        assert_eq!(cpu.user, 46044323);
        assert_eq!(cpu.nice, 3582636);
        assert_eq!(cpu.system, 101626186);
        assert_eq!(cpu.idle, 3138393496);
        assert_eq!(cpu.iowait, 2100903);
        assert_eq!(cpu.irq, 1);
        assert_eq!(cpu.softirq, 0);
        assert_eq!(cpu.steal, 0);
        assert_eq!(cpu.guest, 0);
        assert_eq!(cpu.guest_nice, 0);
    }

    #[test]
    fn returns_back_empty_cpu_if_vec_is_empty() {
        let cpu: CPU = CPU::from_vec(Vec::new());
        
        assert_eq!(cpu.user, 0);
        assert_eq!(cpu.nice, 0);
        assert_eq!(cpu.system, 0);
        assert_eq!(cpu.idle, 0);
        assert_eq!(cpu.iowait, 0);
        assert_eq!(cpu.irq, 0);
        assert_eq!(cpu.softirq, 0);
        assert_eq!(cpu.steal, 0);
        assert_eq!(cpu.guest, 0);
        assert_eq!(cpu.guest_nice, 0);
    }
}
