use eframe::egui::{Vec2, Ui};
//TODO new ttrpg_entity 
// returns the ui height and width as a egui::Vec2 in order to calculate ui sizes
pub fn configuration_ui(ui: &mut Ui) -> Vec2 {
    let config_ui = ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.strong("configuration window test");
        });
    });
    config_ui.response.rect.size()
}
pub fn selected_ttrpg_elements(ui: &mut Ui) -> Vec2 {
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
