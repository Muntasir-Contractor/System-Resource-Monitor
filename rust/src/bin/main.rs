use sysinfo::{System, RefreshKind, CpuRefreshKind, Disks};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string};
use std::error::Error;
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};

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
    gpu_resource: GPU_Resources
}

impl Resources{
    fn to_string(&self) -> String{
        let res = format!("cpu_usage: {:?},\ndisk_usage: {},\ntotal_disk: {},\nmemory_usage: {},\ntotal_memory: {} ", self.cpu_usage,self.disk_usage,self.total_disk,self.memory_usage,self.total_memory);
        res
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct GPU_Resources{
    // Clock speed , GPU utilization %, VRAM total and VRAM used, running compute processes, GPU model/name and compute capability, 
    // Power draw and power limit, GPU model/name and compute capability
    device_brand: Option<String>,
    fan_speed: Option<f64>,
    power_limit: Option<f64>,
    encoder_util: Option<f64>,
    memory_info: Option<f64>
}

fn print_resources(resources: Res){
    match resources{
        Res::JSON(Ok(resources)) => println!("{}",resources),
        Res::JSON(Err(e)) => println!("Error: {}",e),
        Res::Default(resources) => println!("{}", resources)
    }
}

enum Res{
    JSON(Result<String,serde_json::Error>),
    Default(String)
}

fn has_gpu() -> bool {
    let nvml = Nvml::init();

    match nvml {
        Ok(_) => return true,
        Err(_error) => return false 
    }
}

fn poll_resources(json_type : bool) -> Res {
    let mut sys = System::new_all();
    sys.refresh_all();

    
    let mut cpu_usages: Vec<f64> = Vec::new();
    for cpu in sys.cpus(){
        cpu_usages.push(cpu.cpu_usage() as f64);
    }
    let disks = Disks::new_with_refreshed_list();
    let mut disk_usage_: f64 = 1.0;
    let mut disk_space_: f64 = 1.0;
    for disk in &disks{
        disk_space_ = disk.total_space() as f64 / 1000000000.0;
        disk_usage_ = disk_space_ - (disk.available_space() as f64 / 1000000000.0);
    }

    



    let memory_usage_: f64 = sys.used_memory() as f64 / 1000000000.0;
    let memory_space: f64 = sys.total_memory() as f64 / 1000000000.0;
    let mut result: Resources = Resources {
        cpu_usage: cpu_usages,
        disk_usage: disk_usage_,
        total_disk: disk_space_,
        memory_usage: memory_usage_,
        total_memory: memory_space,
        gpu_resource: GPU_Resources{
            device_brand: None,
            fan_speed: None,
            power_limit: None,
            encoder_util: None,
            memory_info: None
        }};

    let has_GPU: bool = has_gpu();
    
        

    if json_type == true{
        if has_GPU{
            println!("Has GPU")
        }
        else{
            println!("Does not have GPU")
        }
        let json_res = serde_json::to_string(&result);
        return Res::JSON(Ok(json_res.expect("Error")))
    }
    else{
        println!("{}",result.to_string());
        return Res::Default(result.to_string());
    }
}

