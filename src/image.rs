use image::Rgb;

use crate::error::BytePusherError;

pub fn build_palette() -> Vec<Rgb<u8>> {
    let mut palette = Vec::with_capacity(216);
    for r in 0..6 {
        for g in 0..6 {
            for b in 0..6 {
                let red = (r * 255 / 5) as u8;
                let green = (g * 255 / 5) as u8;
                let blue = (b * 255 / 5) as u8;
                palette.push(Rgb([red, green, blue]));
            }
        }
    }
    palette
}

/// Trova l'indice del colore più vicino nella palette
fn find_closest_color(pixel: Rgb<u8>, palette: &Vec<Rgb<u8>>) -> u8 {
    let mut min_distance = f64::INFINITY;
    let mut closest_index = 0;
    for (index, palette_color) in palette.iter().enumerate() {
        let distance = color_distance(pixel, *palette_color);
        if distance < min_distance {
            min_distance = distance;
            closest_index = index;
        }
    }
    closest_index as u8
}

/// Calcola la distanza euclidea tra due colori nello spazio RGB
fn color_distance(color1: Rgb<u8>, color2: Rgb<u8>) -> f64 {
    let dr = color1[0] as f64 - color2[0] as f64;
    let dg = color1[1] as f64 - color2[1] as f64;
    let db = color1[2] as f64 - color2[2] as f64;
    (dr * dr + dg * dg + db * db).sqrt()
}

/// Converte un'immagine RGB in formato BytePusher usando il dithering di Floyd–Steinberg, con forza regolabile
pub fn convert_image_dithered_strength(
    image_path: &str,
    dither_strength: f32,
) -> Result<Vec<u8>, BytePusherError> {
    let palette = build_palette();

    let mut img = image::open(image_path)?.to_rgb8();
    if img.width() != 256 || img.height() != 256 {
        img = image::imageops::resize(&img, 256, 256, image::imageops::FilterType::Lanczos3);
    }

    let (width, height) = img.dimensions();
    let buffer = img.clone();
    let mut result = Vec::with_capacity((width * height) as usize);
    let mut error = vec![[0f32; 3]; (width * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let mut old_pixel = buffer.get_pixel(x, y).0;
            // Aggiungi l'errore accumulato
            for c in 0..3 {
                let val = old_pixel[c] as f32 + error[idx][c];
                old_pixel[c] = val.clamp(0.0, 255.0) as u8;
            }
            let quant_idx = find_closest_color(Rgb(old_pixel), &palette);
            let quant_pixel = palette[quant_idx as usize].0;
            result.push(quant_idx);
            // Calcola l'errore di quantizzazione
            let mut quant_error = [0f32; 3];
            for c in 0..3 {
                quant_error[c] = (old_pixel[c] as f32 - quant_pixel[c] as f32) * dither_strength;
            }
            // Diffondi l'errore ai pixel vicini (Floyd–Steinberg)
            // x+1, y    (7/16)
            if x + 1 < width {
                let nidx = (y * width + (x + 1)) as usize;
                for c in 0..3 {
                    error[nidx][c] += quant_error[c] * 7.0 / 16.0;
                }
            }
            // x-1, y+1  (3/16)
            if x > 0 && y + 1 < height {
                let nidx = ((y + 1) * width + (x - 1)) as usize;
                for c in 0..3 {
                    error[nidx][c] += quant_error[c] * 3.0 / 16.0;
                }
            }
            // x, y+1    (5/16)
            if y + 1 < height {
                let nidx = ((y + 1) * width + x) as usize;
                for c in 0..3 {
                    error[nidx][c] += quant_error[c] * 5.0 / 16.0;
                }
            }
            // x+1, y+1  (1/16)
            if x + 1 < width && y + 1 < height {
                let nidx = ((y + 1) * width + (x + 1)) as usize;
                for c in 0..3 {
                    error[nidx][c] += quant_error[c] * 1.0 / 16.0;
                }
            }
        }
    }
    Ok(result)
}
