use image::{DynamicImage, GenericImageView, ImageDecoder, ImageEncoder, ImageFormat};
use std::fs::File;
use std::io::{BufWriter, Cursor, Write};
use std::error::Error;

pub fn resize_to_target_size(
    input_path: &str,
    output_path: &str,
    target_size_mb: f64,
) -> Result<(), Box<dyn Error>> {
    // Wczytaj obraz
    let mut img = image::open(input_path)?;

    // Przelicz docelowy rozmiar w bajtach
    let target_size_bytes = (target_size_mb * 1024.0 * 1024.0) as usize;

    // Początkowe parametry
    let mut quality = 90; // Początkowa jakość
    let mut resized_img = img.clone();

    loop {
        // Bufor na zakodowany obraz
        let mut buffer = Cursor::new(Vec::new());

        // Rozpoznaj format na podstawie rozszerzenia
        let extension = std::path::Path::new(output_path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_default();

        match extension {
            "jpg" | "jpeg" => encode_as_jpeg(&resized_img, &mut buffer, quality)?,
            "png" => encode_as_png(&resized_img, &mut buffer)?,
            "gif" => encode_as_gif(&resized_img, &mut buffer)?,
            "bmp" => encode_as_bmp(&resized_img, &mut buffer)?,
            "tiff" | "tif" => encode_as_tiff(&resized_img, &mut buffer)?,
            "ico" => encode_as_ico(&resized_img, &mut buffer)?,
            _ => return Err(format!("Nieobsługiwany format: {}", extension).into()),
        }

        // Sprawdź rozmiar
        let current_size = buffer.get_ref().len();

        if current_size <= target_size_bytes || quality <= 10 {
            // Zapisz do pliku
            let mut output = BufWriter::new(File::create(output_path)?);
            output.write_all(buffer.get_ref())?;

            println!(
                "Osiągnięto rozmiar: {:.2} MB przy jakości {}",
                current_size as f64 / (1024.0 * 1024.0),
                quality
            );
            break;
        }

        // Jeśli rozmiar za duży, zmniejsz jakość
        quality -= 5;

        // Jeśli jakość jest niska, zmniejsz rozdzielczość
        if quality <= 10 {
            let (width, height) = resized_img.dimensions();
            resized_img = img.resize(
                (width as f32 * 0.9) as u32,
                (height as f32 * 0.9) as u32,
                image::imageops::FilterType::Lanczos3,
            );
        }
    }

    Ok(())
}

// Funkcje do kodowania obrazów w różnych formatach

fn encode_as_jpeg(
    img: &DynamicImage,
    buffer: &mut Cursor<Vec<u8>>,
    quality: u8,
) -> Result<(), Box<dyn Error>> {
    let rgb_image = img.to_rgb8(); // JPEG nie obsługuje przezroczystości
    let mut encoder = jpeg_encoder::Encoder::new(buffer, quality);
    encoder.encode(
        &rgb_image,
        rgb_image.width() as u16,
        rgb_image.height() as u16,
        jpeg_encoder::ColorType::Rgb,
    )?;
    Ok(())
}

fn encode_as_png(
    img: &DynamicImage,
    buffer: &mut Cursor<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let encoder = image::codecs::png::PngEncoder::new(buffer);
    let rgba_image = img.to_rgba8(); // Zachowanie przezroczystości
    encoder.write_image(
        &rgba_image,
        rgba_image.width(),
        rgba_image.height(),
        image::ExtendedColorType::Rgba8,
    )?;
    Ok(())
}

fn encode_as_gif(
    img: &DynamicImage,
    buffer: &mut Cursor<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let mut encoder = image::codecs::gif::GifEncoder::new(buffer);
    let rgba_image = img.to_rgba8();
    encoder.encode(
        rgba_image.as_raw(),
        rgba_image.width(),
        rgba_image.height(),
        image::ExtendedColorType::Rgba8,
    )?;
    Ok(())
}

fn encode_as_bmp(
    img: &DynamicImage,
    buffer: &mut Cursor<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let encoder = image::codecs::bmp::BmpEncoder::new(buffer);
    let rgb_image = img.to_rgb8();
    encoder.write_image(
        &rgb_image,
        rgb_image.width(),
        rgb_image.height(),
        image::ExtendedColorType::Rgb8,
    )?;
    Ok(())
}

fn encode_as_tiff(
    img: &DynamicImage,
    buffer: &mut Cursor<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let encoder = image::codecs::tiff::TiffEncoder::new(buffer);
    let rgba_image = img.to_rgba8(); // TIFF może obsługiwać przezroczystość
    encoder.write_image(
        &rgba_image,
        rgba_image.width(),
        rgba_image.height(),
        image::ExtendedColorType::Rgba8,
    )?;
    Ok(())
}

fn encode_as_ico(
    img: &DynamicImage,
    buffer: &mut Cursor<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    let encoder = image::codecs::ico::IcoEncoder::new(buffer);
    let rgba_image = img.to_rgba8();
    encoder.write_image(
        &rgba_image,
        rgba_image.width(),
        rgba_image.height(),
        image::ExtendedColorType::Rgba8,
    )?;
    Ok(())
}
