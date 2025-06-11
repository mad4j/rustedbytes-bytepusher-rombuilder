use glob::glob;
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
use std::path::Path;

use crate::error::BytePusherError;

/// Risultato di elaborazione di un singolo frame
#[derive(Debug, Clone)]
pub struct ProcessedFrame {
    /// Dati dell'immagine come bytes RGB
    pub rgb_data: Vec<u8>,
    /// Larghezza dell'immagine
    pub width: u32,
    /// Altezza dell'immagine
    pub height: u32,
    /// Indice del frame nella sequenza
    pub frame_index: usize,
    /// Nome del file originale
    pub file_name: String,
}

/// Palette BytePusher VM (216 colori: 6x6x6 RGB)
const PALETTE_SIZE: usize = 216;

/// Genera la palette BytePusher standard con 216 colori (6x6x6 RGB)
fn generate_bytepusher_palette() -> Vec<[u8; 3]> {
    let mut palette = Vec::with_capacity(PALETTE_SIZE);

    for r in 0..6 {
        for g in 0..6 {
            for b in 0..6 {
                let red = (r * 255 / 5) as u8;
                let green = (g * 255 / 5) as u8;
                let blue = (b * 255 / 5) as u8;
                palette.push([red, green, blue]);
            }
        }
    }

    palette
}

/// Applica il dithering Floyd-Steinberg con coerenza tra frame
fn apply_floyd_steinberg_dither(
    img: &DynamicImage,
    palette: &[[u8; 3]],
    frame_index: usize,
) -> RgbImage {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    let mut error_buffer: Vec<Vec<[f32; 3]>> =
        vec![vec![[0.0; 3]; width as usize]; height as usize];

    // Usa un seed deterministico basato sulla posizione e frame per coerenza
    let mut rng_seed = frame_index as u32;

    for y in 0..height {
        for x in 0..width {
            let pixel = rgb_img.get_pixel(x, y);
            let original = [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32];

            // Applica l'errore accumulato dal dithering interno al frame
            let error = error_buffer[y as usize][x as usize];
            let adjusted = [
                (original[0] + error[0]).clamp(0.0, 255.0),
                (original[1] + error[1]).clamp(0.0, 255.0),
                (original[2] + error[2]).clamp(0.0, 255.0),
            ];

            let adjusted_u8 = [adjusted[0] as u8, adjusted[1] as u8, adjusted[2] as u8];

            // Trova i due colori più vicini nella palette
            let mut distances: Vec<(u32, usize, [u8; 3])> = palette
                .iter()
                .enumerate()
                .map(|(idx, &color)| {
                    let dr = adjusted_u8[0] as i32 - color[0] as i32;
                    let dg = adjusted_u8[1] as i32 - color[1] as i32;
                    let db = adjusted_u8[2] as i32 - color[2] as i32;
                    let dist = (dr * dr + dg * dg + db * db) as u32;
                    (dist, idx, color)
                })
                .collect();

            distances.sort_by_key(|&(dist, _, _)| dist);
            let closest_color = distances[0].2;

            // Aggiungi un piccolo rumore deterministico per coerenza tra frame
            // ma solo per colori che sono molto vicini tra loro
            if distances.len() > 1 && distances[1].0 - distances[0].0 < 100 {
                // Usa una funzione hash semplice per determinismo
                rng_seed = rng_seed.wrapping_mul(1664525).wrapping_add(1013904223);
                let noise_threshold = (rng_seed % 100) as f32 / 100.0;

                // Se il rumore supera una certa soglia, usa il secondo colore più vicino
                if noise_threshold > 0.7 {
                    let second_closest = distances[1].2;
                    result.put_pixel(x, y, Rgb(second_closest));

                    // Calcola errore rispetto al secondo colore
                    let quant_error = [
                        adjusted[0] - second_closest[0] as f32,
                        adjusted[1] - second_closest[1] as f32,
                        adjusted[2] - second_closest[2] as f32,
                    ];

                    distribute_error(&mut error_buffer, x, y, width, height, quant_error);
                    continue;
                }
            }

            result.put_pixel(x, y, Rgb(closest_color));

            // Calcola l'errore di quantizzazione normale
            let quant_error = [
                adjusted[0] - closest_color[0] as f32,
                adjusted[1] - closest_color[1] as f32,
                adjusted[2] - closest_color[2] as f32,
            ];

            distribute_error(&mut error_buffer, x, y, width, height, quant_error);
        }
    }

    result
}

