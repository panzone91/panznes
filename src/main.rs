use std::fs;
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::cpu::Cpu;

pub mod cpu;
pub mod bus;
pub mod memory;
pub mod cartridge;

fn get_tick() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Error").as_nanos()
}

fn main() {

    let mut bus = Bus::new();

    let mut file = File::open("/home/panzone/workspace/dk.nes").unwrap();
    let metadata = fs::metadata("/home/panzone/workspace/dk.nes").expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).unwrap();

    let cart = Cartridge::from_ines(&buffer);
    bus.insert_cartridge(&cart);

    let mut cpu = Cpu::new(&mut bus);
    cpu.reset();
    let mut num_clock:u32 = 0;
    const NUM_OP: u32 = 17897725;

    loop {
        let start = SystemTime::now();
        while num_clock <= NUM_OP/60 {
            num_clock += cpu.execute_instruction();
        }
        num_clock -= NUM_OP / 60;
        let end = SystemTime::now();
        //println!("Finished {}", end.duration_since(start).unwrap().as_nanos() / 1000000);
        let delta_t:i128 = 16666666 - (end.duration_since(start).unwrap().as_nanos() as i128);
        if delta_t > 0 {
            //println!("Waiting {}", delta_t);
            sleep(Duration::from_nanos(delta_t as u64));
        };
    }
}
