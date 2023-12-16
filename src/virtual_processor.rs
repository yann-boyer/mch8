use crate::globals::*;
use crate::memory::Memory;
use crate::render_table::RenderTable;
use crate::render_table::PixelState;
use crate::audio_system::AudioSystem;

use rand::{rngs, Rng};

const REGISTERS_COUNT: u8 = 16;
const KEYS_COUNT: u8 = 16;

pub struct VirtualProcessor {
    registers: [u8; REGISTERS_COUNT as usize],
    keys: [bool; KEYS_COUNT as usize],
    stack: Vec<u16>,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    draw_flag: bool,
    rng: rngs::ThreadRng,
}

impl VirtualProcessor {
    pub fn new() -> VirtualProcessor {
        VirtualProcessor {
            registers: [0x0; REGISTERS_COUNT as usize],
            keys: [false; KEYS_COUNT as usize],
            stack: Vec::new(),
            index_register: 0x0,
            delay_timer: 0x0,
            sound_timer: 0x0,
            program_counter: PROCESSOR_INTERNAL_PROGRAM_COUNTER_START,
            draw_flag: false,
            rng: rand::thread_rng(),
        }
    }

    pub fn set_key(&mut self, n: u8, is_down: bool) {
        self.keys[n as usize] = is_down;
    }

    pub fn get_draw_flag(&self) -> bool {
        self.draw_flag
    }

    pub fn reset_draw_flag(&mut self) {
        self.draw_flag = false;
    }

    pub fn update_timers(&mut self, audio_system: &AudioSystem) {
        if self.delay_timer > 0x0 { self.delay_timer -= 1; }
        if self.sound_timer > 0x0 {
            self.sound_timer -= 1;
            if self.sound_timer == 0x1 {
                audio_system.play_beep_sound();
            }
        }
    }

    pub fn fetch_next_opcode(&self, memory: &Memory) -> u16 {
        let msb = memory.read(self.program_counter) as u16;
        let lsb = memory.read(self.program_counter + 1) as u16;

        msb << 8 | lsb
    }

