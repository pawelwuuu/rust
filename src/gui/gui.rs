use crate::file_operations::fileOperations;
use crate::imgUtils::apply_grayscale::apply_grayscale;
use crate::imgUtils::image_dimensions_resize::resize_image_to_dimensions;
use crate::imgUtils::image_resize::resize_to_target_size;
use crate::imgUtils::remove_metadata::remove_metadata;
use eframe::epaint::TextureHandle;
use image::{DynamicImage, ImageFormat};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use egui::{Color32, TextEdit, Vec2};
use crate::imgUtils::is_img_duplicate::are_images_duplicates;

pub struct MyApp {
    resize_target: String,
    resize_height: String,
    resize_width: String,
    folder_path: String,
    duplicates: Option<Vec<Vec<String>>>,
    folder_files: HashMap<String, File>,        // Pliki wczytane z folderu
    loaded_images: HashMap<String, DynamicImage>, // Wczytane obrazy
    selected_images: HashMap<String, bool>, //pliki wybrane checboxem
    thumbnails: HashMap<String, TextureHandle>, // Wygenerowane miniaturki
    thumbnails_ready: bool, // Flaga informująca, czy miniaturki zostały wygenerowane
    status_message: Option<String>, // Komunikat o stanie operacji
}

impl MyApp {
    fn change_size_dimensions(&mut self) {
        let width = match self.resize_width.parse::<u32>() {
            Ok(w) => w,
            Err(_) => {
                self.status_message = Some("Nie udało się przekonwertować szerokości.".to_string());
                return;
            }
        };
        let height = match self.resize_height.parse::<u32>() {
            Ok(h) => h,
            Err(_) => {
                self.status_message = Some("Nie udało się przekonwertować wysokości.".to_string());
                return;
            }
        };

        for filename in self.selected_images.keys() {
            if self.selected_images[filename] {
                if let Err(e) = resize_image_to_dimensions(filename, filename, width, height) {
                    self.status_message = Some(format!("Błąd podczas zmiany rozmiaru obrazu {}: {}", filename, e));
                    return;
                }
            }
        }

        self.status_message = Some("Obrazy zostały zmienione na nowe wymiary.".to_string());
    }

    fn resize_to_target(&mut self) {
        let target_size = match self.resize_target.parse::<f64>() {
            Ok(t) => t,
            Err(_) => {
                self.status_message = Some("Nie udało się przekonwertować rozmiaru docelowego.".to_string());
                return;
            }
        };

        for filename in self.selected_images.keys() {
            if self.selected_images[filename] {
                if let Err(e) = resize_to_target_size(filename, filename, target_size) {
                    self.status_message = Some(format!("Błąd podczas kompresji obrazu {}: {}", filename, e));
                    return;
                }
            }
        }

        self.status_message = Some("Obrazy zostały skompresowane do docelowego rozmiaru.".to_string());
    }

    fn select_all(&mut self) {
        for selected in self.selected_images.values_mut() {
            *selected = true;
        }
    }

    fn deselect_all(&mut self) {
        for selected in self.selected_images.values_mut() {
            *selected = false;
        }
    }

    fn load_files(&mut self) {
        self.selected_images.clear();
        self.loaded_images.clear();
        self.thumbnails.clear();
        self.folder_files.clear();

        let files = match fileOperations::get_workspace_files(&self.folder_path) {
            Ok(f) => f,
            Err(e) => {
                self.status_message = Some(format!("Błąd podczas wczytywania plików: {}", e));
                return;
            }
        };
        self.folder_files = files;

        for (filename, file) in &self.folder_files {
            let mut file = file.try_clone().expect("Nie udało się sklonować pliku");
            let mut buffer = Vec::new();
            if let Err(err) = file.read_to_end(&mut buffer) {
                eprintln!("Błąd odczytu pliku {}: {:?}", filename, err);
                continue;
            }

            let format = match image::guess_format(&buffer) {
                Ok(f) => f,
                Err(_) => {
                    eprintln!("Nieznany format pliku: {}", filename);
                    continue;
                }
            };

            if matches!(format, ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::Gif | ImageFormat::Bmp | ImageFormat::Tiff) {
                match image::load_from_memory(&buffer) {
                    Ok(image) => {
                        self.loaded_images.insert(filename.clone(), image);
                    }
                    Err(err) => {
                        eprintln!("Nie udało się załadować obrazu {}: {:?}", filename, err);
                    }
                }
            } else {
                eprintln!("Format obrazu {} jest nieobsługiwany: {:?}", filename, format);
            }
        }

        for filename in self.folder_files.keys() {
            self.selected_images.insert(filename.clone(), false);
        }

        self.thumbnails_ready = false;

        self.status_message = Some("Pliki zostały wczytane.".to_string());
    }

    fn create_thumbnails(&mut self, ctx: &egui::Context, width: u32, height: u32) {
        if self.thumbnails_ready {
            return;
        }

        for (filename, image) in &self.loaded_images {
            let thumbnail = image.thumbnail(width, height);
            let rgba_image = thumbnail.to_rgba8();

            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [rgba_image.width() as usize, rgba_image.height() as usize],
                &rgba_image.into_raw(),
            );

            let texture = ctx.load_texture(
                filename,
                eframe::epaint::ImageData::Color(Arc::from(color_image)),
                eframe::epaint::textures::TextureOptions::default(),
            );

            self.thumbnails.insert(filename.clone(), texture);
        }

