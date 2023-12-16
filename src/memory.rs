const TOTAL_MEMORY_BYTES: u16 = 0x1000; // 4096 bytes of RAM.

pub struct Memory {
    memory: [u8; TOTAL_MEMORY_BYTES as usize]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0x0; TOTAL_MEMORY_BYTES as usize]
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr > TOTAL_MEMORY_BYTES {
            println!("[Warning] Memory WRITE command out of range !");
            return;
        }

        self.memory[addr as usize] = data;
    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr > TOTAL_MEMORY_BYTES {
            println!("[Warning] Memory READ command out of range !");
            return 0x0;
        }

        self.memory[addr as usize]
    }
}