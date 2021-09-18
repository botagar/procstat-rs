# ProcStat

A simple parser for the `/proc/stat` file found on all unix based systems.

Specifications for this file can be found on [Random Sites Like This](https://man7.org/linux/man-pages/man5/proc.5.html).

## How to use

Import it into your `cargo.toml` as a dependency.

```
[dependencies]
procstat = "1.0.3"
```

Add it to the entry point of your project

```
extern crate procstat;
use procstat::ProcStat;
```

Then call it

```
...
let proc_stat = ProcStat::read();
let aggregated_cpu_data = proc_stat.cpu;
...
```

## Notes from the author

Code review welcome. 

I'm not a full time rust developer by any means so always willing to listen to those who do this more than I.

Stay safe and be good to each other!