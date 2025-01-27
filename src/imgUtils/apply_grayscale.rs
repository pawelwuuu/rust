use image::open;

pub fn apply_grayscale(image_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if output_path.ends_with(".gif") {
        Err("Nie można nadać skali szarości do gifa")?
    }

    let img = open(image_path)?;

    let gray_img = img.to_luma8();
    gray_img.save(output_path)?;

    Ok(())
}