use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub  enum DitheringType {
    Simple,
    Ordered,
    FloydSteinberg,
}

pub struct Dithering {
    pub(crate) image: DynamicImage
}

impl Dithering {
    pub  fn apply(&self, dithering_type: DitheringType) -> DynamicImage {
        match dithering_type {
            DitheringType::Simple => simple(&self.image),
            DitheringType::Ordered => ordered(&self.image),
            DitheringType::FloydSteinberg => floyd_steinberg(&self.image),
        }
    }
}

fn simple(image: &DynamicImage) -> DynamicImage {
    let mut new_image = image.clone();

    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            let sum = pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16;
            let avg = (sum / 3) as u8;

            if avg > 127 {
                new_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            } else {
                new_image.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }
    }

    new_image
}

fn ordered(image: &DynamicImage) -> DynamicImage {
    // TODO: логика Ordered dithering
    return image.clone();
}

fn floyd_steinberg(image: &DynamicImage) -> DynamicImage {
    // TODO: логика FS dithering
    return image.clone();
}