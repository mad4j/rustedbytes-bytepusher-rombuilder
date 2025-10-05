/*
 * T-Runner ROM for BytePusher VM
 * 
 * A simple endless runner game inspired by Chrome's dinosaur game.
 * This demo shows an animated runner with moving obstacles.
 * 
 * Technical details:
 * - 256x256 pixel display
 * - Pre-rendered animation frames showing the running sequence
 * - Obstacles scroll from right to left
 * - Simple looping animation
 * 
 * Due to BytePusher VM limitations, this is implemented as an animated demo
 * rather than a fully interactive game.
 */

use rustedbytes_bytepusher_rombuilder::rom_builder::{RomBuilder, SCREEN_REGISTER_ADDR};

// Memory layout
const KERNEL_START: usize = 0x000100;
const PROGRAM_START: usize = 0x000300;
const AUDIO_START: usize = 0x00FF00;
const SCREEN_START: usize = 0x010000;

// Animation constants
const NUM_FRAMES: usize = 8;  // Number of animation frames

// Colors (BytePusher 6x6x6 RGB palette)
const COLOR_SKY: u8 = 215;        // White/Light gray (5,5,5)
const COLOR_GROUND: u8 = 108;     // Medium gray (3,0,0)
const COLOR_PLAYER: u8 = 36;      // Dark gray (1,0,0)
const COLOR_OBSTACLE: u8 = 72;    // Darker gray (2,0,0)

// T-Rex sprite data (20x20 pixels)
// 0 = transparent (sky), 1 = dark gray (body)
const TREX_SPRITE_1: [[u8; 20]; 20] = [
    [0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0],
    [0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,0,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0],
];

// T-Rex sprite with leg position 2 (running animation)
const TREX_SPRITE_2: [[u8; 20]; 20] = [
    [0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0],
    [0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,1,1,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0],
];

fn main() {
    let mut rm = RomBuilder::new();

    // Initialize registers
    rm.init_regs(0x0000, PROGRAM_START, SCREEN_START, AUDIO_START);

    // Install kernel tables
    rm.org(KERNEL_START);
    rm.install_id_table();
    rm.install_inc_table();

    // Main animation loop
    rm.org(PROGRAM_START);

    // Cycle through frames
    for frame_idx in 0..NUM_FRAMES {
        let frame_addr = SCREEN_START + (frame_idx * 65536);
        
        // Wait for several frames for smoother animation
        for _ in 0..4 {
            rm.sync();
        }
        
        // Set screen register to point to this frame
        rm.cpyi((frame_addr >> 16) as u8, SCREEN_REGISTER_ADDR);
    }
    
    // Jump back to start of animation loop
    rm.jmp(PROGRAM_START);

    // No sound - dummy samples
    rm.org(AUDIO_START).db_arr(&[0; 256]);

    // Generate animation frames
    rm.org(SCREEN_START);
    
    for frame in 0..NUM_FRAMES {
        // Calculate obstacle position for this frame (moving left)
        let obstacle_x = 220 - (frame * 20);
        
        generate_frame(&mut rm, frame, obstacle_x);
    }

    // Save the ROM file
    rm.save_to_file("roms/TRunner.BytePusher")
        .expect("Failed to save ROM file");

    println!("T-Runner ROM created!");
    println!("ROM size: {} bytes", SCREEN_START + NUM_FRAMES * 65536);
    println!("Number of frames: {}", NUM_FRAMES);
    println!("This is an animated demo of the T-Runner game concept");
}

fn generate_frame(rm: &mut RomBuilder, frame_idx: usize, obstacle_x: usize) {
    // Generate a 256x256 frame
    
    // Ground level
    const GROUND_Y: usize = 200;
    const PLAYER_X: usize = 40;
    const PLAYER_Y: usize = 170;
    const PLAYER_SIZE: usize = 20;
    const OBSTACLE_SIZE: usize = 20;
    
    // Select T-Rex sprite based on frame (alternating for running animation)
    let trex_sprite = if frame_idx % 2 == 0 {
        &TREX_SPRITE_1
    } else {
        &TREX_SPRITE_2
    };
    
    for y in 0..256 {
        for x in 0..256 {
            let mut color = COLOR_SKY;
            
            // Draw ground
            if y >= GROUND_Y {
                color = COLOR_GROUND;
            }
            
            // Draw ground line
            if y == GROUND_Y {
                color = COLOR_PLAYER;
            }
            
            // Draw T-Rex sprite
            if x >= PLAYER_X && x < PLAYER_X + PLAYER_SIZE &&
               y >= PLAYER_Y && y < PLAYER_Y + PLAYER_SIZE {
                let sprite_x = x - PLAYER_X;
                let sprite_y = y - PLAYER_Y;
                
                // Get pixel from sprite (1 = draw, 0 = transparent)
                if trex_sprite[sprite_y][sprite_x] == 1 {
                    color = COLOR_PLAYER;
                }
            }
            
            // Draw obstacle (simple cactus rectangle)
            if obstacle_x < 256 && x >= obstacle_x && x < obstacle_x + OBSTACLE_SIZE &&
               y >= GROUND_Y - OBSTACLE_SIZE && y < GROUND_Y {
                color = COLOR_OBSTACLE;
            }
            
            rm.db(color);
        }
    }
}
