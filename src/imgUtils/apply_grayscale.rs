use image::open;

fn apply_grayscale(image_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = open(image_path)?;

    let gray_img = img.to_luma8();
    gray_img.save(output_path)?;

    Ok(())
}