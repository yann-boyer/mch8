use crate::globals::*;

#[derive(PartialEq)]
pub enum PixelState {
    SwitchedOn,
    SwitchedOff,
}

const TOTAL_RENDER_TABLE_SIZE: u16 = CHIP8_SCREEN_WIDTH as u16 * CHIP8_SCREEN_HEIGHT as u16;

pub struct RenderTable {
    render_table: [bool; TOTAL_RENDER_TABLE_SIZE as usize],
}

impl RenderTable {
    pub fn new() -> RenderTable {
        RenderTable {
            render_table: [false; TOTAL_RENDER_TABLE_SIZE as usize],
        }
    }

    pub fn is_pixel_switched_on(&self, x: u8, y: u8) -> bool {
        if x > CHIP8_SCREEN_WIDTH || y > CHIP8_SCREEN_HEIGHT {
            println!("[Warning] Invalid coordinates -> X : {} Y : {}", x, y);
            return false;
        }

        self.render_table[y as usize * CHIP8_SCREEN_WIDTH as usize + x as usize]
    }

    pub fn change_pixel_state_to(&mut self, x: u8, y: u8, new_state: PixelState) {
        if x > CHIP8_SCREEN_WIDTH || y > CHIP8_SCREEN_HEIGHT {
            println!("[Warning] Invalid coordinates -> X : {} Y : {}", x, y);
            return;
        }

        let state: bool = new_state == PixelState::SwitchedOn;

        self.render_table[y as usize * CHIP8_SCREEN_WIDTH as usize + x as usize] = state;
    }

    pub fn clear(&mut self) {
        for y in 0..CHIP8_SCREEN_HEIGHT {
            for x in 0..CHIP8_SCREEN_WIDTH {
                self.render_table[y as usize * CHIP8_SCREEN_WIDTH as usize + x as usize] = false;
            }
        }
    }
}
