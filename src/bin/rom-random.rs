use rustedbytes_bytepusher_rombuilder::rom_builder::RomBuilder;

fn main() {
    let mut rm = RomBuilder::new();

    const PROGRAM_START: usize = 0x000100;
    const AUDIO_START: usize = 0x00FF00;
    const SCREEN_START: usize = 0x010000;

    // Initialize registers
    rm.init_regs(0x0000, PROGRAM_START, SCREEN_START, AUDIO_START);

    // ROM logic
    rm.org(PROGRAM_START);
    rm.wait();

    // No sound dummy samples
    rm.org(AUDIO_START);
    rm.db(&[0; 256]);

    // Generate random pixels
    rm.org(SCREEN_START);
    for _ in 0..65536 {
        rm.dbb(rand::random::<u8>() % 217);
    }

    // Save the ROM file on disk
    rm.save_to_file("roms/RandomPattern.BytePusher")
        .expect("Failed to save ROM file");
}