    pub fn execute_instruction(&mut self, opcode: u16, memory: &mut Memory, render_table: &mut RenderTable) {
        println!("[Info] Currently executed Opcode -> {:#06x}", opcode);

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0x00E0 => {
                        // CLS
                        render_table.clear();
                        self.draw_flag = true;
                        self.program_counter += 2;
                    },
                    0x00EE => {
                        // RET
                        self.program_counter = self.stack.pop().unwrap();
                        self.program_counter += 2;
                    },
                    _ => panic!("[Error] Unknown opcode -> {:#06x}", opcode)
                }
            },
            0x1000 => {
                // JP addr
                self.program_counter = nnn;
            },
            0x2000 => {
                // CALL addr
                self.stack.push(self.program_counter);
                self.program_counter = nnn;
            },
            0x3000 => {
                // SE Vx, byte
                if self.registers[x as usize] == nn { self.program_counter += 4; }
                else { self.program_counter += 2; }  
            },
            0x4000 => {
                // SNE Vx, byte
                if self.registers[x as usize] != nn { self.program_counter += 4; }
                else { self.program_counter += 2; }
            },
            0x5000 => {
                // SE Vx, Vy
                if self.registers[x as usize] == self.registers[y as usize] { self.program_counter += 4; }
                else { self.program_counter += 2; }
            },
            0x6000 => {
                // LD Vx, byte
                self.registers[x as usize] = nn;
                self.program_counter += 2;
            },
            0x7000 => {
                // ADD Vx, byte
                self.registers[x as usize] += nn;
                self.program_counter += 2;
            },
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // LD Vx, Vy
                        self.registers[x as usize] = self.registers[y as usize];
                        self.program_counter += 2;
                    },
                    0x0001 => {
                        // OR Vx, Vy
                        self.registers[x as usize] |= self.registers[y as usize];
                        self.program_counter += 2;
                    },
                    0x0002 => {
                        // AND Vx, Vy
                        self.registers[x as usize] &= self.registers[y as usize];
                        self.program_counter += 2;
                    },
                    0x0003 => {
                        // XOR Vx, Vy
                        self.registers[x as usize] ^= self.registers[y as usize];
                        self.program_counter += 2;
                    },
                    0x0004 => {
                        // ADD Vx, Vy
                        let r = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;

                        self.registers[0xF] = if r > 0xFF { 1 } else { 0 };

                        self.registers[x as usize] = (r & 0xFF) as u8;
                        
                        self.program_counter += 2;
                    },
                    0x0005 => {
                        // SUB Vx, Vy
                        self.registers[0xF] = if self.registers[x as usize] > self.registers[y as usize] { 1 } else { 0 };

                        self.registers[x as usize] -= self.registers[y as usize];

                        self.program_counter += 2;
                    },
                    0x0006 => {
                        // SHR Vx {, Vy}
                        self.registers[0xF] = self.registers[x as usize] & 0x1;
                        self.registers[x as usize] >>= 1;

                        self.program_counter += 2;
                    },
                    0x0007 => {
                        // SUBN Vx, Vy
                        self.registers[0xF] = if self.registers[y as usize] > self.registers[x as usize] { 1 } else { 0 };

                        self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];

                        self.program_counter += 2;
                    },
                    0x000E => {
                        // SHL Vx {, Vy}
                        self.registers[0xF] = (self.registers[x as usize] & 128) >> 7;
                        self.registers[x as usize] <<= 1;
                        
                        self.program_counter += 2;
                    },
                    _ => panic!("[Error] Unknown opcode -> {:#06x}", opcode)
                }
           },
           0x9000 => {
                // SNE Vx, Vy
                if self.registers[x as usize] != self.registers[y as usize] { self.program_counter += 4; }
                else { self.program_counter += 2; }
           },
           0xA000 => {
                // LD I, addr
                self.index_register = nnn;
                self.program_counter += 2;
           },
           0xB000 => {
                // JP V0, addr
                self.program_counter = self.registers[0x0] as u16 + nnn;
                self.program_counter += 2;
           },
           0xC000 => {
                // RND Vx, byte
                self.registers[x as usize] = self.rng.gen_range(0..0xFF) & nn;
                self.program_counter += 2;
           },
           0xD000 => {
                // DRW Vx, Vy, nibble
                self.registers[0xF] = 0;

                let origin_x = self.registers[x as usize];
                let origin_y = self.registers[y as usize];

                for y_coord in 0..n {
                    for x_coord in 0..8 {
                        let pixel = memory.read(self.index_register + y_coord as u16);
                        if pixel & (0x80 >> x_coord) != 0 {
                            let pixel_x = origin_x.wrapping_add(x_coord) % CHIP8_SCREEN_WIDTH;
                            let pixel_y = origin_y.wrapping_add(y_coord) % CHIP8_SCREEN_HEIGHT;

                            if render_table.is_pixel_switched_on(pixel_x, pixel_y) {
                                render_table.change_pixel_state_to(pixel_x, pixel_y, PixelState::SwitchedOff);
                                self.registers[0xF] = 1; // collision.
                            } else {
                                render_table.change_pixel_state_to(pixel_x, pixel_y, PixelState::SwitchedOn);
                            }
                        }
                    }
                }

                self.draw_flag = true;
                self.program_counter += 2;
           },
           0xE000 => {
                match opcode & 0x00FF {
                    0x009E => {
                        // SKP Vx
                        if self.keys[self.registers[x as usize] as usize] { self.program_counter += 4; }
                        else { self.program_counter += 2 };
                    },
                    0x00A1 => {
                        // SKNP VX
                        if !self.keys[self.registers[x as usize] as usize] { self.program_counter += 4; }
                        else { self.program_counter += 2; }
                    },
                    _ => panic!("[Error] Unknown opcode -> {:#06x}", opcode)
                }
           },
           0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                        // LD Vx, DT
                        self.registers[x as usize] = self.delay_timer;
                        self.program_counter += 2;
                    },
                    0x000A => {
                        // LD Vx, K
                        let mut pressed = false;

                        for i in 0..KEYS_COUNT {
                            if self.keys[i as usize] {
                                self.registers[x as usize] = i;
                                pressed = true;
                            }
                        }

                        if !pressed {
                            return;
                        }

                        self.program_counter += 2;
                    },
                    0x0015 => {
                        // LD DT, Vx
                        self.delay_timer = self.registers[x as usize];
                        self.program_counter += 2;
                    },
                    0x0018 => {
                        // LD ST, Vx
                        self.sound_timer = self.registers[x as usize];
                        self.program_counter += 2;
                    },
                    0x001E => {
                        // ADD I, Vx
                        self.index_register += self.registers[x as usize] as u16;
                        self.program_counter += 2;
                    },
                    0x0029 => {
                        // LD F, Vx
                        self.index_register = self.registers[x as usize] as u16 * 5;
                        self.program_counter += 2;
                    },
                    0x0033 => {
                        // LD B, Vx

                        let reg_val = self.registers[x as usize];

                        memory.write(self.index_register, reg_val / 100); // Hundreds.
                        memory.write(self.index_register + 1, (reg_val % 100) / 10); // Tens.
                        memory.write(self.index_register + 2, reg_val % 10);

                        self.program_counter += 2;
                    },
                    0x0055 => {
                        // LD [I], Vx
                        for i in 0..(x + 1) {
                            let reg = self.registers[i as usize];
                            memory.write(self.index_register + i as u16, reg);
                        }

                        self.program_counter += 2;
                    },
                    0x0065 => {
                        // LD Vx, [I]
                        for i in 0..(x + 1) {
                            let reg_from_mem = memory.read(self.index_register + i as u16);
                            self.registers[i as usize] = reg_from_mem;
                        }

                        self.program_counter += 2;
                    },
                    _ => panic!("[Error] Unknown opcode -> {:#06x}", opcode)
                }
           },
            _ => panic!("[Error] Unknown opcode -> {:#06x}", opcode)
        }
    }
}