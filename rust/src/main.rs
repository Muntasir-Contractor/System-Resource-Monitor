use sysinfo::{System, RefreshKind, CpuRefreshKind, Disks};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use std::error::Error;

fn main(){
    let rres: Res = poll_resources(true);
    print_resources(rres);
    std::thread::sleep(Duration::from_secs(10));

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

#[derive(Deserialize, Serialize, Debug)]
struct Resources{
    cpu_usage: Vec<f64>,
    disk_usage: f64,
    total_disk: f64,
    memory_usage: f64,
    total_memory: f64,
}

fn print_resources(resources: Res){
    match resources{
        Res::JSON(Ok(resources)) => println!("{}", serde_json::to_string(&resources).unwrap()),
        Res::JSON(Err(e)) => println!("Error: {}",e),
        Res::Default(resources) => println!("{}", resources)
    }
}

enum Res{
    JSON(Result<Resources,serde_json::Error>),
    Default(String)
}

fn poll_resources(json_type : bool) -> Res {
    let mut res = String::from("{ ");
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut cpu_usages: Vec<f64> = Vec::new();
    for cpu in sys.cpus(){
        cpu_usages.push(cpu.cpu_usage() as f64);
    }

    res.push_str(&format!("\"cpu_usage\": {:?},", cpu_usages));
    let disks = Disks::new_with_refreshed_list();
    let mut disk_usage: f64;
    let mut disk_space: f64;
    for disk in &disks{
        disk_space = disk.total_space() as f64 / 1000000000.0;
        disk_usage = disk_space - (disk.available_space() as f64 / 1000000000.0);
        res.push_str(&format!("\"disk_usage\": {:.2},", disk_usage));
        res.push_str(&format!("\"total_disk\": {:.2},", disk_space));
    }


    let memory_usage: f64 = sys.used_memory() as f64 / 1000000000.0;
    let memory_space: f64 = sys.total_memory() as f64 / 1000000000.0;
    res.push_str(&format!("\"memory_usage\": {:.2},",memory_usage));
    res.push_str(&format!("\"total_memory\": {:.2}", memory_space));
    res.push_str("}");

    if json_type == true{
        let json_res = from_str::<Resources>(&res);
        return Res::JSON(json_res)
    }
    else{
        println!("{}",res);
        return Res::Default(res);
    }
}

