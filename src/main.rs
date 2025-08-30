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
        println!("=== Starting {} method ===", dithering_type_to_string(method));
        
        // File dialog timing
        let dialog_start = Instant::now();
        println!("📂 Opening file dialog...");

        let path = rfd::FileDialog::new()
            .set_title("Choose wallpaper")
            .pick_file()
            .ok_or_else(|| anyhow!("No file selected via GUI"));

        println!("⏱️  File dialog completed in {:.2?}", dialog_start.elapsed());

        // Image loading timing
        let load_start = Instant::now();
        println!("📖 Reading image from disk...");
        
        let image = image::open(path.unwrap()).expect("Failed to open image");
        let load_time = load_start.elapsed();
        
        println!("⏱️  Image loaded in {:.2?} ({}x{} pixels)", 
                load_time, image.width(), image.height());

        // Dithering timing
        let dither_start = Instant::now();
        println!("🎨 Dithering image via {} method...", dithering_type_to_string(method));
        
        let dither_simple = dithering::Dithering { image };
        let dithered_image = dither_simple.apply(DitheringType::Simple);
        let dither_time = dither_start.elapsed();
        
        println!("⏱️  Dithering completed in {:.2?}", dither_time);

        // Save dialog timing
        let save_dialog_start = Instant::now();
        println!("💾 Opening save dialog...");
        
        let path = rfd::FileDialog::new()
            .set_title("Save image")
            .set_file_name("image.png")
            .add_filter("png", &["png"])
            .save_file()
            .ok_or_else(|| anyhow!("No file selected via save dialog"));

        println!("⏱️  Save dialog completed in {:.2?}", save_dialog_start.elapsed());

        // Image saving timing
        let save_start = Instant::now();
        println!("💿 Writing image to disk...");

        dithered_image
            .save(path.unwrap())
            .expect("Failed to save image");
        let save_time = save_start.elapsed();
        
        println!("⏱️  Image saved in {:.2?}", save_time);

        let method_total = program_instant.elapsed();
        println!("✅ {} method completed!", dithering_type_to_string(method));
        println!("📊 Performance breakdown:");
        println!("   • Loading: {:.2?}", load_time);
        println!("   • Dithering: {:.2?}", dither_time); 
        println!("   • Saving: {:.2?}", save_time);
        println!("   • Total: {:.2?}", method_total);
        println!("=== {} method finished ===\n", dithering_type_to_string(method));
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
