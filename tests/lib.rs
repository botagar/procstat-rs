#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use procstat::ProcStat;


    #[test]
    fn given_something() {
        let procstat: ProcStat = ProcStat::read_file(File::open(Path::new("tests/files/stat.time.1")).unwrap());
    }
}
