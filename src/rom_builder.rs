use std::fs::File;
use std::io::Write;

pub struct RomBuilder {
    rom: Vec<u8>,
    program_counter: usize,
}

impl RomBuilder {
    pub fn new() -> Self {
        RomBuilder {
            rom: vec![0; 16 * 1024 * 1024],
            program_counter: 0,
        }
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        let mut trimmed_rom = self.rom.clone();
        while let Some(&0) = trimmed_rom.last() {
            trimmed_rom.pop();
        }
        file.write_all(&trimmed_rom)?;
        Ok(())
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
        self.rom.extend_from_slice(&value.to_be_bytes()[1..]);
        self.program_counter += 3;
    }

    pub fn write_u16(&mut self, value: u16) {
        self.rom.extend_from_slice(&value.to_be_bytes());
        self.program_counter += 2;
    }

    pub fn write_u8(&mut self, value: u8) {
        self.rom[self.program_counter] = value;
        self.program_counter += 1;
    }

    pub fn init_regs(
        &mut self,
        keyb_flags: u16,
        program_addr: usize,
        screen_addr: usize,
        audio_addr: usize,
    ) {
        self.write_u16(keyb_flags);
        self.write_u24(program_addr as u32);
         self.write_u16((screen_addr >> 8) as u16);
        self.write_u8((audio_addr >> 16) as u8);
    }

    pub fn label(&self) -> usize {
        self.program_counter
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

    pub fn db(&mut self, data: &[u8]) {
        for d in data {
            self.write_u8(*d);
        }
    }

    pub fn dbb(&mut self, data: u8) {
        self.write_u8(data);
    }
}
