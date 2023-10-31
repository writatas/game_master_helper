use std::collections::HashMap;
use std::cell::Cell;
use eframe::egui::{self, Ui};
use egui::Pos2;
use gm_helper_corelibrary::{TtrpgEntity, Story, Attribute, Counter, Skill, Table};
use crate::collapsables::*;

pub struct MainWindow {
    new_database: Cell<String>,
    configure_creation_window: Cell<bool>,
    selected_ttrpg_elements: Cell<bool>,
    dice_rolls_creation_history: Cell<bool>,
    saved_configs_window: Cell<bool>,
    saved_configurations: HashMap<String, bool>,
    active_ttrpg_elements: Vec<TtrpgEntity>,
    ttrpg_creation: Cell<TtrpgEntity> // Just a dummy ttrpg
}

impl Default for MainWindow {
    fn default() -> Self {
        let new_database: Cell<String> = Cell::new("".to_string());
        let configure_creation_window: Cell<bool> = Cell::new(false);
        let selected_ttrpg_elements: Cell<bool> = Cell::new(false);
        let dice_rolls_creation_history: Cell<bool> = Cell::new(false);
        let saved_configs_window: Cell<bool> = Cell::new(false);
        let saved_configurations: HashMap<String, bool> = HashMap::new();
        let active_ttrpg_elements: Vec<TtrpgEntity> = Vec::new();
        // This is a dummy value that helps pass newly created ttrpg elements into the actual Vector that holds user created elements 
        let ttrpg_creation: Cell<TtrpgEntity> = Cell::new(TtrpgEntity::new(false, false, None, "TTrpg Creation".to_string(), None));
        Self {
            new_database,
            configure_creation_window,
            selected_ttrpg_elements,
            dice_rolls_creation_history,
            saved_configs_window,
            saved_configurations,
            active_ttrpg_elements,
            ttrpg_creation
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Track the cursor position to expand a detract the sections of the main window
        let cursor_pos = track_cursor_position(ctx);
        let upper_x = ctx.available_rect().size().x;
        let upper_y = ctx.available_rect().size().y;
        // Cursor tracking is in this order; Left, Right, Top, Bottom
        if cursor_pos.x < 5.0 && cursor_pos.x > 0.0 && self.selected_ttrpg_elements.get() == false {
            self.selected_ttrpg_elements.set(true);
        }
        if cursor_pos.x > (upper_x - 5.0) && cursor_pos.x < upper_x && self.saved_configs_window.get() == false {
            self.saved_configs_window.set(true);
        }
        if cursor_pos.y < 5.0 && cursor_pos.y > 0.0 && self.configure_creation_window.get() == false {
            self.configure_creation_window.set(true);
        }
        if cursor_pos.y > (upper_y - 5.0) && cursor_pos.y < upper_y && self.dice_rolls_creation_history.get() == false {
            self.dice_rolls_creation_history.set(true);
        }

        // SELECTED TTRPG WINDOW - left
        if self.selected_ttrpg_elements.get() {
            egui::SidePanel::left("selected_ttrpgs_window").show(ctx, |ui| {
                let selected_ttrpg_window_size = selected_ttrpg_elements(ui, &mut self.active_ttrpg_elements);
                if cursor_pos.x > selected_ttrpg_window_size.x {
                    self.selected_ttrpg_elements.set(false);
                }
            });
        }
        // SAVED CONFIGS WINDOW- right
        if self.saved_configs_window.get() {
            egui::SidePanel::right("saved_configs_window").show(ctx, |ui| {
                let saved_configs_window_size = saved_configs_window(ui);
                if cursor_pos.x < (upper_x - saved_configs_window_size.x) {
                    self.saved_configs_window.set(false);
                }
            });
        }
        // CONFIGURATION WINDOW -top
        if self.configure_creation_window.get() {
            egui::TopBottomPanel::top("configure_creation_window").show(ctx, |ui| {
                let config_window_size = configuration_ui(
                    ui,&mut self.active_ttrpg_elements, 
                    &mut self.new_database, 
                    &mut self.ttrpg_creation
                );
                if cursor_pos.y > config_window_size.y {
                    self.configure_creation_window.set(false);
                }
            });
        }
         // DICE ROLLS CREATION HISTORY - botton
         if self.dice_rolls_creation_history.get() {
            egui::TopBottomPanel::bottom("dice_rolls_and_creation_history_window").show(ctx, |ui| {
                let dice_rolls_and_creation_history_window_size = dice_rolls_and_creation_history(ui);
                if cursor_pos.y < (upper_y - dice_rolls_and_creation_history_window_size.y) {
                    self.dice_rolls_creation_history.set(false);
                }
            });
        }
        // ACTIVE TTRPG ELEMENTS CENTRAL PANEL
        egui::CentralPanel::default().show(ctx, |ui| {
            display_active_elements(ui, &mut self.active_ttrpg_elements);
        });
    }
}

fn display_active_elements(ui: &mut egui::Ui, ttrpg_entities: &mut Vec<TtrpgEntity>) {
    for entity in ttrpg_entities {
        ui.horizontal(|ui| {
            if entity.active.get() && !entity.edit.get() {
                ui.label(entity.name.clone());
                // display elements that are active and where edit is false
            }
            
        });
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