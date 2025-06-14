use rustedbytes_bytepusher_rombuilder::{
    image::convert_image_dithered_strength, rom_builder::RomBuilder,
};

fn main() {
    let mut rm = RomBuilder::new();

    const PROGRAM_START: usize = 0x000100;
    const AUDIO_START: usize = 0x00FF00;
    const SCREEN_START: usize = 0x010000;

    let image = convert_image_dithered_strength("resources/evy-256x256.png", 1.0)
        .expect("Failed to load image file.");

    // Initialize registers
    rm.init_regs(0x0000, PROGRAM_START, SCREEN_START, AUDIO_START);

    // ROM logic
    rm.org(PROGRAM_START).wait();

    // No sound dummy samples
    rm.org(AUDIO_START).db(&[0; 256]);

    // Generate random pixels
    rm.org(SCREEN_START).db(&image);

    // Save the ROM file on disk
    rm.save_to_file("roms/Evy.BytePusher")
        .expect("Failed to save ROM file.");
}
