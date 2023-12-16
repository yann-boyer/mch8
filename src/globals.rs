pub const CHIP8_SCREEN_WIDTH: u8 = 64;
pub const CHIP8_SCREEN_HEIGHT: u8 = 32;
pub const SCALE_FACTOR: u8 = 10;
pub const WINDOW_WIDTH: u32 = CHIP8_SCREEN_WIDTH as u32 * SCALE_FACTOR as u32;
pub const WINDOW_HEIGHT: u32 = CHIP8_SCREEN_HEIGHT as u32 * SCALE_FACTOR as u32;
pub const CPU_CLOCK_DELAY: u16 = 1000; // 1000 microseconds, 1ms.
pub const TIMER_DIVISION_CLOCK: u8 = 9;
pub const PROCESSOR_INTERNAL_PROGRAM_COUNTER_START: u16 = 0x200; // Processor's PC starts 0x200(512).
