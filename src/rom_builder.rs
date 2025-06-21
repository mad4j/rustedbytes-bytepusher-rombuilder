use std::fs::File;
use std::io::Write;
use std::ops::Range;

pub const KEYBOARD_REGISTER_ADDR: usize = 0x000000;
pub const PROGRAM_COUNTER_ADDR: usize = 0x000002;
pub const SCREEN_REGISTER_ADDR: usize = 0x000005;
pub const AUDIO_REGISTER_ADDR: usize = 0x000006;

pub struct RomBuilder {
    rom: Vec<u8>,
    program_counter: usize,

    id_table_addr: Option<usize>,
    inc_table_addr: Option<usize>,
}

impl RomBuilder {
    pub fn new() -> Self {
        Self {
            rom: vec![0; 16 * 1024 * 1024],
            program_counter: 0,
            id_table_addr: None,
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

    pub fn set_current_addr(&mut self, addr: usize) {
        self.program_counter = addr;
    }

    pub fn get_current_addr(&self) -> usize {
        self.program_counter
    }

    pub fn get_next_addr(&self) -> usize {
        self.program_counter + 3
    }

    pub fn get_next_instr_addr(&self) -> usize {
        self.program_counter + 9
    }

    pub fn write_addr(&mut self, addr: usize) -> &mut Self {
        self.write_u24((addr & 0x00FFFFFF) as u32)
    }

    pub fn write_current_addr(&mut self) -> &mut Self {
        self.write_addr(self.get_current_addr())
    }

    pub fn write_next_addr(&mut self) -> &mut Self {
        self.write_addr(self.get_next_addr())
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

    pub fn install_id_table(&mut self) -> &mut Self {
        if self.id_table_addr.is_some() {
            panic!("Inc table already installed.")
        }

        if self.get_current_addr() % 256 != 0 {
            panic!("Id table needs to be 256-bytes memory aligned.")
        }

        self.id_table_addr = Some(self.get_current_addr());
        for x in 0..256 {
            self.write_u8(x as u8);
        }

        self
    }

    pub fn get_id_table_addr(&self) -> Option<usize> {
        self.id_table_addr
    }

    pub fn install_inc_table(&mut self) -> &mut Self {
        if self.inc_table_addr.is_some() {
            panic!("Inc table already installed.")
        }

        if self.get_current_addr() % 256 != 0 {
            panic!("Inc table needs to be 256-bytes memory aligned.")
        }

        self.inc_table_addr = Some(self.get_current_addr());
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
