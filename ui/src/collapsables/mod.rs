use std::{collections::HashSet};
use std::cell::Cell;
use std::path::{Path, PathBuf};
use gm_helper_corelibrary::TtrpgEntity;
use eframe::egui::{Vec2, Ui, ComboBox, Button, Label, Window, Context, ScrollArea, Response};
use sqlite::{Connection};
//TODO new ttrpg_entity 
// returns the ui height and width as a egui::Vec2 in order to calculate ui sizes
pub fn configuration_ui(ui: &mut Ui, ttrpgs: &mut Vec<TtrpgEntity>, new_database: &mut Cell<String>, new_ttrpg: &mut Cell<TtrpgEntity>) -> Vec2 { // Select database and load elements
    let config_ui = ui.group(|ui| {
            ui.group(|ui|{
                if ui.button("Create database").clicked() {
                    let dummy_ttrpg = TtrpgEntity::new(false, None, "dummy".to_string(), Some(new_database.get_mut()));
                    let (db_string, string_len) = (new_database.get_mut().clone(), new_database.get_mut().clone().len());
                    // Create the database as long under condition checks:
                    // cannot contain whitespace, must be alphabetic,
                    // and between the lengths of 0 to 50
                    if !db_string.contains(char::is_whitespace) &&
                    db_string.contains(char::is_alphabetic) && 
                    string_len > 0 &&
                    string_len < 50 &&
                    !dummy_ttrpg.database.is_file() {
                        let connection = Connection::open(dummy_ttrpg.database.as_os_str()).expect("Database creation failed!");
                        let query = "
                            CREATE table ttrpgs (
                                id INTEGER PRIMARY KEY AUTOINCREMENT,
                                date DATETIME DEFAULT CURRENT_TIMESTAMP,
                                json_string TEXT NOT NULL
                            );
                        ";
                        connection.execute(query).expect("SQL query failure; could not create database...");
                    }
                }
                ui.text_edit_singleline(new_database.get_mut())
            });
        
        new_ttrpg.get_mut().active.set(true);
        if new_ttrpg.get_mut().active.get() == true {
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Create TTRPG!").clicked() {
                        if new_ttrpg.get_mut().name.clone().len() > 0 {
                            //Create a new copy of dummy value to pass user defined name into active ttrpgs
                            let new_ttrpg_element = TtrpgEntity::new(true, None, new_ttrpg.get_mut().name.clone().to_string(), None);
                            ttrpgs.push(new_ttrpg_element);
                            new_ttrpg.get_mut().active.set(false);
                            new_ttrpg.get_mut().name = "".to_string();
                        }
                        new_ttrpg.get_mut().active.set(false);
                    }
                    ui.text_edit_singleline(&mut new_ttrpg.get_mut().name);
                });
        }
    });
        
    config_ui.response.rect.size()
}

pub fn selected_ttrpg_elements(ui: &mut Ui, ttrpgs: &mut Vec<TtrpgEntity>) -> Vec2 {
    let mut ttrpg_without_databases: u32 = 0;
    for ttrpg in ttrpgs.iter() {
        if ttrpg.database.as_os_str().len() == 12 { // 12 is the length of the default path string
            ttrpg_without_databases += 1;
        }
    }
    let existing_paths = std::fs::read_dir("./saved_dbs/").unwrap();
    let mut paths = Vec::new();
    for p in existing_paths {
        paths.push(p.unwrap().path().display().to_string());
    }
    let mut dbs_to_delete: Vec<String> = Vec::new();
    let selected_ttrpg_ui = ui.group(|ui| {
        ui.strong(format!("Number of ttrpg entities: {}", ttrpgs.len()));
        ui.strong(format!("Number of ttrpg entities without chosen databases: {}", ttrpg_without_databases));
        ScrollArea::vertical().show(ui, |ui| {
            for ttrpg in ttrpgs.iter_mut() {
                ui.group(|ui| {
                    ui.strong(ttrpg.name.clone());
                    ui.horizontal_wrapped(|ui| {
                        let active_text = if ttrpg.active.get() {"Active"} else {"Not Active"};
                        ui.checkbox(ttrpg.active.get_mut(), active_text);
                    });
                    ui.label(format!("Number of elements {}", ttrpg.elements.len()));
                    // Database selection and creation per ttrpg element
                    ui.horizontal(|ui| {
                        // Select the Database this ttrpg should save to
                        if ttrpg.database.as_os_str().is_empty() {
                            ui.label("No database to save to!");
                        } else {
                            ui.label("Selected database: ");
                        }
                        ComboBox::from_id_source("ttrpg_db_selection")
                            .selected_text(ttrpg.database.as_os_str().to_str().expect("Could not retrieve selected text"))
                            .show_ui(ui, |ui| {
                                for path in paths.clone() {
                                    let mut current_path = Some(ttrpg.database.as_os_str().to_str().expect("could not get selectable value").to_string());
                                    let path = if path.len() > 12 {path} else {"None selected".to_string()};
                                    let selectable_value = ui.selectable_value(&mut current_path, Some(path.to_string().clone()), &path);
                                    if selectable_value.clicked() {
                                        ttrpg.database = Path::new(&path).to_path_buf();
                                    }
                                    if selectable_value.secondary_clicked() {
                                        std::fs::remove_file(&path).expect("Failed to delete database file...");
                                        // pushed to a vector to be looped over to set all the existing ttrpgs databases which where shared tp be nothing
                                        dbs_to_delete.push(path);
                                    }
                                }
                            });
                    });
                });
            }
        });
    });
    if dbs_to_delete.len() > 0 {
        let dummy_id: Option<u32> = None;
        let dummy_db: Option<&str> = None;
        for ttrpg in ttrpgs {
            for db in dbs_to_delete.iter() {
                let delete_to_path_buff = Path::new(&db).to_path_buf();
                if ttrpg.database.eq(&delete_to_path_buff) {
                    ttrpg.database = TtrpgEntity::new(false, dummy_id, "deletion of database".to_string(), dummy_db).database; 
                }
            }
        }
    }
    selected_ttrpg_ui.response.rect.size()
}
pub fn dice_rolls_and_creation_history (ui: &mut Ui) -> Vec2 {
    let dice_rolls_and_creation_history_ui = ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.strong("dice_rolls_and_creation_history_ui test");
        });
    });
    dice_rolls_and_creation_history_ui.response.rect.size()
}
pub fn saved_configs_window(ui: &mut Ui) -> Vec2 {
    let saved_configs_window_ui = ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.strong("saved_configs_window ui test");
        });
    });
    saved_configs_window_ui.response.rect.size()
}
