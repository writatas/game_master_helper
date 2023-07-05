#![allow(dead_code)]
use eframe::egui::TextBuffer;
use std::cell::Cell;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use serde_json;

//Constants
const NUMBER_LIMIT:i32 = 10_000;

// enums for structs
pub enum Boon {
    Advantage,
    Disadvantage,
    Plain
}
// Traits
trait DiceRoll {
    fn roll(&self) -> (Vec<u32>, String);
}
pub trait SaveLoad {
    type Entity;
    fn save(&self, database_path: &str, campaign_id: u32, order_num: u32, edit: bool) -> Result<(), String>;
    fn update(&self, database_path: &str, entity_id: u32, order_num: u32, update_entity: Self::Entity) -> Result<(), String>;
    fn delete(&self, database_path: &str, entity_id: u32) -> Result<(), String>;
}
//Helper functions for structs
fn parse_key(key_str: &str) -> Result<(u32, u32), std::num::ParseIntError> {
    let parts: Vec<&str> = key_str.split(',').collect();
    let x = parts[0].parse()?;
    let y = parts[1].parse()?;
    Ok((x, y))
}
// structs
#[derive(Clone, Debug)]
struct Story {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub raw_narration: String,
}

impl Story {
    pub fn new(id: u32, order_num: u32, label: &str, raw_narration: &str) -> Result<Story, String> {
        let story = Story {
            edit:Cell::new(true), // defaults to true
            id, 
            order_num,
            label: label.to_string(), 
            raw_narration: raw_narration.to_string(), 
        };
        Ok(story)
    }
    // TODO create a summarizing type to initilize the summary of the Story when created
    pub fn summary(self) -> Result<String, String> {
        Ok(self.raw_narration)
    }
    pub fn get_word_count(self) -> u32 {
        let words: Vec<&str> = self.raw_narration.split_whitespace().into_iter().collect();
        words.len() as u32
    }
}

impl TextBuffer for Story {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        &self.raw_narration
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.raw_narration.insert_str(char_index, text);
        char_index + text.len()
    }
    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        let start = char_range.start;
        let end = char_range.end;
        self.raw_narration.replace_range(start..end, "");
    }
}

#[derive(Clone, Debug)]
struct Attribute {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub description: String,
    pub modifier: u32, // (Ability score - 10) / 2
    pub roll: Outcome
}

impl Attribute {
    pub fn new(id: u32, order_num: u32, label: String, description: String, roll: Roll, critical: u32) -> Result<Attribute, String> {
        let roll = Outcome::new(&roll, critical, true);
        let attribute = Attribute {
            id,
            order_num,
            label,
            description,
            edit: Cell::new(false),
            modifier: (roll.base_result - 10) / 2 as u32,
            roll
        };
        Ok(attribute)
    }
    pub fn get_description(self) -> String {
        let description = format!(
            "{} {}({})",
            self.label,
            self.roll.base_result,
            self.modifier
        );
        description
    }
}

#[derive(Clone, Debug)]
struct Skill {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub level: u32,
    pub skill_level: u32,
    pub has_proficiency: bool,
    pub proficiency: i32 // 2 + (1/4 * level - 1)
}

impl Skill {
    pub fn new(id: u32, order_num: u32, label: String, level: u32, skill_level: u32, has_proficiency: bool) -> Result<Skill, String> {
        let skill = Skill {
            id,
            order_num,
            label,
            edit: Cell::new(false),
            level, 
            skill_level,
            has_proficiency,
            proficiency: 2 + (1/4 * level - 1) as i32
        };
        Ok(skill)
    }

    pub fn get_description(self) -> Result<String, String> {
        if self.has_proficiency {
            return Ok(format!("{} {}({})", self.label, self.skill_level, self.proficiency));
        }
        Ok(format!("{} {}", self.label, self.skill_level))
    }
    pub fn roll_skill(self, advantage: Boon, critical: u32, difficulty: u32) -> String {
            match advantage {
                Boon::Advantage => {
                    let mut roll = Outcome::new(&Roll::new(20, 2), critical, false);
                    roll.base_result = if self.proficiency < 0 {roll.max - self.proficiency as u32} else {roll.max + self.proficiency as u32};
                    let success = roll.success_of_roll(None, difficulty);
                    format!(
                        "{}({}) - rolled with advantage {}, {} vs {}",
                        self.label,
                        self.proficiency,
                        roll.base_result,
                        if success.0 {"success"} else {"failure"},
                        success.1
                    )
                },
                Boon::Disadvantage => {
                    let mut roll = Outcome::new(&Roll::new(20, 2), critical, false);
                    roll.base_result = if self.proficiency < 0 {roll.min - self.proficiency as u32} else {roll.min + self.proficiency as u32};
                    let success = roll.success_of_roll(None, difficulty);
                    format!(
                        "{}({}) - rolled with disadvantage {}, {} vs {}",
                        self.label,
                        self.proficiency,
                        roll.base_result,
                        if success.0 {"success"} else {"failure"},
                        success.1
                    )
                },
                Boon::Plain => {
                    let mut roll = Outcome::new(&Roll::new(20, 1), critical, false);
                    roll.base_result += if self.proficiency < 0 {roll.base_result - self.proficiency as u32} else {roll.base_result + self.proficiency as u32};
                    let success = roll.success_of_roll(None, difficulty);
                    format!(
                        "{}({}) - rolled {}, {} vs {}",
                        self.label,
                        self.proficiency,
                        roll.base_result,
                        if success.0 {"success"} else {"failure"},
                        success.1
                    )
                }
            }
    }
}

#[derive(Clone, Debug)]
struct Counter {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub number: i32
}

