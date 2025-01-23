use std::fs;
use std::path::PathBuf;

const IMAGE_EXTENSIONS: [&str; 2] = [".png", ".jpg"];


pub fn get_file_paths_from_folder(path: &str) -> Vec<PathBuf> {
    let mut file_paths = Vec::new();

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        match path {
            Ok(path) => {
                file_paths.push(path.path());
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    file_paths
}

pub fn filter_images_from_folder(paths: Vec<PathBuf>) -> Vec<String> {
    let mut filteredPaths: Vec<String> = Vec::new();

    for path in paths {
        let path_str = path.into_os_string().into_string().unwrap();

        for imageExtension in IMAGE_EXTENSIONS.iter() {
            if path_str.contains(imageExtension) {
                filteredPaths.push(path_str.clone());
                break;
            }
        }
    }

    filteredPaths
}