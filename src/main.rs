use std::fs;
use std::path::Path;
use image::{ImageError, ImageFormat};
use glob::glob;
use clap::Parser;

fn resize_image(input_path: &Path, output_path: &Path, width: u32, height: u32) -> Result<(), ImageError> {
    let img = image::open(input_path)?;
    let thumbnail = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    thumbnail.save(output_path)?;
    Ok(())
}

fn convert_webp_to_png(input_path: &Path, output_path: &Path) -> Result<(), ImageError> {
    let img = image::open(input_path)?;
    img.save_with_format(output_path, ImageFormat::Png)?;
    Ok(())
}

#[derive(Parser)]
#[command(name = "resize", version, about)]
struct Cli {
    #[arg(short = 'i', long = "input", default_value = "./images")]
    input_dir: String,
    #[arg(short = 'o', long = "output", default_value = "./thumbnails")]
    output_dir: String,
    #[arg(short = 'W', long = "width", default_value = "200")]
    width: u32,
    #[arg(short = 'H', long = "height", default_value = "200")]
    height: u32,
    #[arg(short = 'e', long = "extension", default_value = ".jpg")]
    extension: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let input_dir = cli.input_dir;
    let output_dir = cli.output_dir;
    let thumbnail_width: u32 = cli.width;
    let thumbnail_height: u32 = cli.height;
    let extension = cli.extension;

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Use glob to iterate over all image files in the input directory by extension
    let pattern = format!("{}/*{}", input_dir, extension);

    for entry in glob(&pattern)? {
        match entry {
            Ok(path) => {
                let extension = path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("");
                let output_path = Path::new(&output_dir).join(path.file_stem().unwrap()).with_extension(&extension);

                if extension == "webp" {
                    match convert_webp_to_png(&path, &output_path) {
                        Ok(_) => println!("Converted {} to PNG", path.display()),
                        Err(e) => eprintln!("Failed to convert {}: {}", path.display(), e),
                    }
                }

                match resize_image(&path, &output_path, thumbnail_width, thumbnail_height) {
                    Ok(_) => println!("Resized image saved to {:?}", output_path),
                    Err(e) => eprintln!("Failed to resize image {}: {}", path.display(), e),
                }
            }
            Err(e) => eprintln!("Failed to read entry: {}", e),
        }
    }

    Ok(())
}

