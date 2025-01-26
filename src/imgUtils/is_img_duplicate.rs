use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};

fn compute_image_hash(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(image_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

pub fn are_images_duplicates(image1_path: &str, image2_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let hash1 = compute_image_hash(image1_path)?;
    let hash2 = compute_image_hash(image2_path)?;
    Ok(hash1 == hash2)
}