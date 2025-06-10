
pub struct RomBuilder {
    rom: Vec<u8>,
    program_counter: usize,
}

impl RomBuilder {
    pub fn new() -> Self {
        RomBuilder {
            rom: Vec::new(),
            program_counter: 0,
        }
    }

    pub fn get_current_addr(&self) -> usize {
        self.program_counter
    }

    pub fn get_next_addr(&mut self) -> usize {
        self.program_counter + 3
    }

    pub fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn set_program_counter(&mut self, addr: usize) {
        self.program_counter = addr;
    }

    pub fn write_u24(&mut self, value: u32) {
        self.rom.extend_from_slice(&value.to_le_bytes());
        self.program_counter += 4;
    }

    pub fn write_u16(&mut self, value: u16) {
        self.rom.extend_from_slice(&value.to_le_bytes());
        self.program_counter += 2;
    }

    pub fn write_u8(&mut self, value: u8) {
        self.rom.push(value);
        self.program_counter += 1;
    }

    pub fn org(&mut self, addr: usize) {
        self.program_counter = addr;
    }

    pub fn bbj(&mut self, source: usize, target: usize, jump: usize) {
        self.write_u24(source as u32);
        self.write_u24(target as u32);
        self.write_u24(jump as u32);
    }

    /// No operation
    pub fn nop(&mut self) {
        self.write_u24(0x000000);
        self.write_u24(0x000000);
        let next_addr = self.get_next_addr() as u32;
        self.write_u24(next_addr);
    }

    /// Wait until next frame
    /// This is used to synchronize with the frame rate of the game
    /// Program counter needs to be programmed before calling this function
    pub fn wait(&mut self) {
        self.write_u24(0x000000);
        self.write_u24(0x000000);
        let next_addr = self.get_current_addr() as u32;
        self.write_u24(next_addr);
    }

    /// Unconditional jump to provided address
    pub fn jmp(&mut self, addr: usize) {
        self.write_u24(0x000000);
        self.write_u24(0x000000);
        self.write_u24(addr as u32);
    }

    
    pub fn cpy(&mut self, source: usize, target: usize) {
        self.write_u24(source as u32);
        self.write_u24(target as u32);
        let next_addr = self.get_next_addr() as u32;
        self.write_u24(next_addr);

    }

}
