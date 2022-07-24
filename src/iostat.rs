use sysinfo::{System, SystemExt};

pub fn get_iostat(sys: System) {
    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }
}