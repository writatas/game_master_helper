use std::cell::Cell;
use std::path::Path;
use gm_helper_corelibrary::{TtrpgEntity, SaveLoad, record_audio, transcribe_audio_file};
use eframe::egui::{Vec2, Ui, ComboBox, ScrollArea};
use sqlite::{Connection, State};
use rand::{distributions::Alphanumeric, Rng}; 
//TODO new ttrpg_entity 
// returns the ui height and width as a egui::Vec2 in order to calculate ui sizes
pub fn configuration_ui(ui: &mut Ui, ttrpgs: &mut Vec<TtrpgEntity>, new_database: &mut Cell<String>, new_ttrpg: &mut Cell<TtrpgEntity>) -> Vec2 { // Select database and load elements
    let config_ui = ui.group(|ui| {
        ui.group(|ui|{
            ui.horizontal(|ui| {
                if ui.button("Create database").clicked() {
                    let dummy_ttrpg = TtrpgEntity::new(false, false, None, "dummy".to_string(), Some(new_database.get_mut()));
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
                                string_id TEXT NOT NULL,
                                date DATETIME DEFAULT CURRENT_TIMESTAMP,
                                json_string TEXT NOT NULL
                            );
                        ";
                        connection.execute(query).expect("SQL query failure; could not create database...");
                        new_database.set("".to_string());
                    }
                }
                ui.text_edit_singleline(new_database.get_mut())
            });
        });
        
        new_ttrpg.get_mut().active.set(true);
        if new_ttrpg.get_mut().active.get() == true {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Create TTRPG!").clicked() {
                    if new_ttrpg.get_mut().name.clone().len() > 0 {
                        //Create a new copy of dummy value to pass user defined name into active ttrpgs
                        let new_ttrpg_element = TtrpgEntity::new(true, false, None, new_ttrpg.get_mut().name.clone().to_string(), None);
                        let mut existing_names: Vec<String> = Vec::new();
                        for ttrpg in ttrpgs.iter() {
                            existing_names.push(ttrpg.name.clone())
                        }
                        //ttrpg names should be unique
                        if !existing_names.contains(&new_ttrpg_element.name) {
                            ttrpgs.push(new_ttrpg_element);
                            new_ttrpg.get_mut().active.set(false);
                            new_ttrpg.get_mut().name = "".to_string();
                        }
                    }
                    new_ttrpg.get_mut().active.set(false);
                }
                ui.text_edit_singleline(&mut new_ttrpg.get_mut().name);
            });
        }
        
        //Handling the recording and transcription of audio to text with whisper.cpp
        
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
    let existing_paths = std::fs::read_dir("./saved_dbs/").expect("Could not read existing database paths");
    let mut paths = Vec::new();
    for p in existing_paths {
        paths.push(p.unwrap().path().display().to_string());
    }
    let mut dbs_to_delete: Vec<String> = Vec::new();
    let mut ttrpgs_to_delete:Vec<(usize, String, bool)> = Vec::new(); // return a bool at the end to signify that it needs to be deleted from a database
    let mut ttrpgs_to_load: Vec<TtrpgEntity> = Vec::new();
    let selected_ttrpg_ui = ui.group(|ui| {
        if ui.button("clear view").clicked() { ttrpgs.clear();}
        ui.strong(format!("Number of ttrpg entities: {}", ttrpgs.len()));
        ui.strong(format!("Number of ttrpg entities without chosen databases: {}", ttrpg_without_databases));
        ScrollArea::vertical().show(ui, |ui| {
            for (index, ttrpg) in ttrpgs.iter_mut().enumerate() {
                let db_selected = ttrpg.database.as_os_str().to_str().unwrap()[12..].len().gt(&0);// the bool
                ui.group(|ui| {
                    ui.strong(ttrpg.name.clone());
                    ui.horizontal_wrapped(|ui| {
                        let active_text = if ttrpg.active.get() {"Active"} else {"Not Active"};
                        ui.checkbox(ttrpg.active.get_mut(), active_text);

                        if ui.small_button("Delete").clicked() {
                            if db_selected && ttrpg.id.len() > 0 {
                                let del_from_db = Connection::open(ttrpg.database.as_os_str())
                                    .expect("Unable to create database connection");
                                let query = format!("DELETE FROM ttrpgs WHERE string_id = '{}'", ttrpg.id);
                                del_from_db.execute(query).expect("Could not delete ttrpg from database");
                                ttrpgs_to_delete.push((index, ttrpg.name.clone(), db_selected));
                            }
                            else {
                                ttrpgs_to_delete.push((index, ttrpg.name.clone(), db_selected));
                            }
                        }
                    });
                    ui.label(format!("Number of elements {}", ttrpg.elements.len()));
                    // Database selection and creation per ttrpg element
                    ui.horizontal(|ui| {
                        // Select the Database this ttrpg should save to
                        ui.label("Selected database: ");
                        ComboBox::from_id_source(format!("{}{}", &ttrpg.name, &index.to_string())) //ui id is the name of the ttrpg and the index to which it shows up on the list
                        .selected_text(ttrpg.database.as_os_str().to_str().expect("Could not retrieve selected text"))
                        .show_ui(ui, |ui| {
                                for path in paths.iter() {
                                        let mut current_path = "".to_string();
                                        let path_cut = &path.clone()[12..];
                                        let selectable_value = ui.selectable_value(&mut current_path, path_cut.to_string() ,path_cut.to_string());
                                        if selectable_value.clicked() {
                                            ttrpg.database = Path::new(&path).to_path_buf();
                                            ttrpg.id = "".to_string();
                                            let load_ttrpgs = load_selected_database(&ttrpg.database);
                                            ttrpgs_to_load = load_ttrpgs;
                                        }
                                        if selectable_value.secondary_clicked() {
                                            std::fs::remove_file(&path).expect("Failed to delete database file...");
                                            // pushed to a vector to be looped over to set all the existing ttrpgs databases which where shared to be nothing
                                            dbs_to_delete.push(path.clone());
                                        }
                                }
                        });
                        if db_selected {
                            if ui.small_button("Save").clicked() {
                                let connection = Connection::open(ttrpg.database.as_os_str())
                                    .expect(format!("Failed to open database for ttrpg {} - {:?}", &ttrpg.name, &ttrpg.database.as_os_str()).as_str());
                                if ttrpg.id.len() == 0 {
                                    ttrpg.id = random_string();
                                    let query = format!(
                                        "
                                            INSERT INTO ttrpgs (
                                                json_string,
                                                string_id
                                            )
                                            VALUES ('{}', '{}');

                                        ",
                                        ttrpg.values_to_json(),
                                        &ttrpg.id
                                    );
                                    println!("Saved {}", &ttrpg.name);
                                    connection.execute(query).expect(format!("Unable to save {}", &ttrpg.name).as_str());
                                }
                                else {  
                                    let query = format!(
                                        "
                                            UPDATE ttrpgs SET json_string = '{}' WHERE string_id = '{}';
                                        ",
                                        ttrpg.values_to_json(),
                                        &ttrpg.id
                                    );
                                    println!("Updated {}", &ttrpg.name);
                                    connection.execute(query).expect(format!("Unable to update {}", &ttrpg.name).as_str());
                                }
                        }
                    }
                    });
                });
            }
            
        });
    });
    if ttrpgs_to_delete.len() > 0 {
        for delete in ttrpgs_to_delete.iter() {
            ttrpgs.remove(delete.0);
            println!("Deleted ttrpg: {}", delete.1);
        }
        ttrpgs_to_delete.clear();
    }

    if dbs_to_delete.len() > 0 {
        let dummy_id: Option<String> = None;
        let dummy_db: Option<&str> = None;
        for ttrpg in ttrpgs.iter_mut() {
            for db in dbs_to_delete.iter_mut() {
                let delete_to_path_buff = Path::new(&db).to_path_buf();
                if ttrpg.database.eq(&delete_to_path_buff) {
                    let db_delete = TtrpgEntity::new(false, false, dummy_id.clone(), "deletion of database".to_string(), dummy_db).database;
                    ttrpg.database =  db_delete;
                }
            }
        }
    }
    if ttrpgs_to_load.len() > 0 {
        for t in ttrpgs_to_load.iter() {
            ttrpgs.push(
                TtrpgEntity {
                    active: Cell::new(false),
                    edit: Cell::new(false),
                    id: t.id.clone(),
                    name: t.name.clone(),
                    database: t.database.clone(),
                    elements: t.elements.clone()
                }
            );
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

fn random_string() -> String {
    let s:String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(8)
    .map(char::from)
    .collect();
    s
}

fn load_selected_database(path: &Path) -> Vec<TtrpgEntity> {
    let mut ttrpgs: Vec<TtrpgEntity> = Vec::new();
    let query = "SELECT * FROM ttrpgs";
    let connection = Connection::open(path).expect("Could not load data from database");
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        let json_string = statement.read::<String, _>("json_string").unwrap();
        let mut load_ttrpg = TtrpgEntity::new(false, false, None, "dummy".to_string(), None);
        load_ttrpg.values_from_json(&json_string.as_str()).expect("Could not load ttrpg into vector from database");
        ttrpgs.push(load_ttrpg);
    }

    ttrpgs
}