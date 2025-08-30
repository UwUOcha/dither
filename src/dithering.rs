use image::DynamicImage;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DitheringType {
    Simple,
    Ordered,
    FloydSteinberg,
}

pub struct Dithering {
    pub(crate) image: DynamicImage,
}

impl Dithering {
    pub fn apply(&self, dithering_type: DitheringType) -> DynamicImage {
        match dithering_type {
            DitheringType::Simple => simple(&self.image),
            DitheringType::Ordered => ordered(&self.image),
            DitheringType::FloydSteinberg => floyd_steinberg(&self.image),
        }
    }
}

fn simple(image: &DynamicImage) -> DynamicImage {
    let width = image.width();
    let height = image.height();

    // Convert to RGBA8 for consistent pixel format
    let rgba_image = image.to_rgba8();
    let input_pixels = rgba_image.as_raw();

    // Process pixels in parallel chunks of 4 bytes (RGBA)
    let output_pixels: Vec<u8> = input_pixels
        .par_chunks_exact(4)
        .flat_map(|chunk| {
            let r = chunk[0] as u16;
            let g = chunk[1] as u16;
            let b = chunk[2] as u16;
            let a = chunk[3];

            // Calculate luminance using perceptually accurate formula
            // Using bit shifts for speed: (r * 77 + g * 150 + b * 29) >> 8
            let luminance = ((r * 77 + g * 150 + b * 29) >> 8) as u8;

            if luminance > 127 {
                [255u8, 255u8, 255u8, a]
            } else {
                [0u8, 0u8, 0u8, a]
            }
        })
        .collect();

    // Create new image from raw pixel data
    let output_image = image::RgbaImage::from_raw(width, height, output_pixels)
        .expect("Failed to create image from raw data");

    DynamicImage::ImageRgba8(output_image)
}

fn ordered(image: &DynamicImage) -> DynamicImage {
    // TODO: Ordered dithering logic
    image.clone()
}

fn floyd_steinberg(image: &DynamicImage) -> DynamicImage {
    // TODO: Floyd-Steinber dithering logic
    image.clone()
}
