mod cpu;
mod mail;
mod ram;
mod read_file;

use crate::mail::send_mail;
use std::path::PathBuf;

fn main() {
    let file_path: PathBuf = PathBuf::from("src/config.yml");
    let config = read_file::read_config(file_path).unwrap();
    // println!("{:?}", config);

    if config.cpu.enabled {
        let cpu_usages = cpu::get_cpu_usage(5);
        let avg_cpu_usage: f32 = cpu_usages.iter().sum::<f32>() / cpu_usages.len() as f32;
        println!("Uso de CPU promedio: {:.2}%", avg_cpu_usage);
    }

    if config.ram.enabled {
        let ram_usage = ram::get_ram_usage();
        println!("RAM usage: {:.2}%", ram_usage);
    }

    if ram_usage > config.ram.threshold {
        let to = "recipient@example.com";
        let subject = "Test email";
        let body = "This is a test email sent from Rust!";

        match mail::send_mail(to, subject, body) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => println!("Error sending email: {:?}", e),
        }
    }
}
