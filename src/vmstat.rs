use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};



pub fn get_vmstat(sys: System) {
    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("total swap  : {} KB", sys.total_swap());
    println!("used swap   : {} KB", sys.used_swap());
}