impl Counter {
    pub fn new(id: u32, order_num: u32, label: String, number: i32) -> Counter{
        let number = if number > NUMBER_LIMIT || number < (NUMBER_LIMIT * -1) {0} else {number};
        Counter {
            id,
            order_num,
            edit: Cell::new(false),
            label,
            number
        }
    }
    pub fn get_description(self) -> String {
        format!("{}: {}", self.label, self.number)
    }
    pub fn increment(&mut self, number: i32) {
        self.number += number;
        if self.number > NUMBER_LIMIT || self.number < NUMBER_LIMIT * -1 {self.number = 0} else {self.number = self.number};
    }
    pub fn decrement(&mut self, number: i32) {
        self.number -= number;
        if self.number > NUMBER_LIMIT || self.number < NUMBER_LIMIT * -1 {self.number = 0} else {self.number = self.number};
    }
}


#[derive(Clone, Debug)]
struct Table {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub table: HashMap<(u32, u32), String>
}
impl Table {
    pub fn new(id: u32, order_num: u32, label: String, table: Vec<((u32, u32), String)>) -> Result<Table, String> {
        let vec_to_hash: HashMap<(u32, u32), String> = table.into_iter().collect();
        let new_table = Table {
            edit: Cell::new(false),
            id,
            order_num,
            label,
            table: vec_to_hash
        };
        Ok(new_table)
    }
        // Serialize the table values into json to be easily stored in a database
        pub fn values_to_json(&self) -> String {
            let map = &self.table;
            let mut data = Vec::new();
            for (key, value) in map {
                let key_str = format!("{},{}", key.0, key.1);
                let item = serde_json::json!({key_str: value});
                data.push(item);
            }
            let value: serde_json::Value = data.into();
            serde_json::to_string(&value).unwrap()
        }
        // Use to Deserialize table values from it's Serialized json value
        pub fn values_from_json(json_str: &str) ->  HashMap<(u32, u32), String> {
            let value: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let mut map = HashMap::new();
            if let serde_json::Value::Array(items) = value {
                for item in items {
                    if let serde_json::Value::Object(obj) = item {
                        for (key_str, value) in obj {
                            if let Ok((x, y)) = parse_key(&key_str) {
                                if let serde_json::Value::String(s) = value {
                                    map.insert((x, y), s);
                                }
                            }
                        }
                    }
                }
            }
            map
        }
        pub fn roll_to_text(&self, roll: &Outcome) -> String {
            let roll_result: u32 = roll.base_result;
            let mut result: String = String::from("");
            for (range, value) in self.table.iter() {
                if roll_result >= range.0 && roll_result <= range.1 {
                    result = value.clone();
                }
            }
            if result.len() == 0 {
                return "Roll failed to produce a value.".to_string()
            }
            result
        }
        pub fn add_row(&mut self, higher: u32, text: String) {
            let keys:Vec<&(u32, u32)> = self.table.keys().collect();
            let lower = if Some(keys.last()).is_some() {keys.last().unwrap().1} else {0};
            let higher = if higher > lower && higher < NUMBER_LIMIT as u32 {higher} else {lower};
            self.table.insert((lower, higher), text);
        }
        pub fn clear_table(&mut self) {
            self.table.clear()
        } 
}

#[derive(Clone, Debug)]
pub struct Roll {
    pub pinned: Cell<bool>,
    pub dice_label: String,
    pub dice: u32,
    pub amount: u32,
}

impl Roll {
    pub fn new(dice: u32, amount: u32) -> Roll {
        let label = format!("{}d{}", &amount, &dice).to_string();
        Roll {
            pinned: Cell::new(false),
            dice_label: label,
            dice,
            amount
        }
    }
}

impl DiceRoll for Roll {
    fn roll(&self) -> (Vec<u32>, String) {
        let mut rng = thread_rng();
        let mut rolls: Vec<u32> = vec![];
        for _ in 0..self.amount {
            rolls.push(rng.gen_range(1..= self.dice));
        }
        (rolls, self.dice_label.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Outcome {
    pub roll_description: String,
    pub base_result: u32,
    pub max: u32,
    pub min: u32,
    pub attribute: bool, // for creating attributes automatically
    pub critical: u32
}

impl Outcome {
    pub fn new(roll: &Roll, critical: u32, attribute: bool) -> Outcome {  
        let mut rolled = roll.roll().0;           
        let roll_description = format!("Roll: {}", roll.dice_label);
        let (max, min) =
            if critical == 20 {
                (*rolled.iter().max().unwrap(), *rolled.iter().min().unwrap())
            }
            else {
                (*rolled.iter().min().unwrap(),  *rolled.iter().max().unwrap())
            };
        if attribute == true && critical == 20 {
            if let Some(min_index) = rolled.iter().position(|&x| x == min) {
                rolled.remove(min_index);
            }
        }
        else if attribute == true && critical == 1 {
            if let Some(max_index) = rolled.iter().position(|&x| x == max) {
                rolled.remove(max_index);
            }
        }
        let base_result: u32 = rolled.iter().sum();
        
        Outcome {
            roll_description,
            base_result,
            max,
            min,
            attribute,
            critical
        }
    }

    pub fn success_of_roll(&self, opposition: Option<&Outcome>, difficulty: u32) -> (bool, u32) {
        let difficulty = match opposition {
            Some(opposition) if opposition.attribute => opposition.base_result.clone(),
            _ => difficulty,
        };
        let winner = match opposition {
            Some(opposition) if self.critical == 20 => {
                self.base_result >= difficulty && self.base_result >= opposition.base_result
            }
            Some(opposition) => self.base_result <= difficulty && self.base_result <= opposition.base_result,
            None if self.critical == 20 => self.base_result >= difficulty,
            None => self.base_result <= difficulty,
        };
    
        (winner, difficulty)
    }    
}