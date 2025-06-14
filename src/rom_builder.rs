use std::fs::File;
use std::io::Write;
use std::ops::Range;

pub struct RomBuilder {
    rom: Vec<u8>,
    program_counter: usize,

    inc_table_addr: Option<usize>,
}

impl RomBuilder {
    pub fn new() -> Self {
        RomBuilder {
            rom: vec![0; 16 * 1024 * 1024],
            program_counter: 0,
            inc_table_addr: None,
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

    pub fn write_u24(&mut self, value: u32) -> &mut Self {
        let addr = self.program_counter;
        self.rom[addr] = ((value & 0xFF0000) >> 16) as u8;
        self.rom[addr + 1] = ((value & 0xFF00) >> 8) as u8;
        self.rom[addr + 2] = (value & 0xFF) as u8;
        self.program_counter += 3;
        self
    }

    pub fn write_u16(&mut self, value: u16) -> &mut Self {
        let addr = self.program_counter;
        self.rom[addr] = ((value & 0xFF00) >> 8) as u8;
        self.rom[addr + 1] = (value & 0xFF) as u8;
        self.program_counter += 2;
        self
    }

    pub fn write_u8(&mut self, value: u8) -> &mut Self {
        self.rom[self.program_counter] = value;
        self.program_counter += 1;
        self
    }

    pub fn init_regs(
        &mut self,
        keyb_flags: u16,
        program_addr: usize,
        screen_addr: usize,
        audio_addr: usize,
    ) -> &mut Self {
        self.write_u16(keyb_flags)
            .write_u24(program_addr as u32)
            .write_u8((screen_addr >> 16) as u8)
            .write_u16((audio_addr >> 8) as u16)
    }

    pub fn install_inc_table(&mut self, base_addr: usize) -> &mut Self {
        self.inc_table_addr = Some(base_addr);
        self.org(base_addr);
        for x in 0..256 {
            self.write_u8(((x + 1) % 256) as u8);
        }
        self
    }

    pub fn get_inc_table_addr(&self) -> Option<usize> {
        self.inc_table_addr
    }
}

// Implement Index and IndexMut traits for RomBuilder at module scope

impl std::ops::Index<usize> for RomBuilder {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rom[index]
    }
}

impl std::ops::IndexMut<usize> for RomBuilder {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rom[index]
    }
}

impl std::ops::Index<Range<usize>> for RomBuilder {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.rom[index]
    }
}

impl std::ops::IndexMut<Range<usize>> for RomBuilder {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.rom[index]
    }
}
