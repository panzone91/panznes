extern crate core;

use crate::cartridge::Cartridge;
use crate::nes::{Nes, NesControllerButton};
use sdl2::event::Event;
use sdl2::libc::exit;
use sdl2::pixels::{Color, PixelFormat};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::ops::{Div, Sub};
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, fs};
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use crate::nes::NesControllerButton::START;
use crate::NesControllerButton::{A, B, DOWN, LEFT, RIGHT, SELECT, UP};

pub mod cartridge;
pub mod memory;

pub mod nes;

fn get_tick() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error")
        .as_nanos()
}

fn draw_screen(canvas: &mut WindowCanvas, screen_pixels_rgba: &[u32; 256 * 240]) {
    let scaling = 3;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    for i in 0..240 {
        for k in 0..256 {
            let color = screen_pixels_rgba[(256 * i) + k];

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
            canvas.fill_rect(pixel).expect("Error writing pixel");
        }
    }
    canvas.present();
}

fn convert_keycode_to_nes (key: Option<Keycode>) -> Option<NesControllerButton>{
    match key {
        Some(Keycode::Up) => Some(UP),
        Some(Keycode::Down) => Some(DOWN),
        Some(Keycode::Left) => Some(LEFT),
        Some(Keycode::Right) => Some(RIGHT),
        Some(Keycode::Z) => Some(A),
        Some(Keycode::X) => Some(B),
        Some(Keycode::Return) => Some(START),
        Some(Keycode::Backspace) => Some(SELECT),
        Some(_) => None,
        _ => None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args.get(1).expect("Missing ROM filename");

    let mut file = File::open(path).expect("Cannot open ROM file");
    let metadata = fs::metadata(path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Error reading rom");

    let sdl_context = sdl2::init().expect("Error init SDL2");
    let video_subsystem = sdl_context.video().expect("Error init SDL2 video");

    let scaling = 3;

    let window = video_subsystem
        .window("panznes", 256 * scaling, 240 * scaling)
        .position_centered()
        .build()
        .expect("Error init window");

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let cart = Cartridge::from_ines(&buffer);

    let mut nes = Nes::create_nes();
    nes.insert_cartdrige(&cart);

    nes.reset();

    let mut num_clock: u32 = 0;
    const NUM_OP: u32 = 1789773;
    let mut execute = true;

    while execute == true {
        let start = SystemTime::now();
        while num_clock <= NUM_OP / 60 {
            let cpu_clock = nes.execute_instruction();
            nes.execute_ppu(cpu_clock);
            num_clock += cpu_clock;
        }

        draw_screen(&mut canvas, &nes.screen);

        let mut event_pump = sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    execute = false;
                },
                Event::KeyUp { keycode, .. } => {
                    let nes_button = convert_keycode_to_nes(keycode);
                    if nes_button.is_some() {nes.set_controller_status(nes_button.unwrap(), false)}
                },
                Event::KeyDown { keycode, .. } => {
                    let nes_button = convert_keycode_to_nes(keycode);
                    if nes_button.is_some() {nes.set_controller_status(nes_button.unwrap(), true)}
                },
                _ => {}
            }
        }

        num_clock -= NUM_OP / 60;
        let end = SystemTime::now()
            .duration_since(start)
            .expect("Frame duration negative");
        println!("Finished {}", end.as_secs_f64());
        let delta_t = Duration::from_secs_f64(f64::from(1).div(f64::from(60)))
            .checked_sub(end)
            .unwrap_or_default();
        //println!("DeltaT {}", delta_t.as_secs_f64());
        if delta_t.as_millis() > 0 {
            //println!("Waiting {}", delta_t.as_millis());
            sleep(delta_t);
        };
    }
}