        self.thumbnails_ready = true;
    }

    fn set_grayscale(&mut self) {
        for filename in self.selected_images.keys() {
            if self.selected_images[filename] {
                if let Err(e) = apply_grayscale(filename, filename) {
                    self.status_message = Some(format!("Błąd podczas nadawania skali szarości obrazowi {}: {}", filename, e));
                    return;
                }
            }
        }

        self.status_message = Some("Skala szarości została zastosowana do wybranych obrazów.".to_string());
    }

    fn find_duplicates(&mut self) {
        let mut duplicates: Vec<Vec<String>> = Vec::new();
        let mut omit_list: std::collections::HashSet<String> = std::collections::HashSet::new();

        let mut to_compare : Vec<&String> = Vec::new();
        for filename in self.selected_images.keys() {
            if self.selected_images[filename] {
                to_compare.push(filename);
            }
        }

        if (to_compare.is_empty()) {
            self.status_message = Option::from(String::from("Żadne ze zdjęć nie zostało wybrane"));
            return;
        }

        for i in 0..to_compare.len() {
            if omit_list.contains(to_compare[i]) {
                continue;
            }

            let mut current_duplicate: Vec<String> = Vec::new();
            current_duplicate.push(to_compare[i].clone());

            for j in i + 1..to_compare.len() {
                if omit_list.contains(to_compare[j]) {
                    continue;
                }

                if are_images_duplicates(to_compare[i], to_compare[j]).unwrap() {
                    current_duplicate.push(to_compare[j].clone());
                    omit_list.insert(to_compare[j].clone());
                }
            }

            if current_duplicate.len() > 1 {
                duplicates.push(current_duplicate);
                omit_list.insert(to_compare[i].clone());
            }
        }

        self.duplicates = Some(duplicates);
    }


    fn delete_metadata(&mut self) {
        for filename in self.selected_images.keys() {
            if self.selected_images[filename] {
                if let Err(e) = remove_metadata(&self.loaded_images[filename], filename) {
                    self.status_message = Some(format!("Błąd podczas usuwania metadanych z obrazu {}: {}", filename, e));
                    return;
                }
            }
        }

        self.status_message = Some("Metadane zostały usunięte z wybranych obrazów.".to_string());
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            duplicates: None,
            resize_width: String::new(),
            status_message: None,
            resize_height: String::new(),
            resize_target: String::new(),
            folder_path: String::from("/home/pawe-wojcik/img"),
            folder_files: HashMap::new(),
            loaded_images: HashMap::new(),
            selected_images: HashMap::new(),
            thumbnails: HashMap::new(),
            thumbnails_ready: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Wpisz ścieżkę do folderu:");
                ui.add(TextEdit::singleline(&mut self.folder_path).hint_text("/home/user/images"));
                if ui.button("Wczytaj dane").clicked() {
                    self.load_files();
                }

                ui.add_space(30.0);
                if ui.button("Zaznacz wszystkie").clicked() {
                    self.select_all();
                }

                if ui.button("Odznacz wszystkie").clicked() {
                    self.deselect_all();
                }

            });

            ui.horizontal(|ui| {
                if ui.button("Nadaj skale szarości").clicked() {
                    self.set_grayscale();
                }

                if ui.button("Usuń metadane").clicked() {
                    self.delete_metadata();
                }
                if ui.button("Znajdz duplikaty").clicked() {
                    self.find_duplicates();
                }


                ui.add_space(25.0);

                ui.label("Skompresuj do MB");
                ui.add_sized(Vec2::new(80.0, 20.0), TextEdit::singleline(&mut self.resize_target).hint_text("3.2"));
                if ui.button("Skompresuj").clicked() {
                    self.resize_to_target();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Skompresuj obraz do wymiarów");
                ui.label("Wysokosc");
                ui.add_sized(Vec2::new(80.0, 20.0), TextEdit::singleline(&mut self.resize_height).hint_text("100"));
                ui.label("Szerokosc");
                ui.add_sized(Vec2::new(80.0, 20.0), TextEdit::singleline(&mut self.resize_width).hint_text("120"));
                if ui.button("Skompresuj").clicked() {
                    self.change_size_dimensions();
                }
            });


            if let Some(message) = &self.status_message {
                ui.add_space(10.0);
                ui.label(egui::RichText::new(message).color(Color32::YELLOW));
                ui.add_space(20.0);
            }

            if let Some(duplicates) = &self.duplicates {
                for group in duplicates {
                    ui.group(|ui| {
                        ui.label("Grupa duplikatów:");
                        for (index, image_path) in group.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}. {}", index + 1, image_path));
                            });
                        }
                    });
                }

                if ui.button("Zamknij").clicked() {
                    self.duplicates = None;
                }
            }

            self.create_thumbnails(ctx, 100, 100);

            ui.horizontal_wrapped(|ui| {
                for (filename, texture_id) in &self.thumbnails {
                    ui.image(texture_id);

                    ui.vertical(|ui| {
                        ui.label(Path::new(filename).file_name().unwrap().to_str().unwrap());
                        if (ui.checkbox( &mut self.selected_images[filename].clone(), "Zaznacz").clicked()) {
                            self.selected_images.insert(filename.clone(), !self.selected_images[filename]);
                        }
                    });

                    ui.add_space(20.0);
                }
            });
        });
    }
}