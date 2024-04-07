use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use image::{ImageBuffer, Rgba};

mod utils;

#[derive(Parser, Debug)]
struct Motifier {
    #[arg(short = 'm', long = "motif")]
    motif: PathBuf,
    #[arg(short = 'b', long = "block-size")]
    block_size: u32,
    #[arg(short = 'r', long = "rows")]
    rows: u32,
    #[arg(short = 'c', long = "columns")]
    columns: u32,
    #[arg(short = 'l', long = "layout", num_args = 1.., value_delimiter = ' ')]
    layout: Vec<String>,
    #[arg(short = 'o', long = "output")]
    output: PathBuf,
}

impl Motifier {
    fn execute(&self) -> Result<()> {

        if self.block_size * self.block_size != self.layout.len() as u32 {
            return Err(anyhow::anyhow!("Layout size does not match the motif size"));
        }

        let motif = image::open(&self.motif)?.to_rgba16();
        let (width, height) = motif.dimensions();

        let mut block_image: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(width * self.block_size, height * self.block_size);

        for outer_x in 0..self.block_size {
            for outer_y in 0..self.block_size {
                let oriented_motif: ImageBuffer<Rgba<u16>, Vec<u16>> = {
                    let key = self.layout[(outer_x * self.block_size + outer_y) as usize].clone();
                    utils::orient_image(&motif, key)?
                };

                for block_x in 0..width {
                    for block_y in 0..height {
                        let x = outer_x * width + block_x;
                        let y = outer_y * height + block_y;
                        block_image.put_pixel(x, y, *oriented_motif.get_pixel(block_x, block_y));
                    }
                }
            }
        }
        
        let mut full_image: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(
            width * self.block_size * self.columns, 
            height * self.block_size * self.rows
        );

        for row in 0..self.rows {
            for column in 0..self.columns {
                for x in 0..width * self.block_size {
                    for y in 0..height * self.block_size {
                        let x = column * width * self.block_size + x;
                        let y = row * height * self.block_size + y;
                        full_image.put_pixel(x, y, *block_image.get_pixel(x % (width * self.block_size), y % (height * self.block_size)));
                    }
                }
            }
        }
        full_image.save(&self.output)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Motifier::parse();
    args.execute()?;
    Ok(())
}
