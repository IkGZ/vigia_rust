use sysinfo::{System, SystemExt};

pub fn get_ram_usage() -> f32 {
    let mut system = System::new_all();
    system.refresh_all();

    let used_ram = system.get_used_memory() as f32;
    let total_ram = system.get_total_memory() as f32;

    (used_ram / total_ram) * 100.0
}
