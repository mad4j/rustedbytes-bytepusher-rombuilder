/*
 * Animated Random Noise ROM for BytePusher VM
 * 
 * This ROM demonstrates animated random noise patterns using a pseudo-random
 * number generator. The screen continuously cycles through 4 pre-generated
 * frames of random noise, creating an animated effect.
 * 
 * Technical details:
 * - 4 frames of 256x256 pixels (65,536 bytes each)
 * - Linear Congruential Generator (LCG) for random number generation
 * - Animation loop at ~3.75 FPS (4 sync operations per frame)
 * - Frame counter at 0x000010 for performance measurement
 * - Total ROM size: ~320KB
 */

use rustedbytes_bytepusher_rombuilder::rom_builder::{RomBuilder, SCREEN_REGISTER_ADDR};

fn main() {
    let mut rm = RomBuilder::new();

    // Memory layout
    const KERNEL_START: usize = 0x000100;    // Kernel tables (ID and INC)
    const PROGRAM_START: usize = 0x000300;   // Animation loop code
    const AUDIO_START: usize = 0x00FF00;     // Audio samples
    const SCREEN_START: usize = 0x010000;    // Frame data
    
    // Counter for frame rate measurement (24-bit counter)
    const FRAME_COUNTER_ADDR: usize = 0x000010;
    
    // Initialize registers
    rm.init_regs(0x0000, PROGRAM_START, SCREEN_START, AUDIO_START);

    // Initialize frame counter to 0
    rm.org(FRAME_COUNTER_ADDR);
    rm.db_arr(&[0, 0, 0]);

    // Install kernel tables
    rm.org(KERNEL_START);
    rm.install_id_table();
    rm.install_inc_table();

    // ROM logic - Animation loop
    // This loop cycles through 4 frames, waiting 4 sync periods between each frame
    // to achieve a slower, more visible animation speed
    rm.org(PROGRAM_START);
    
    // Generate 4 different frames of random noise at compile time
    // Each frame is 256x256 pixels = 65536 bytes
    const NUM_FRAMES: usize = 4;
    
    // Animation loop: cycle through frames
    // Each iteration: sync 4 times (for smoother playback) then switch to next frame
    for frame_idx in 0..NUM_FRAMES {
        let frame_addr = SCREEN_START + (frame_idx * 65536);
        
        // Wait for several frames to slow down animation
        for _ in 0..4 {
            rm.sync();
        }
        
        // Set screen register to point to this frame
        rm.cpyi_addr(frame_addr, SCREEN_REGISTER_ADDR);
        
        // Increment frame counter
        rm.inc(FRAME_COUNTER_ADDR + 2); // Increment low byte
    }
    
    // Jump back to start of animation loop
    rm.jmp(PROGRAM_START);

    // No sound - dummy samples
    rm.org(AUDIO_START).db_arr(&[0; 256]);

    // Generate random noise frames
    // Uses different seeds for each frame to ensure variety
    rm.org(SCREEN_START);
    for frame in 0..NUM_FRAMES {
        // Use different seed for each frame to ensure variety
        let mut seed = (frame as u32 * 12345 + 67890) as u64;
        
        for _ in 0..65536 {
            // Simple but effective LCG (Linear Congruential Generator)
            // Constants from Numerical Recipes: a=1103515245, c=12345
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let random_value = ((seed >> 16) & 0xFF) as u8;
            
            // Modulo 216 to fit BytePusher's 6x6x6 RGB palette (216 colors)
            rm.db(random_value % 216);
        }
    }

    // Save the ROM file
    rm.save_to_file("roms/AnimatedNoise.BytePusher")
        .expect("Failed to save ROM file");
    
    // Print information about the ROM
    println!("Animated Random Noise ROM created!");
    println!("ROM size: {} bytes", SCREEN_START + NUM_FRAMES * 65536);
    println!("Number of frames: {}", NUM_FRAMES);
    println!("Frame size: 256x256 pixels = 65536 bytes");
    println!("Expected frame rate: ~60 FPS / 16 syncs = ~3.75 FPS (4 syncs per frame × 4 frames)");
    println!("");
    println!("Frame counter is at address 0x{:06X}", FRAME_COUNTER_ADDR);
    println!("Monitor this address to verify actual frame rate!");
}
