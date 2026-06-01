use sysinfo::{System, RefreshKind, CpuRefreshKind, Disks};
use serde::{Serialize, Deserialize};
use serde_json::from_str;

fn main(){
    let mut sys = System::new_all();
    let mut s = System::new_with_specifics(
    RefreshKind::everything().with_cpu(CpuRefreshKind::everything()),
);

    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    let mut used_memory: f64 = sys.used_memory() as f64;
    used_memory = used_memory/1000000000 as f64;
    println!("Total memory usage: {:.2}GB",used_memory);

    for cpu in sys.cpus() {
        println!("{}%", cpu.cpu_usage());
    }

    loop {
        sys.refresh_all();
        println!(" -- CPU USAGE ------------------------------------");
        for (i,cpu) in sys.cpus().iter().enumerate(){
            println!("CPU #{}: {}%", i, cpu.cpu_usage());

        }

        println!("-------------------------------------------------------");

        println!(" -- MEMORY USAGE -------------------------------------");
        println!("{:.2}GB/{:.2}GB", (sys.used_memory() as f64/1000000000.0) as f64, (sys.total_memory() as f64/1000000000.0) as f64);

        println!("-------------------------------------------------------");

        println!("-- DISK USAGE ------------------------------------------");

        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("{:?}: {:.2}GB / {:.2}GB", disk.name(), (disk.total_space()-disk.available_space()) as f64 / 1000000000.0,disk.total_space() as f64 / 1000000000.0);
        }



        std::thread::sleep(std::time::Duration::from_secs(1));

    }

    // loop {
       // s.refresh_all();
        //sys.refresh_all();


    //}
    
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Resources{
    cpu_usage: Vec<f64>,
    disk_usage: f64,
    total_disk: i64,
    memory_usage: f64,
    total_memory: i64,
}

fn poll_resources(json_type : bool) -> Result<Resources, Error, String> {
    let mut res = String::from("r# { ");
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut cpu_usages: Vec<f64> = Vec::new();
    for cpu in sys.cpus(){
        cpus_usages.push(cpu.cpu_usage);
    }

    res.push("cpu_usage: {:?},", cpu_usages)
    let disks = Disks::new_with_refreshed_list();
    let mut disk_usage: f64;
    let mut disk_space: f64;
    for disk in &disks{
        disk_space = disk.total_space() as f64 / 1000000000.0;
        disk_usage = disk_space - (disc.available_space as f64 / 1000000000.0);
    }



    res.push_str("disk_usage: {:.2},", disk_usage);
    res.push_str("total_disk: {:.2}," disk_space);

    const memory_usage: f64 = sys.used_memory() as f64 / 1000000000.0;
    const memory_space: f64 = sys.total_memory() as f64 / 1000000000.0;
    res.push_str("memory_usage: {:.2},",memory_usage);
    res.push_str("total_memory: {:.2}", memory_space);

    if json_type{
        let json_res = from_str::<Resources>(&res);
        return json_res
    }
    else{
        println!("{}",res);
        return res;
    }
}

