use crate::audio_system::AudioSystem;
use crate::globals::*;
use crate::memory::Memory;
use crate::render_table::RenderTable;
use crate::virtual_processor::VirtualProcessor;
use std::fs::File;
use std::io::{BufReader, Read};

const MAX_ROM_PROGRAM_SIZE: u16 = 0xFFF - PROCESSOR_INTERNAL_PROGRAM_COUNTER_START;
const FONTSET_SIZE: u8 = 80;
const FONTSET: [u8; FONTSET_SIZE as usize] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct VirtualMachine {
    virtual_processor: VirtualProcessor,
    memory: Memory,
    render_table: RenderTable,
    audio_system: AudioSystem,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            virtual_processor: VirtualProcessor::new(),
            memory: Memory::new(),
            render_table: RenderTable::new(),
            audio_system: AudioSystem::new(),
        }
    }

    pub fn init_audio(&mut self) {
        self.audio_system.init();
    }

    fn load_fontset(&mut self) {
        for i in 0..FONTSET_SIZE {
            self.memory.write(i as u16, FONTSET[i as usize]);
        }
    }

    pub fn load_rom(&mut self, rom_path: &str) -> Result<(), &str> {
        let rom_file_result = File::open(rom_path);
        if rom_file_result.is_err() {
            return Err("[Error] Unable to open the given ROM file !");
        }
        let rom_file = rom_file_result.unwrap();

        let mut rom_buffer = Vec::new();
        let mut file_buf_reader = BufReader::new(rom_file);

        let rom_reader_result = file_buf_reader.read_to_end(&mut rom_buffer);
        if rom_reader_result.is_err() {
            return Err("[Error] Unable to read the given ROM file !");
        }

        let rom_buffer_len = rom_buffer.len();

        if rom_buffer_len > MAX_ROM_PROGRAM_SIZE as usize {
            return Err("[Error] The ROM file given is too big to fit into memory !");
        }

        self.load_fontset(); // Load fontset into memory before anything else.

        // TEST WARN
        for (byte_index, byte_value) in rom_buffer.iter().enumerate() {
            self.memory.write(
                PROCESSOR_INTERNAL_PROGRAM_COUNTER_START + byte_index as u16,
                *byte_value,
            );
        }

        println!("[Info] ROM successfully loaded into memory !");

        Ok(())
    }

    pub fn set_key(&mut self, n: u8, is_down: bool) {
        self.virtual_processor.set_key(n, is_down);
    }

    pub fn screen_need_repaint(&self) -> bool {
        self.virtual_processor.get_draw_flag()
    }

    pub fn disable_repaint(&mut self) {
        self.virtual_processor.reset_draw_flag();
    }

    pub fn is_pixel_switched_on(&self, x: u8, y: u8) -> bool {
        self.render_table.is_pixel_switched_on(x, y)
    }

    pub fn execute_processor_instruction(&mut self) {
        let opcode = self.virtual_processor.fetch_next_opcode(&self.memory);
        self.virtual_processor.execute_instruction(
            opcode,
            &mut self.memory,
            &mut self.render_table,
        );
    }

    pub fn update_processor_timers(&mut self) {
        self.virtual_processor.update_timers(&self.audio_system);
    }
}
