use std::{collections::HashSet};
use std::cell::Cell;
use gm_helper_corelibrary::TtrpgEntity;
use eframe::egui::{Vec2, Ui, ComboBox, Button, Label, Window, Context, ScrollArea};
use sqlite::{Connection};
//TODO new ttrpg_entity 
// returns the ui height and width as a egui::Vec2 in order to calculate ui sizes
pub fn configuration_ui(ui: &mut Ui, ttrpgs: &mut Vec<TtrpgEntity>, selected_db: &mut Cell<String>, new_database: &mut Cell<String>, new_ttrpg: &mut Cell<TtrpgEntity>) -> Vec2 { // Select database and load elements
    let config_ui = ui.group(|ui| {
        let existing_paths = std::fs::read_dir("./saved_dbs/").unwrap();
        let mut paths = Vec::new();
        for p in existing_paths {
            paths.push(p.unwrap().path().display().to_string());
        }
        if paths.len() > 0 {
            ui.group(|ui| {
                ui.add(Label::new("Select Database"));
                ComboBox::from_id_source("databases")
                    .selected_text(selected_db.get_mut().as_str())
                    .show_ui(ui, |ui| {
                        for path in paths {
                            let selectable_value = ui.selectable_value(selected_db.get_mut(), path.to_string().clone(), &path);
                            if selectable_value.clicked() {
                                selected_db.set(path);
                            }
                        }
                    });
            });
        }
        else {
            ui.label("No database detected.. Create a new one!");
        }
            ui.group(|ui|{
                if ui.button("Create database").clicked() {
                    let dummy_ttrpg = TtrpgEntity::new(false, None, "dummy".to_string(), Some(new_database.get_mut()));
                    // Create the database as long under condition checks if the selected_db is not empty, contains whitespace, and that the newly created one does not exist already
                    if !selected_db.get_mut().is_empty() && 
                    !selected_db.get_mut().contains(char::is_whitespace) &&
                    !dummy_ttrpg.database.is_file()
                    {
                        println!("{:?}", dummy_ttrpg.database.as_os_str());
                        let connection = Connection::open(dummy_ttrpg.database.as_os_str()).expect("Database creation failed!");
                        let query = "
                            CREATE table ttrpgs (
                                id INTEGER PRIMARY KEY AUTOINCREMENT,
                                date DATETIME DEFAULT CURRENT_TIMESTAMP,
                                json_string TEXT NOT NULL
                            );
                        ";
                        connection.execute(query).expect("SQL query failure!");
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
    let selected_ttrpg_ui = ui.group(|ui| {
            ui.strong(format!("Number of ttrpgs: {}", ttrpgs.len()));
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
                        });
                    });
                }
            });
    });
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