/// Distribuisce l'errore di quantizzazione usando l'algoritmo Floyd-Steinberg
fn distribute_error(
    error_buffer: &mut Vec<Vec<[f32; 3]>>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    quant_error: [f32; 3],
) {
    fn distribute(
        error_buffer: &mut Vec<Vec<[f32; 3]>>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        quant_error: [f32; 3],
        dx: i32,
        dy: i32,
        factor: f32,
    ) {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
            let nx = nx as usize;
            let ny = ny as usize;

            for i in 0..3 {
                error_buffer[ny][nx][i] += quant_error[i] * factor;
            }
        }
    }

    distribute(
        error_buffer,
        x,
        y,
        width,
        height,
        quant_error,
        1,
        0,
        7.0 / 16.0,
    );
    distribute(
        error_buffer,
        x,
        y,
        width,
        height,
        quant_error,
        -1,
        1,
        3.0 / 16.0,
    );
    distribute(
        error_buffer,
        x,
        y,
        width,
        height,
        quant_error,
        0,
        1,
        5.0 / 16.0,
    );
    distribute(
        error_buffer,
        x,
        y,
        width,
        height,
        quant_error,
        1,
        1,
        1.0 / 16.0,
    );
}

/// Processa un singolo file immagine e lo converte nella palette BytePusher
fn process_single_image(
    image_path: &Path,
    frame_index: usize,
    palette: &[[u8; 3]],
) -> Result<ProcessedFrame, BytePusherError> {
    let img = image::open(image_path)?;
    let dithered = apply_floyd_steinberg_dither(&img, palette, frame_index);

    let (width, height) = dithered.dimensions();
    let rgb_data = dithered.into_raw();
    let file_name = image_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    Ok(ProcessedFrame {
        rgb_data,
        width,
        height,
        frame_index,
        file_name,
    })
}

/// Funzione pubblica principale che processa una sequenza di immagini PNG
/// usando il pattern glob specificato e le converte nella palette BytePusher
///
/// # Argomenti
/// * `glob_pattern` - Pattern glob per trovare i file PNG (es. "frames/*.png")
///
/// # Ritorna
/// Un `Vec<ProcessedFrame>` contenente tutti i frame elaborati, ordinati per nome file
///
/// # Esempio
/// ```rust
/// use bytepusher_lib::process_png_sequence;
///
/// let frames = process_png_sequence("input/*.png")?;
/// for frame in frames {
///     println!("Frame {}: {}x{} pixels, {} bytes",
///              frame.frame_index, frame.width, frame.height, frame.rgb_data.len());
/// }
/// ```
pub fn process_png_sequence(glob_pattern: &str) -> Result<Vec<ProcessedFrame>, BytePusherError> {
    // Genera la palette BytePusher
    let palette = generate_bytepusher_palette();

    // Trova tutti i file che corrispondono al pattern
    let mut file_paths: Vec<_> = glob(glob_pattern)?.collect::<Result<Vec<_>, _>>()?;

    if file_paths.is_empty() {
        return Err(BytePusherError::NoFilesFound);
    }

    // Ordina i file per nome per garantire un ordine coerente
    file_paths.sort();

    let mut processed_frames = Vec::with_capacity(file_paths.len());

    // Processa ogni file
    for (i, path) in file_paths.iter().enumerate() {
        let frame = process_single_image(path, i, &palette)?;
        processed_frames.push(frame);
    }

    Ok(processed_frames)
}

