use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, Read};

const IMAGE_EXTENSIONS: [&str; 3] = [".png", ".jpg", ".jpeg"];

fn get_file_paths_from_folder(path: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut file_paths = Vec::new();

    let paths = fs::read_dir(path)?;
    for path in paths {
        match path {
            Ok(entry) => {
                file_paths.push(entry.path());
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    Ok(file_paths)
}

fn filter_images_from_folder(paths: Vec<PathBuf>) -> Vec<String> {
    let mut filtered_paths: Vec<String> = Vec::new();

    for path in paths {
        let path_str = path.into_os_string().into_string().unwrap();

        for image_extension in IMAGE_EXTENSIONS.iter() {
            if path_str.contains(image_extension) {
                filtered_paths.push(path_str.clone());
                break;
            }
        }
    }

    filtered_paths
}

fn load_files_from_folder(paths: Vec<String>) -> Result<HashMap<String, File>, io::Error> {
    let mut files: HashMap<String, File> = HashMap::new();

    for path in paths {
        let file = File::open(&path)?;
        files.insert(path.clone(), file);
    }

    Ok(files)
}

pub fn get_workspace_files(path: &str) -> Result<HashMap<String, File>, io::Error> {
    let file_paths = get_file_paths_from_folder(path)?;
    let filtered_paths = filter_images_from_folder(file_paths);
    let files = load_files_from_folder(filtered_paths)?;

    Ok(files)
}
