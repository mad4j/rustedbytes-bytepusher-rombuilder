use rustedbytes_bytepusher_rombuilder::rom_builder::RomBuilder;

fn main() {
    let mut rm = RomBuilder::new();

    const PROGRAM_START: usize = 0x000100;
    const AUDIO_START: usize = 0x00FF00;
    const SCREEN_START: usize = 0x001000;

    // ROM logic
    rm.org(PROGRAM_START);
    rm.wait();

    // No sound dummy samples
    rm.org(AUDIO_START);
    rm.db(&[0; 256]);

    rm.org(SCREEN_START);
    for _ in 0..65536 {
        rm.db(&[rand::random::<u8>() % 217]);
    }
}
