#![crate_type = "lib"]
#![crate_name = "procstat"]

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
    pub cpus: HashMap<usize,CPU>,
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
const KEY_PROCESSES: &str = "processes";
const KEY_PROCESSES_RUNNING: &str = "procs_running";
const KEY_PROCESSES_BLOCKED: &str = "procs_blocked";

impl ProcStat {
    fn empty() -> Self {
        Self {
            cpu: CPU::empty(),
            cpus: HashMap::new(),
            interrupts: Interrupts::empty(),
            contexts: Contexts::empty(),
            boot_time: BootTime::empty(),
            processes: Processes::empty(),
            soft_irq: SoftIrq::empty(),
        }
    }

    pub fn read() -> ProcStat {

        let file = File::open(Path::new("/proc/stat")).unwrap();
        
        ProcStat::read_file(file)
    }

    pub fn read_file(file: File) -> ProcStat {
        let mut proc_stat = Self::empty();

        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut tokenised = line.split_whitespace();
            let key = tokenised.next().unwrap_or("");
            let values: Vec<u64> = tokenised.by_ref().map(|t| u64::from_str_radix(t, 10).unwrap_or_default()).collect();

            if key.eq(KEY_CPU) {
                proc_stat.cpu = CPU::from_vec(values);
            } else if key.starts_with(KEY_CPU) {
                let cpu = CPU::from_vec(values);
                let cpu_index = usize::from_str_radix(key.split_at(3).1, 10).unwrap();
                proc_stat.cpus.insert(cpu_index, cpu);
            } else if key.eq(KEY_INTERRUPTS) {
                proc_stat.interrupts = Interrupts::from_vec(values);
            } else if key.eq(KEY_CONTEXTS) {
                proc_stat.contexts = Contexts::from_vec(values);
            } else if key.eq(KEY_BOOT_TIME) {
                proc_stat.boot_time = BootTime::from_vec(values);
            } else if key.eq(KEY_SOFT_IRQ) {
                proc_stat.soft_irq = SoftIrq::from_vec(values);
            } else if key.eq(KEY_PROCESSES) {
                proc_stat.processes.forks_since_boot = *values.first().unwrap();
            } else if key.eq(KEY_PROCESSES_RUNNING) {
                proc_stat.processes.running_processes = *values.first().unwrap();
            } else if key.eq(KEY_PROCESSES_BLOCKED) {
                proc_stat.processes.blocked_processes = *values.first().unwrap();
            }
        }

        proc_stat
    }
}
