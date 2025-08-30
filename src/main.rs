use crate::dithering::DitheringType;
use anyhow::anyhow;
use std::time::Instant;

mod dithering;

fn main() {
    let program_instant = Instant::now();
    let title = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    println!("{}", title);

    let methods: Vec<DitheringType> = [DitheringType::Simple].to_vec();

    for method in methods {
        println!("reading image from disk");

        let path = rfd::FileDialog::new()
            .set_title("Choose wallpaper")
            .pick_file()
            .ok_or_else(|| anyhow!("No file selected via GUI"));

        let image = image::open(path.unwrap()).expect("Failed to open image");

        println!(
            "dithering image via {} method",
            dithering_type_to_string(method)
        );
        let dither_simple = dithering::Dithering { image };

        let dithered_image = dither_simple.apply(DitheringType::Simple);

        let path = rfd::FileDialog::new()
            .set_title("Save image")
            .set_file_name("image.png")
            .add_filter("png", &["png"])
            .save_file()
            .ok_or_else(|| anyhow!("No file selected via save dialog"));

        println!("writing image to disk");

        dithered_image
            .save(path.unwrap())
            .expect("Failed to save image");

        println!(
            "{} method done within {:.2?}",
            dithering_type_to_string(method),
            program_instant.elapsed()
        );
    }

    println!("Done within {:.2?}", program_instant.elapsed());
}

fn dithering_type_to_string(dithering_type: DitheringType) -> &'static str {
    match dithering_type {
        DitheringType::Simple => "simple",
        DitheringType::Ordered => "ordered",
        DitheringType::FloydSteinberg => "floyd-steinberg",
    }
}
