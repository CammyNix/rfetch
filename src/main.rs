fn main() {
    use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, CpuExt};
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.refresh_cpu(); // Refreshing CPU information.
    for cpu in sys.cpus() {
    println!("CPU: {}", cpu.brand());
    }
    println!("MEM: {}", sys.total_memory());
    println!("OS: {:?}", sys.name());
    
}
