use std::collections::HashSet;
use std::cell::Cell;
use gm_helper_corelibrary::TtrpgEntity;
use eframe::egui::{Vec2, Ui, ComboBox, Button, Label, Window, Context};
//TODO new ttrpg_entity 
// returns the ui height and width as a egui::Vec2 in order to calculate ui sizes
pub fn configuration_ui(ui: &mut Ui, ttrpgs: &mut Vec<TtrpgEntity>, selected_db: &mut Cell<String>, ctx: &Context, new_ttrpg: &mut Cell<TtrpgEntity>) -> Vec2 { // Select database and load elements
    let config_ui = ui.group(|ui| {
        // if the length of loaded TtrpgEntities is larger than 0, implement a Combox ui to select different databases among the ones present in existing ttrpgs
        let mut existing_paths = HashSet::new();
        for ttrpg in ttrpgs.iter() {
            existing_paths.insert(ttrpg.database.as_os_str().to_str().unwrap());
        }
        if existing_paths.len() > 0 && ttrpgs.len() > 0 {
            let existing_paths: Vec<&str> = existing_paths.drain().collect();
            ui.group(|ui| {
                ui.add(Label::new("Select Database"));
                ComboBox::from_id_source("databases")
                    .selected_text(selected_db.get_mut().as_str())
                    .show_ui(ui, |ui| {
                        for path in existing_paths {
                            let selectable_value = ui.selectable_value(selected_db.get_mut(), path.to_string().clone(), path);
                            if selectable_value.clicked() {
                                selected_db.set(path.to_string());
                            }
                        }
                    });
            });
            if ui.add(Button::new("Create New!")).clicked() {
                new_ttrpg.get_mut().active.set(true);
            }
        }
        else {
            new_ttrpg.get_mut().active.set(true);
        }
        if new_ttrpg.get_mut().active.get() == true {
            Window::new("Creation Window")
                .show(ctx, |ui| {
                    ui.text_edit_singleline(&mut new_ttrpg.get_mut().name);
                });
        }
    });
        
    config_ui.response.rect.size()
}
pub fn selected_ttrpg_elements(ui: &mut Ui, ttrpgs: &mut Vec<TtrpgEntity>) -> Vec2 {
    let selected_ttrpg_ui = ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.strong("selected_ttrpg window test");
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