/// Funzione di utilità che restituisce i dati RGB come un singolo Vec<u8>
/// concatenando tutti i frame in sequenza
///
/// # Argomenti
/// * `glob_pattern` - Pattern glob per trovare i file PNG
///
/// # Ritorna
/// Un `Vec<u8>` contenente tutti i dati RGB dei frame concatenati
pub fn process_png_sequence_flat(glob_pattern: &str) -> Result<Vec<u8>, BytePusherError> {
    let frames = process_png_sequence(glob_pattern)?;

    let total_size: usize = frames.iter().map(|f| f.rgb_data.len()).sum();
    let mut result = Vec::with_capacity(total_size);

    for frame in frames {
        result.extend(frame.rgb_data);
    }

    Ok(result)
}

/// Restituisce la palette BytePusher standard
pub fn get_bytepusher_palette() -> Vec<[u8; 3]> {
    generate_bytepusher_palette()
}

/// Salva i frame elaborati come file PNG
///
/// # Argomenti
/// * `frames` - Riferimento ai frame elaborati da salvare
/// * `output_dir` - Directory di destinazione per i file PNG
/// * `suffix` - Suffisso da aggiungere ai nomi dei file (es. "_processed")
///
/// # Ritorna
/// Un `Result` che indica il successo o il fallimento dell'operazione
pub fn save_processed_frames_as_png(
    frames: &[ProcessedFrame],
    output_dir: &str,
    suffix: &str,
) -> Result<(), BytePusherError> {
    use image::RgbImage;
    use std::fs;
    use std::path::PathBuf;

    fs::create_dir_all(output_dir)?;

    // Converti i dati RGB in indici di palette (u32), poi riconverti in RGB per salvataggio PNG
    let palette = generate_bytepusher_palette();

    for frame in frames {
        let mut output_name = frame.file_name.clone();
        if let Some(dot_pos) = output_name.rfind('.') {
            output_name.insert_str(dot_pos, suffix);
        } else {
            output_name.push_str(suffix);
        }
        let mut output_path = PathBuf::from(output_dir);
        output_path.push(output_name);

        let mut indexed_pixels = Vec::with_capacity((frame.width * frame.height) as usize);

        for chunk in frame.rgb_data.chunks(3) {
            let rgb = [chunk[0], chunk[1], chunk[2]];
            // Trova l'indice del colore più vicino nella palette
            let idx = palette.iter().position(|&c| c == rgb).unwrap_or(0) as u32;
            indexed_pixels.push(idx);
        }

        // Ricostruisci l'immagine RGB usando la palette e gli indici
        let mut rgb_pixels = Vec::with_capacity((frame.width * frame.height * 3) as usize);
        for &idx in &indexed_pixels {
            let color = palette.get(idx as usize).copied().unwrap_or([0, 0, 0]);
            rgb_pixels.extend_from_slice(&color);
        }

        let img = RgbImage::from_raw(frame.width, frame.height, rgb_pixels)
            .ok_or(BytePusherError::InvalidFormat)?;

        img.save(output_path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_generation() {
        let palette = generate_bytepusher_palette();
        assert_eq!(palette.len(), 216);

        // Verifica che i colori agli estremi siano corretti
        assert_eq!(palette[0], [0, 0, 0]); // Nero
        assert_eq!(palette[215], [255, 255, 255]); // Bianco

        // Verifica alcuni colori intermedi
        assert_eq!(palette[35], [255, 0, 0]); // Rosso puro
        assert_eq!(palette[210], [0, 255, 0]); // Verde puro  
    }

    #[test]
    fn test_processed_frame_structure() {
        let frame = ProcessedFrame {
            rgb_data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255],
            width: 3,
            height: 1,
            frame_index: 0,
            file_name: "test.png".to_string(),
        };

        assert_eq!(frame.rgb_data.len(), 9); // 3 pixel * 3 componenti RGB
        assert_eq!(frame.width, 3);
        assert_eq!(frame.height, 1);
        assert_eq!(frame.frame_index, 0);
        assert_eq!(frame.file_name, "test.png");
    }
}
