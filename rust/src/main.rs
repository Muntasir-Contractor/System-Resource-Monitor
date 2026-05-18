use sysinfo::{System, RefreshKind, CpuRefreshKind, Disks};

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

