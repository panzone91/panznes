use crate::cartridge::Cartridge;
use crate::nes::Nes;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod cartridge;
pub mod memory;

pub mod nes;

fn get_tick() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error")
        .as_nanos()
}

fn main() {
    let mut file = File::open("/home/panzone/workspace/dk.nes").unwrap();
    let metadata = fs::metadata("/home/panzone/workspace/dk.nes").expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).unwrap();

    let cart = Cartridge::from_ines(&buffer);

    let mut nes = Nes::create_nes();
    nes.insert_cartdrige(&cart);

    nes.reset();
    let mut num_clock: u32 = 0;
    const NUM_OP: u32 = 17897725;

    loop {
        let start = SystemTime::now();
        while num_clock <= NUM_OP / 60 {
            num_clock += nes.execute_instruction();
            nes.execute_ppu(num_clock);
        }
        num_clock -= NUM_OP / 60;
        let end = SystemTime::now();
        //println!("Finished {}", end.duration_since(start).unwrap().as_nanos() / 1000000);
        let delta_t: i128 = 16666666 - (end.duration_since(start).unwrap().as_nanos() as i128);
        if delta_t > 0 {
            //println!("Waiting {}", delta_t);
            sleep(Duration::from_nanos(delta_t as u64));
        };
    }
}
