use sysinfo::{System, SystemExt};

pub fn get_cpustat(sys: System) {
    // Number of processors:
    println!("NB processors: {}", sys.processors().len());

}