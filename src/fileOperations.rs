use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

const IMAGE_EXTENSIONS: [&str; 2] = [".png", ".jpg"];


fn get_file_paths_from_folder(path: &str) -> Vec<PathBuf> {
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

fn filter_images_from_folder(paths: Vec<PathBuf>) -> Vec<String> {
    let mut filtered_paths: Vec<String> = Vec::new();

    for path in paths {
        let path_str = path.into_os_string().into_string().unwrap();

        for imageExtension in IMAGE_EXTENSIONS.iter() {
            if path_str.contains(imageExtension) {
                filtered_paths.push(path_str.clone());
                break;
            }
        }
    }

    filtered_paths
}

fn load_files_from_folder(paths: Vec<String>) -> HashMap<String, File> {
    let mut files: HashMap<String, File> = HashMap::new();

    for path in paths {
        files.insert(path.clone(), File::open(path.clone()).unwrap());
    }

    files
}

pub fn get_workspace_files(path: &str) -> HashMap<String, File> {
    load_files_from_folder(filter_images_from_folder(get_file_paths_from_folder(path)))
}
