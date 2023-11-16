use std::{collections::HashMap, ops::Add};
use std::cell::Cell;
use eframe::egui::{self, Ui};
use egui::Pos2;
use gm_helper_corelibrary::{TtrpgEntity, Story, Attribute, Counter, Skill, Table, Elements};
use crate::collapsables::*;

pub struct MainWindow {
    new_database: Cell<String>,
    configure_creation_window: Cell<bool>,
    selected_ttrpg_elements: Cell<bool>,
    dice_rolls_creation_history: Cell<bool>,
    saved_configs_window: Cell<bool>,
    saved_configurations: HashMap<String, bool>,
    active_ttrpg_elements: Vec<TtrpgEntity>,
    ttrpg_creation: Cell<TtrpgEntity>, // Just a dummy ttrpg
    new_text_label: String,
    new_text_body: String,
    new_number: u32,
    transcribed_audio: String,
    recording: Cell<bool>

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
        let new_text_label = "".to_string();
        let new_text_body = "".to_string();
        let new_number = 0;
        let transcribed_audio = String::from("");
        let recording = Cell::new(false);
        Self {
            new_database,
            configure_creation_window,
            selected_ttrpg_elements,
            dice_rolls_creation_history,
            saved_configs_window,
            saved_configurations,
            active_ttrpg_elements,
            ttrpg_creation,
            new_text_label,
            new_text_body,
            new_number,
            transcribed_audio,
            recording
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                display_active_elements(ui, &mut self.active_ttrpg_elements, &mut self.new_text_label, &mut self.new_text_body, &mut self.new_number);
            });
        });
    }
}

fn display_active_elements(ui: &mut egui::Ui, ttrpg_entities: &mut Vec<TtrpgEntity>, new_text_label: &mut String, new_text_body: &mut String, new_number: &mut u32) {
    for entity in ttrpg_entities {
            if entity.active.get() {
                ui.horizontal_top(|ui| {
                    if ui.button("Story").clicked() {
                        &new_text_label.clear();
                        &new_text_body.clear();
                        let elements_len = entity.elements.len() + 1;
                        let elements_len = elements_len as u32;
                        let new_story = Story::new( // Ids of elements stay 0 and change when saved
                            elements_len + 1,
                            elements_len + 1,
                            &new_text_label,
                            &new_text_body
                        );
                        entity.add_element(
                            Elements::Story(new_story.unwrap())
                        );
                    }
                });
                // display elements that are active and where edit is false
                let element_ids = entity.retrieve_all_element_keys();
                for key in element_ids {
                    match entity.elements.get_mut(&key).unwrap() {
                        Elements::Story(s) => {
                            ui.group(|ui| {
                                if ui.button("edit").clicked() && !s.edit.get() { // if the edit button is clicked and not already selected
                                    s.edit.set(true);
                                    new_text_label.push_str(&s.label);
                                    new_text_body.push_str(&s.raw_narration);
                                }
                                if s.edit.get() {
                                    if ui.button("Save").clicked() {
                                        s.edit.set(false);
                                        s.edit(new_text_label.clone(), new_text_body.clone());
                                        new_text_label.clear();
                                        new_text_body.clear();

                                    }
                                    let mut label = ui.text_edit_singleline(new_text_label);
                                    let mut body = ui.text_edit_multiline(new_text_body);

                                    let label_available_width = label.ctx.available_rect();
                                    let body_available_width = body.ctx.available_rect();

                                    label.rect.set_width(label_available_width.width());
                                    body.rect.set_width(body_available_width.width());
            
                                }
                                else {
                                    ui.label(s.label.clone());
                                    ui.label(s.raw_narration.clone());
                                }
                            });
                        },
                        Elements::Attribute(a) => {
                        },
                        Elements::Skill(sk) => {
                        },
                        Elements::Counter(c) => {
                        },
                        Elements::Table(t) => {
                        },
                    }
                }
            }
            
    }
 
}

fn track_cursor_position(ctx: &egui::Context) -> Pos2 {
    if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {pos} else {egui::pos2(0.0, 0.0)}
}

fn escape_special_chars(input_string: &str, reverse: bool) -> String { // To escape sql characters
    let mut output_string = String::new();
    for c in input_string.chars() {
        match c {
            '\'' | '\"' | '\\' | '\n' | '\r' | '\t' => {
                if !reverse {
                    output_string.push('\\');
                }
                output_string.push(c);
            }
            _ => output_string.push(c),
        }
    }
    if !reverse {
        output_string = output_string.replace("%", "\\%");
        output_string = output_string.replace("_", "\\_");
    } else {
        output_string = output_string.replace("\\%", "%");
        output_string = output_string.replace("\\_", "_");
    }
    output_string
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