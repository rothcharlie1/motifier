use anyhow::Result;
use image::{Rgba, ImageBuffer};

pub fn orient_image(motif: &ImageBuffer<Rgba<u16>, Vec<u16>>, layout: String) -> Result<ImageBuffer<Rgba<u16>, Vec<u16>>> {
    let mut output_image = motif.clone();
    match layout.as_str() {
        "one" => {}
        "two" => {
            output_image = image::imageops::rotate90(&output_image);
        }
        "three" => {
            output_image = image::imageops::rotate180(&output_image);
        }
        "four" => {
            output_image = image::imageops::rotate270(&output_image);
        }
        "Uone" => {
            output_image = image::imageops::flip_horizontal(&output_image);
        }
        "Utwo" => {
            output_image = image::imageops::rotate90(&output_image);
            output_image = image::imageops::flip_horizontal(&output_image);
        }
        "Uthree" => {
            output_image = image::imageops::rotate180(&output_image);
            output_image = image::imageops::flip_horizontal(&output_image);
        }
        "Ufour" => {
            output_image = image::imageops::rotate270(&output_image);
            output_image = image::imageops::flip_horizontal(&output_image);
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown orientation key"));
        }
    }
    Ok(output_image)
}