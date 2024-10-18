pub const RAM_SIZE: usize = 256;

pub struct RAM {
    memory: [u8; RAM_SIZE],
}

impl RAM {
    pub fn new() -> Self {
        RAM {
            memory: [0; RAM_SIZE],
        }
    }

    pub fn read(&self, address: usize) -> Result<u8, String> {
        if address >= RAM_SIZE {
            Err(format!("Address 0x{:X} is out of bounds.", address))
        } else {
            Ok(self.memory[address])
        }
    }

    pub fn write(&mut self, address: usize, value: u8) -> Result<(), String> {
        if address >= RAM_SIZE {
            Err(format!("Address 0x{:X} is out of bounds.", address))
        } else {
            self.memory[address] = value;
            Ok(())
        }
    }

    pub fn dump(&self, start: usize, length: usize) -> Vec<String> {
        let end = start.saturating_add(length).min(RAM_SIZE);
        self.memory[start..end]
            .iter()
            .map(|&x| format!("{:08b}", x))
            .collect()
    }
}