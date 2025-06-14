use clap::Parser;
use image::RgbImage;
use rustedbytes_bytepusher_rombuilder::{
    image::convert_image_dithered_strength, rom_builder::RomBuilder,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input image file path
    #[arg(short, long)]
    image: String,

    /// Dithering strength (default: 1.0)
    #[arg(short, long, default_value_t = 1.0)]
    dithering: f32,

    /// Output ROM file path (default: <image>.BytePusher)
    #[arg(short, long)]
    output: Option<String>,

    /// Salva una preview PNG dell'immagine convertita (BytePusher)
    #[arg(long)]
    preview: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut rm = RomBuilder::new();

    const PROGRAM_START: usize = 0x000100;
    const AUDIO_START: usize = 0x00FF00;
    const SCREEN_START: usize = 0x010000;

    let image = convert_image_dithered_strength(&args.image, args.dithering)
        .expect("Failed to load image file.");

    // Initialize registers
    rm.init_regs(0x0000, PROGRAM_START, SCREEN_START, AUDIO_START);

    // ROM logic
    rm.org(PROGRAM_START).wait();

    // No sound dummy samples
    rm.org(AUDIO_START).db(&[0; 256]);

    // Generate random pixels
    rm.org(SCREEN_START).db(&image);

    // Determine output ROM file name
    let output_rom = match &args.output {
        Some(path) => path.clone(),
        None => {
            let path = std::path::Path::new(&args.image);
            let stem = path.file_stem().unwrap_or_default();
            let parent = path.parent().unwrap_or_else(|| std::path::Path::new(""));
            let mut out_path = parent.join(stem);
            out_path.set_extension("BytePusher");
            out_path.to_string_lossy().to_string()
        }
    };

    // Save the ROM file on disk
    rm.save_to_file(output_rom.as_str())
        .expect("Failed to save ROM file.");

     // Se richiesto, salva la preview PNG
    if let Some(preview_path) = &args.preview {
        save_bytepusher_preview_png(&image, preview_path)
            .expect("Failed to save preview PNG");
    }


}

/// Salva una preview PNG a partire dal buffer BytePusher (256x256, palette 216 colori)
fn save_bytepusher_preview_png(data: &[u8], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let palette = rustedbytes_bytepusher_rombuilder::image::build_palette();
    let mut img = RgbImage::new(256, 256);
    for (i, &idx) in data.iter().enumerate() {
        let x = (i % 256) as u32;
        let y = (i / 256) as u32;
        img.put_pixel(x, y, palette[idx as usize]);
    }
    img.save(path)?;

    Ok(())
}