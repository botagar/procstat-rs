#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use procstat::ProcStat;

    #[test]
    fn aggregated_cpu_details_are_filled_in() {
        let proc_stat: ProcStat = ProcStat::read_file(File::open(Path::new("tests/files/stat.time.1")).unwrap());

        assert_eq!(proc_stat.cpu.user, 58309439);
        assert_eq!(proc_stat.cpu.nice, 4107967);
        assert_eq!(proc_stat.cpu.system, 134201259);
        assert_eq!(proc_stat.cpu.idle, 3696315357);
        assert_eq!(proc_stat.cpu.iowait, 2499873);
        assert_eq!(proc_stat.cpu.irq, 0);
        assert_eq!(proc_stat.cpu.softirq, 408096);
        assert_eq!(proc_stat.cpu.steal, 0);
        assert_eq!(proc_stat.cpu.guest, 0);
        assert_eq!(proc_stat.cpu.guest_nice, 0);
    }

    #[test]
    fn individual_cpu_details_are_filled_in() {
        let proc_stat: ProcStat = ProcStat::read_file(File::open(Path::new("tests/files/stat.time.1")).unwrap());

        assert_eq!(proc_stat.cpus.len(), 24);

        assert_eq!(proc_stat.cpus.get(&0).unwrap().user, 2338176);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().nice, 169382);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().system, 5177739);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().idle, 154355884);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().iowait, 96072);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().irq, 0);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().softirq, 60316);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().steal, 0);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().guest, 0);
        assert_eq!(proc_stat.cpus.get(&0).unwrap().guest_nice, 0);

        assert_eq!(proc_stat.cpus.get(&23).unwrap().user, 2590929);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().nice, 169828);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().system, 5228832);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().idle, 154197362);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().iowait, 123836);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().irq, 0);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().softirq, 1069);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().steal, 0);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().guest, 0);
        assert_eq!(proc_stat.cpus.get(&23).unwrap().guest_nice, 0);
    }

    #[test]
    fn does_not_catastrophically_fail_if_file_contains_unexpected_characters() {
        let proc_stat: ProcStat = ProcStat::read_file(File::open(Path::new("tests/files/stat.time.2")).unwrap());

        assert_eq!(proc_stat.cpu.user, 58309530);
        assert_eq!(proc_stat.cpu.nice, 4107967);
        assert_eq!(proc_stat.cpu.system, 134201778);
        assert_eq!(proc_stat.cpu.idle, 3696322868);
        assert_eq!(proc_stat.cpu.iowait, 2499878);
        assert_eq!(proc_stat.cpu.irq, 0);
        assert_eq!(proc_stat.cpu.softirq, 0);
        assert_eq!(proc_stat.cpu.steal, 0);
        assert_eq!(proc_stat.cpu.guest, 0);
        assert_eq!(proc_stat.cpu.guest_nice, 0);
    }
}
