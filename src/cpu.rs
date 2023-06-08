use std::thread;
use std::time::Duration;
use sysinfo::{ProcessorExt, System, SystemExt};

pub fn get_cpu_usage(duration: u64) -> Vec<f32> {
    let mut system = System::new_all();
    system.refresh_all();
    // let cpu = system.get_global_processor_info();

    let mut cpu_usages = Vec::new();
    let mut elapsed_time = 0;

    while elapsed_time < duration {
        let mut new_system = System::new_all();
        new_system.refresh_all();
        let new_cpu = new_system.get_global_processor_info();
        let usage = new_cpu.get_cpu_usage();
        cpu_usages.push(usage);
        thread::sleep(Duration::from_secs(1));
        elapsed_time += 1;
    }

    system.refresh_all();
    cpu_usages
}
