use eframe;
use eframe::egui;

pub fn horizontal_menu_bar(ui: &mut egui::Ui, buttons: Vec<egui::Button>) {
    ui.horizontal(|ui| {
            for b in buttons {
                ui.add(b);
            }
    });
}
