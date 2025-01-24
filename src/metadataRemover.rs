use image::{DynamicImage, ImageError};

pub fn remove_metadata(img: &DynamicImage, output_path: &str) -> Result<(), ImageError> {
    img.save(output_path)?;
    let clean_img = image::open(output_path)?;
    clean_img.save(output_path)?;
    Ok(())
}