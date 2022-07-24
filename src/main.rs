use std::io;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use sysperf::vmstat::get_vmstat;
use sysperf::iostat::get_iostat;
use sysperf::cpustat::get_cpustat;
use sysperf::netstat::get_netstat;

fn main() {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    

    // Components temperature:
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    
    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        println!(
            "[{0: <10}] PROCESS NAME: {1: <15} {2:?}",
            pid,
            process.name(),
            process.disk_usage()
        );
    }

    println!("Enter a command. (ex: vmstat)");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("failed to read line");

    match input.trim() {
        "vmstat" => get_vmstat(sys),
        "iostat" => get_iostat(sys),
        "cpustat" => get_cpustat(sys),
        "netstat" => get_netstat(sys),
        
        _ => println!("_ {}", input.as_str())
    }


}
