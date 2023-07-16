extern crate systemstat;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

fn main() {
    let sys = System::new();

    match sys.memory() {
        Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => println!("\nMemory: error: {}", x)
    }

    match sys.cpu_load() {
        Ok(cpu)=> {
            println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
        },
        Err(x) => println!("\nCPU load: error: {}", x)
    }
}
