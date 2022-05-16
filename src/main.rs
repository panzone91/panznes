use crate::cartridge::Cartridge;
use crate::nes::Nes;
use sdl2::event::Event;
use sdl2::libc::exit;
use sdl2::pixels::{Color, PixelFormat};
use sdl2::rect::Rect;
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
    let path = "/home/panzone/Downloads/ice_climbers.nes";
    let mut file = File::open(path).unwrap();
    let metadata = fs::metadata(path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let scaling = 3;

    let window = video_subsystem
        .window("panznes", 256 * scaling, 240 * scaling)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let cart = Cartridge::from_ines(&buffer);

    let mut nes = Nes::create_nes();
    nes.insert_cartdrige(&cart);

    nes.reset();
    let mut num_clock: u32 = 0;
    const NUM_OP: u32 = 17897725;
    let mut execute = true;

    while execute == true {
        let start = SystemTime::now();
        while num_clock <= NUM_OP / 60 {
            let cpu_clock = nes.execute_instruction();
            nes.execute_ppu(cpu_clock);
            num_clock += cpu_clock;
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let mut event_pump = sdl_context.event_pump().unwrap();

        for i in 0..240 {
            for k in 0..256 {
                let color = nes.screen[(256 * i) + k];

                let pixel = Rect::new(
                    (k * scaling as usize) as i32,
                    (i * scaling as usize) as i32,
                    scaling,
                    scaling,
                );
                let color = Color::RGBA(
                    ((color >> 24) & 0xFF) as u8,
                    ((color >> 16) & 0xFF) as u8,
                    ((color >> 8) & 0xFF) as u8,
                    0,
                );
                canvas.set_draw_color(color);
                canvas.fill_rect(pixel);
            }
        }
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    execute = false;
                }
                _ => {}
            }
        }

        num_clock -= NUM_OP / 60;
        let end = SystemTime::now();
        /*println!(
            "Finished {}",
            end.duration_since(start).unwrap().as_nanos() / 1000000
        );*/
        let delta_t: i128 = 16666666 - (end.duration_since(start).unwrap().as_nanos() as i128);
        if delta_t > 0 {
            //println!("Waiting {}", delta_t);
            sleep(Duration::from_nanos(delta_t as u64));
        };
    }
}
