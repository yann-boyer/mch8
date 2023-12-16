extern crate sdl2;

use mch8::virtual_machine::VirtualMachine;
use mch8::globals::*;
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("[Error] No input file provided !");
        eprintln!("[Info] Usage : ./mch8 <chip8.rom>");
        std::process::exit(1);
    }

    let rom_path = &args[1];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _audio = sdl_context.audio().unwrap();
    let _ = sdl2::mixer::init(InitFlag::all()).unwrap();
    sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 2048).unwrap();

    let mut virtual_machine = VirtualMachine::new();

    let r = virtual_machine.load_rom(rom_path);
    if r.is_err() {
        let err_message = r.err().unwrap();
        println!("{}", err_message);
        std::process::exit(1);
    }


    virtual_machine.init_audio();

    let window = video_subsystem.window("MCH8 by Yann BOYER", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut is_running = true;

    let mut div_cycles: u8 = 0;

    while is_running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    println!("[Info] Exiting...");
                    is_running = false;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => virtual_machine.set_key(0x1, true),
                        Keycode::Num2 => virtual_machine.set_key(0x2, true),
                        Keycode::Num3 => virtual_machine.set_key(0x3, true),
                        Keycode::Num4 => virtual_machine.set_key(0xC, true),
                        Keycode::Q => virtual_machine.set_key(0x4, true),
                        Keycode::W => virtual_machine.set_key(0x5, true),
                        Keycode::E => virtual_machine.set_key(0x6, true),
                        Keycode::R => virtual_machine.set_key(0xD, true),
                        Keycode::A => virtual_machine.set_key(0x7, true),
                        Keycode::S => virtual_machine.set_key(0x8, true),
                        Keycode::D => virtual_machine.set_key(0x9, true),
                        Keycode::F => virtual_machine.set_key(0xE, true),
                        Keycode::Z => virtual_machine.set_key(0xA, true),
                        Keycode::X => virtual_machine.set_key(0x0, true),
                        Keycode::C => virtual_machine.set_key(0xB, true),
                        Keycode::V => virtual_machine.set_key(0xF, true),
                        _ => ()
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => virtual_machine.set_key(0x1, false),
                        Keycode::Num2 => virtual_machine.set_key(0x2, false),
                        Keycode::Num3 => virtual_machine.set_key(0x3, false),
                        Keycode::Num4 => virtual_machine.set_key(0xC, false),
                        Keycode::Q => virtual_machine.set_key(0x4, false),
                        Keycode::W => virtual_machine.set_key(0x5, false),
                        Keycode::E => virtual_machine.set_key(0x6, false),
                        Keycode::R => virtual_machine.set_key(0xD, false),
                        Keycode::A => virtual_machine.set_key(0x7, false),
                        Keycode::S => virtual_machine.set_key(0x8, false),
                        Keycode::D => virtual_machine.set_key(0x9, false),
                        Keycode::F => virtual_machine.set_key(0xE, false),
                        Keycode::Z => virtual_machine.set_key(0xA, false),
                        Keycode::X => virtual_machine.set_key(0x0, false),
                        Keycode::C => virtual_machine.set_key(0xB, false),
                        Keycode::V => virtual_machine.set_key(0xF, false),
                        _ => ()
                    }
                },
                _ => {}
            }
        }

        virtual_machine.execute_processor_instruction();
        div_cycles += 1;

        if virtual_machine.screen_need_repaint() {
            for y in 0..CHIP8_SCREEN_HEIGHT {
                for x in 0..CHIP8_SCREEN_WIDTH {
                    let pixel = Rect::new(x as i32 * SCALE_FACTOR as i32, y as i32 * SCALE_FACTOR as i32, SCALE_FACTOR as u32, SCALE_FACTOR as u32);
                    if virtual_machine.is_pixel_switched_on(x, y) {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                    } else {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                    }

                    canvas.fill_rect(pixel).unwrap();
                }
            }

            canvas.present();
            canvas.clear();
            virtual_machine.disable_repaint();
        }

        if div_cycles == TIMER_DIVISION_CLOCK {
            virtual_machine.update_processor_timers();
            div_cycles = 0;
        }

        std::thread::sleep(Duration::from_micros(CPU_CLOCK_DELAY as u64));
    }
}
