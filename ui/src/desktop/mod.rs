use std::collections::HashMap;
use std::cell::Cell;
use std::sync::Arc;
use eframe::egui;
use egui::{Pos2};
use gm_helper_corelibrary::{TtrpgEntity, Story, Attribute, Counter, Skill, Table};
use crate::buttons::text_button;
use crate::collapsables::horizontal_menu_bar;

pub struct MainWindow {
    selected_database:Cell<String>,
    configure_creation_window: Cell<bool>,
    selected_ttrpg_elements: Cell<bool>,
    dice_rolls_creation_history: Cell<bool>,
    saved_configs_window: Cell<bool>,
    saved_configurations: HashMap<String, bool>,
    active_ttrpg_elements: Arc<Vec<TtrpgEntity>>
}

impl Default for MainWindow {
    fn default() -> Self {
        let selected_database: Cell<String> = Cell::new("".to_string());
        let configure_creation_window: Cell<bool> = Cell::new(false);
        let selected_ttrpg_elements: Cell<bool> = Cell::new(false);
        let dice_rolls_creation_history: Cell<bool> = Cell::new(false);
        let saved_configs_window: Cell<bool> = Cell::new(false);
        let saved_configurations: HashMap<String, bool> = HashMap::new();
        let active_ttrpg_elements: Arc<Vec<TtrpgEntity>> = Arc::new(Vec::new());
        Self {
            selected_database,
            configure_creation_window,
            selected_ttrpg_elements,
            dice_rolls_creation_history,
            saved_configs_window,
            saved_configurations,
            active_ttrpg_elements
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Track the cursor position to expand a detract the sections of the main window
        let cursor_pos = track_cursor_position(ctx);
        let upper_x = ctx.used_size().to_pos2().x;
        let upper_y = ctx.used_size().to_pos2().y;
        // Cursor tracking is in this order; Left, Right, Top, Bottom
        if cursor_pos.x < 5.0 && cursor_pos.x > 0.0 && self.selected_ttrpg_elements.get() == false {
            self.selected_ttrpg_elements.set(true);
        }
        else {
            self.selected_ttrpg_elements.set(false);
        }
        if cursor_pos.x > (upper_x - 5.0) && cursor_pos.x < upper_x && self.saved_configs_window.get() == false {
            self.saved_configs_window.set(true);
        }
        else {
            self.saved_configs_window.set(false);
        }
        if cursor_pos.y < 5.0 && cursor_pos.y > 0.0 && self.configure_creation_window.get() == false {
            self.configure_creation_window.set(true);
        }
        else {
            self.configure_creation_window.set(false);
        }
        if cursor_pos.y > (upper_y - 5.0) && cursor_pos.y < upper_y && self.dice_rolls_creation_history.get() == false {
            self.dice_rolls_creation_history.set(true);
        }
        else {
            self.dice_rolls_creation_history.set(false);
        }

        // CONFIGURATION WINDOW
        if self.configure_creation_window.get() {
            egui::TopBottomPanel::top("configure_creation_window").show(ctx, |ui| {
                let buttons = vec![text_button("Create DB", (100, 100, 100), (10.0, 10.0))];
                horizontal_menu_bar(ui, buttons);
            });
        }
    }
}



fn track_cursor_position(ctx: &egui::Context) -> Pos2 {
    if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {pos} else {egui::pos2(0.0, 0.0)}
}

pub fn start_desktop_app() -> Result<(), eframe::Error>
{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
    "TTRPG Maker",
    options,
    Box::new(|_cc| Box::new(MainWindow::default())),
    )
}