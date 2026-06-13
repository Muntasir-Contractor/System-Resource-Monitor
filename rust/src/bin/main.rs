use sysinfo::{System, RefreshKind, CpuRefreshKind, Disks};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string};
use std::error::Error;
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};
use nvml_wrapper::Device;

fn main(){
    loop {
        let res = poll_resources(true);
        print_resources(&res);
        std::thread::sleep(Duration::from_secs(5));
    }
    
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
        let res = format!("cpu_usage: {:?},\ndisk_usage: {},\ntotal_disk: {},\nmemory_usage: {},\ntotal_memory: {}, gpu_resources: {:?} ", self.cpu_usage,self.disk_usage,self.total_disk,self.memory_usage,self.total_memory,self.gpu_resource);
        res
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct GPU_Resources{
    // Clock speed , GPU utilization %, VRAM total and VRAM used, running compute processes, GPU model/name and compute capability, 
    // Power draw and power limit, GPU model/name and compute capability
    device_brand: Option<String>,
    architecture: Option<String>,
    vram_total: Option<u64>,
    vram_used: Option<u64>,
    vram_free: Option<u64>,
    gpu_utilization: Option<u32>,
    temperature: Option<u32>,
    power_limit: Option<u32>,
    power_draw: Option<u32>,
    compute_processes: Option<u32>
}

fn print_resources(resources: &Res){
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

fn poll_gpu_resources(device: &Device) -> GPU_Resources{
    let mem_info = device.memory_info();

    let res = GPU_Resources {
        // use device.name.ok() and so on and sofourth to turn the Result<T,E> into Option<T>
        // use mem_info.as_ref().ok().map(|m| m.used) so on and so fourth
        device_brand: device.name().ok(),
        architecture: device.architecture().ok().map(|y| format!("{:?}",y)),
        vram_total: mem_info.as_ref().ok().map(|m| m.total),
        vram_used: mem_info.as_ref().ok().map(|m| m.used),
        vram_free: mem_info.as_ref().ok().map(|m| m.free),
        gpu_utilization: device.utilization_rates().ok().map(|x| x.gpu),
        temperature: device.temperature(TemperatureSensor::Gpu).ok(),
        power_limit: device.power_management_limit().ok(),
        power_draw: device.power_usage().ok(),
        compute_processes: device.running_compute_processes().ok().map(|p| p.len() as u32)

    };
    res


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
            architecture: None,
            vram_total: None,
            vram_used: None,
            vram_free: None,
            gpu_utilization: None,
            temperature: None,
            power_limit: None,
            power_draw: None,
            compute_processes: None
        }};

    let has_GPU: bool = has_gpu();
    
        

    if json_type == true{
        let mut gpu_res: GPU_Resources;
        if has_GPU{
            let nvml = Nvml::init().unwrap();
            let device = nvml.device_by_index(0).unwrap();
            gpu_res = poll_gpu_resources(&device);
            result.gpu_resource = gpu_res;
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

