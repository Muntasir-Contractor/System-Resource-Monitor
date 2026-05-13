use sysinfo::{System, RefreshKind, CpuRefreshKind};

fn main(){
    let mut s = System::new_with_specifics(
    RefreshKind::everything().with_cpu(CpuRefreshKind::everything()),
);
/ 
    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again to get actual value.
    s.refresh_all();

    for cpu in s.cpus() {
        println!("{}%", cpu.cpu_usage());
    }
    
}
