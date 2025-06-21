use crate::rom_builder::{PROGRAM_COUNTER_ADDR, RomBuilder};

impl RomBuilder {
    /// Move current address to the specified address
    /// 0 bytes are written to the address
    pub fn org(&mut self, addr: usize) -> &mut Self {
        self.set_current_addr(addr);
        self
    }

    /// Write a ByteByteJump instruction with source, target, and jump addresses
    /// 9 bytes are written to the ROM
    pub fn bbj(&mut self, source: usize, target: usize, jump: usize) -> &mut Self {
        self.write_addr(source).write_addr(target).write_addr(jump)
    }

    /// No operation
    /// 9 bytes are written to the ROM
    pub fn nop(&mut self) -> &mut Self {
        self.bbj(0x000000, 0x000000, self.get_next_instr_addr())
    }

    /// Wait until next frame
    /// This is used to synchronize with the frame rate of the game
    /// Program counter needs to be programmed before calling this function
    /// 9 bytes are written to the ROM
    pub fn wait(&mut self) -> &mut Self {
        self.bbj(0x000000, 0x000000, self.get_current_addr())
    }

    /// Wait until next frame before continuing
    /// 36 bytes are written to the ROM (maybe optimizable to 27 bytes)
    pub fn sync(&mut self) -> &mut Self {
        self.cpyi_addr(self.get_current_addr() + 36, PROGRAM_COUNTER_ADDR)
            .wait()
    }

    /// Unconditional jump to provided address
    /// 9 bytes are written to the ROM
    pub fn jmp(&mut self, addr: usize) -> &mut Self {
        self.bbj(0x0, 0x0, addr)
    }

    /// Copy byte value from source to target address
    /// 9 bytes are written to the ROM
    pub fn cpy(&mut self, source: usize, target: usize) -> &mut Self {
        self.bbj(source, target, self.get_next_instr_addr())
    }

    /// Copy an immediate value to target address
    /// 9 bytes are written to the ROM
    pub fn cpyi(&mut self, value: u8, target: usize) -> &mut Self {
        let id_table_addr = self
            .get_id_table_addr()
            .expect("Please install the identity table first using install_id_table()");

        self.cpy(id_table_addr + value as usize, target)
    }

    /// Copy an immediate 24-bit value to target address
    /// 27 bytes are written to the ROM
    pub fn cpyi_addr(&mut self, value: usize, target: usize) -> &mut Self {
        self.cpyi(((value >> 16) & 0xFF) as u8, target)
            .cpyi(((value >> 8) & 0xFF) as u8, target + 1)
            .cpyi((value & 0xFF) as u8, target + 2)
    }

    pub fn db_arr(&mut self, data: &[u8]) -> &mut Self {
        for &x in data {
            self.write_u8(x);
        }
        self
    }

    pub fn db(&mut self, data: u8) -> &mut Self {
        self.write_u8(data)
    }

    pub fn inc(&mut self, addr: usize) -> &mut Self {
        let inc_table_addr = self
            .get_inc_table_addr()
            .expect("Please install the increment table first using install_inc_table()");

        self.cpy(addr, self.get_next_instr_addr() + 2)
            .cpy(inc_table_addr, addr)
    }
}

#[cfg(test)]
mod tests {

    use crate::rom_builder::RomBuilder;

    fn exec_bbj(rom: &mut RomBuilder, instr_addr: usize) -> usize {
        let read_addr = |offset| {
            ((rom[instr_addr + offset] as usize) << 16)
                | ((rom[instr_addr + offset + 1] as usize) << 8)
                | (rom[instr_addr + offset + 2] as usize)
        };
        let source = read_addr(0);
        let target = read_addr(3);
        let jump = read_addr(6);

        rom[target] = rom[source];

        jump
    }

    #[test]
    fn test_inc_generates_expected_bytes() {
        let mut rb = RomBuilder::new();

        rb.org(0x000000);
        rb.install_inc_table();

        rb.org(0x000100);

        rb.db(0x00);
        rb.inc(0x000100);
        rb.inc(0x000100);

        let mut pc = 0x000101;
        for _ in 0..2 {
            pc = exec_bbj(&mut rb, pc);
            println!("{:?}", &rb[0x000100..0x000100 + 19]);
        }
        assert_eq!(rb[0x000100], 0x01);

        for _ in 0..2 {
            pc = exec_bbj(&mut rb, pc);
            println!("{:?}", &rb[0x000100..0x000100 + 19]);
        }
        assert_eq!(rb[0x000100], 0x02);
    }
}
