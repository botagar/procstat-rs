#![crate_type = "lib"]
#![crate_name = "procstat"]
#[allow(warnings)]

use std::collections::HashMap;
use std::{fs::File, io::{BufRead, BufReader}, path::Path};

mod boot_time;
mod cpu;
mod contexts;
mod interrupts;
mod processes;
mod soft_irq;

use boot_time::*;
use cpu::*;
use contexts::*;
use interrupts::*;
use processes::*;
use soft_irq::*;

#[derive(Debug)]
pub struct ProcStat {
    pub cpu: CPU,
    pub cpus: HashMap<String,CPU>,
    pub interrupts: Interrupts,
    pub contexts: Contexts,
    pub boot_time: BootTime,
    pub processes: Processes,
    pub soft_irq: SoftIrq,
}

const KEY_CPU: &str = "cpu";
const KEY_INTERRUPTS: &str = "intr";
const KEY_CONTEXTS: &str = "ctxt";
const KEY_BOOT_TIME: &str = "btime";
const KEY_SOFT_IRQ: &str = "softirq";

impl ProcStat {
    pub fn read() -> ProcStat {

        let file = File::open(Path::new("/proc/stat")).unwrap();
        
        ProcStat::read_file(file)
    }

    pub fn read_file(file: File) -> ProcStat {

        let mut cpus: HashMap<String,CPU> = HashMap::new();
        let cpu: CPU;
        let mut interrupts: Interrupts = Interrupts::empty();
        let mut contexts: Contexts = Contexts::empty();
        let mut boot_time: BootTime = BootTime::empty();
        let mut soft_irq: SoftIrq = SoftIrq::empty();

        // let mut file_lines: HashMap<String,Vec<String>> = HashMap::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut tokenised = line.split_whitespace();
            let key = tokenised.next().unwrap_or("");
            let values: Vec<u64> = tokenised.by_ref().map(|t| u64::from_str_radix(t, 10).unwrap()).collect();

            if key.starts_with(KEY_CPU) {
                let cpu = CPU::from_vec(values);
                cpus.insert(key.to_string(), cpu);
            } else if key.eq(KEY_INTERRUPTS) {
                interrupts = Interrupts::from_vec(values);
            } else if key.eq(KEY_CONTEXTS) {
                contexts = Contexts::from_vec(values)
            } else if key.eq(KEY_BOOT_TIME) {
                boot_time = BootTime::from_vec(values)
            } else if key.eq(KEY_SOFT_IRQ) {
                soft_irq = SoftIrq::from_vec(values)
            }
        }

        cpu = cpus.remove(KEY_CPU).unwrap();

        let ps = ProcStat {
            cpu,
            cpus,
            interrupts,
            contexts,
            boot_time,
            processes: Processes {
                forks_since_boot: 0,
                running_processes: 0,
                blocked_processes: 0,
            },
            soft_irq
        };

        print!("ps -> {:#?}", ps);

        ps
    }
}


#[cfg(test)]
mod tests {
    use crate::{ProcStat};

    #[test]
    fn it_works() {

        let proc_stat: ProcStat = ProcStat::read();
        
        assert_eq!(proc_stat.cpu.user, 1);
    }
}
