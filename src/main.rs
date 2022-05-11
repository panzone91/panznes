use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::bus::Bus;
use crate::cpu::Cpu;

pub mod cpu;

pub mod bus;
pub mod memory;

fn get_tick() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Error").as_millis()
}

fn main() {

    let bus = Bus::new();
    let mut cpu = Cpu::new(bus);
    let mut num_clock:u32 = 0;
    const NUM_OP: u32 = 17897725;

    loop {
        let start = get_tick();
        while num_clock <= NUM_OP {
            num_clock += cpu.execute_instruction();
        }
        num_clock -= NUM_OP;
        let delta_t:f64 = 16.6 - f64::from((get_tick() - start) as u32);
        println!("Finished {}", (get_tick() - start) as u32);
        if delta_t > 0.0 {
            println!("Waiting {}", delta_t);
            sleep(Duration::from_millis(delta_t as u64))
        };
    }
}
