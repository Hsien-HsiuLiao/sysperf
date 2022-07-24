use sysinfo::{System, SystemExt, NetworkExt};

pub fn get_netstat(sys: System) {


    // Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }
}