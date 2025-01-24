use std::error::Error;
use std::fs::File;
use std::io::{BufWriter};

pub fn resize_image_to_dimensions(
    input_path: &str,
    output_path: &str,
    new_width: u32,
    new_height: u32,
) -> Result<(), Box<dyn Error>> {
    // Wczytaj obraz
    let img = image::open(input_path)?;

    // Zmień rozmiar obrazu
    let resized_img = img.resize_exact(
        new_width,
        new_height,
        image::imageops::FilterType::Lanczos3, // Algorytm interpolacji
    );

    // Zapisz obraz do pliku
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    resized_img.write_to(&mut writer, image::ImageFormat::from_path(output_path)?)?;

    println!(
        "Zmieniono rozmiar: {} -> {}x{} i zapisano w {}",
        input_path, new_width, new_height, output_path
    );

    Ok(())
}
