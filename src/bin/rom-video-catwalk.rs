use rustedbytes_bytepusher_rombuilder::{
    rom_builder::{RomBuilder, SCREEN_REGISTER_ADDR},
    video::process_png_sequence_flat,
};

const KERNEL_START: usize = 0x000100;
const PROGRAM_START: usize = 0x000300;
const AUDIO_START: usize = 0x00FF00;
const SCREEN_START: usize = 0x010000;

fn main() {
    let mut rm = RomBuilder::new();

    let video = process_png_sequence_flat("resources/videos/frame_*.png")
        .expect("Failed to load video frames");

    let frame_count = video.len() / (64 * 1024);

    // Initialize registers
    rm.init_regs(0x0000, PROGRAM_START, SCREEN_START, AUDIO_START);

    // ROM logic
    rm.org(KERNEL_START);
    rm.install_id_table();
    rm.install_inc_table();

    rm.org(PROGRAM_START);
    for _ in 1..frame_count {
        rm.inc(SCREEN_REGISTER_ADDR).sync().sync().sync().sync();
    }

    // No sound dummy samples
    rm.org(AUDIO_START).db_arr(&[0; 256]);

    // Add video frames
    rm.org(SCREEN_START).db_arr(&video);

    // Save the ROM file on disk
    rm.save_to_file("roms/Catwalk.BytePusher")
        .expect("Failed to save ROM file");
}
