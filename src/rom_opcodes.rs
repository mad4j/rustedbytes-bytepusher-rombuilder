use crate::rom_builder::RomBuilder;

impl RomBuilder {
    pub fn org(&mut self, addr: usize) -> &mut Self {
        self.set_program_counter(addr);
        self
    }

    pub fn bbj(&mut self, source: usize, target: usize, jump: usize) -> &mut Self {
        self.write_u24(source as u32)
            .write_u24(target as u32)
            .write_u24(jump as u32)
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
    pub fn wait(&mut self) -> &mut Self {
        self.write_u24(0x000000);
        self.write_u24(0x000000);
        let next_addr = self.get_current_addr() as u32;
        self.write_u24(next_addr);
        self
    }

    /// Unconditional jump to provided address
    pub fn jmp(&mut self, addr: usize) -> &mut Self {
        self.write_u24(0x000000)
            .write_u24(0x000000)
            .write_u24(addr as u32)
    }

    pub fn cpy(&mut self, source: usize, target: usize) -> &mut Self {
        let next_addr = self.get_next_addr() as u32;
        self.write_u24(source as u32)
            .write_u24(target as u32)
            .write_u24(next_addr)
    }

    pub fn db(&mut self, data: &[u8]) -> &mut Self {
        for &x in data {
            self.write_u8(x);
        }
        self
    }

    pub fn dbb(&mut self, data: u8) -> &mut Self {
        self.write_u8(data)
    }

    pub fn inc(&mut self, addr: usize) -> &mut Self {
        let inc_table_addr = self
            .get_inc_table_addr()
            .expect("Please install the increment table first using install_inc_table().");
        self.cpy(inc_table_addr + (addr & 0xFF), addr)
    }

}
