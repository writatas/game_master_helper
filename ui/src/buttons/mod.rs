use eframe;
use eframe::egui;

pub fn text_button(text: &str, rgb: (u8, u8, u8), min_size: (f32, f32)) -> egui::Button {
    let bg_color: eframe::epaint::Color32 = egui::Color32::from_rgb(rgb.0, rgb.1, rgb.2);
    let b: egui::Button = egui::Button::new(text)
        .fill(bg_color)
        .wrap(true)
        .min_size(egui::vec2(min_size.0, min_size.1));
    b
}