#![allow(dead_code)]
use std::cell::Cell;
use anyhow::{Ok, Error};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

//Constants
const NUMBER_LIMIT:i32 = 10_000;
const OS: &str = std::env::consts::OS;
// specific enums for structs
pub enum Boon {
    Advantage,
    Disadvantage,
    Plain
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Elements {
    Story(Story),
    Attribute(Attribute),
    Skill(Skill),
    Counter(Counter),
    Table(Table)
}
// Traits
trait DiceRoll {
    fn roll(&self) -> (Vec<u32>, String);
}
pub trait SaveLoad {
    type Entity;
    fn values_to_json(&self) -> String;
    fn values_from_json(&mut self, serialized: &str) -> Result<(), Error>;
    fn delete_element(&mut self, entity_label: &str) -> Result<(), Error>;
}
//Helper functions for structs
fn parse_key(key_str: &str) -> (u32, u32) {
    let parts: Vec<&str> = key_str.split(',').collect();
    let x = parts[0].parse().unwrap();
    let y = parts[1].parse().unwrap();
    (x, y)
}

fn escape_sql(input: &str) -> String {
    input.replace("'", "''")
}

// structs
#[derive(Serialize, Deserialize)]
pub struct TtrpgEntity {
    pub active: Cell<bool>,
    pub edit: Cell<bool>,
    pub id: String,
    pub name: String,
    pub database: PathBuf,
    pub elements: HashMap<String, Elements>
}

impl TtrpgEntity {
    pub fn new(active: bool, edit: bool, id: Option<String>, name: String, database: Option<&str>) -> TtrpgEntity {
        let current_dir = PathBuf::new();
        let db_string = database.unwrap_or("");
        let id_string = id.unwrap_or("".to_string());
        let path = match OS {
            "linux" => current_dir.join(format!("./saved_dbs/{}", db_string)),
            "macos" => current_dir.join(format!("./saved_dbs/{}", db_string)),
            "windows" => current_dir.join(format!("./saved_dbs\\{}", db_string)),
            _ => panic!("Unsupported OS!")
        };
        TtrpgEntity {
            active: Cell::new(active),
            edit: Cell::new(edit),
            id: id_string,
            name,
            database: path,
            elements: HashMap::new()
        }
    }
    pub fn add_element(&mut self, element: Elements) -> Option<Elements>{
        match element {
            Elements::Story(s) => {
                self.elements.insert(s.label.clone(), Elements::Story(s))
            },
            Elements::Attribute(a) => {
                self.elements.insert(a.label.clone(), Elements::Attribute(a))
            },
            Elements::Skill(sk) => {
                self.elements.insert(sk.label.clone(), Elements::Skill(sk))
            },
            Elements::Counter(c) => {
                self.elements.insert(c.label.clone(), Elements::Counter(c))
            },
            Elements::Table(t) => {
                self.elements.insert(t.label.clone(), Elements::Table(t))
            },
        }
    }
    pub fn retrieve_all_element_ids(&self) -> Vec<String> {
        let mut hash_ids = Vec::new();
        for (hash, _el) in self.elements.iter() {
            hash_ids.push(hash.clone());
        }
        hash_ids
    }
}

impl SaveLoad for TtrpgEntity {
    type Entity = TtrpgEntity;
    fn values_to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("{}", serialized);
        serialized
    }
    // Will transfer serialized values from a string that is retrieved from a database and assigns its values to be the same as the serialized values
    fn values_from_json(&mut self, serialized: &str) -> Result<(), Error> {
        let deserialized = serde_json::from_str::<TtrpgEntity>(&serialized).unwrap();
        *self = deserialized;
        Ok(())
    }
  
    fn delete_element(&mut self, entity_label: &str) -> Result<(), Error> {
        // implementation here
        self.elements.remove(entity_label);
        Ok(())
    }
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Story {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub raw_narration: String,
}

impl Story {
    pub fn new(id: u32, order_num: u32, label: &str, raw_narration: &str) -> Result<Story, Error> {
        let story = Story {
            edit:Cell::new(true), // defaults to true
            id, 
            order_num,
            label: label.to_string(), 
            raw_narration: escape_sql(raw_narration), 
        };
        Ok(story)
    }
    // TODO create a summarizing type to initilize the summary of the Story when created
    pub fn summary(self) -> Result<String, Error> {
        // Looking to using rust-bert crate to implement text summarization and text generation AI!
        Ok(self.raw_narration)
    }
    pub fn get_word_count(self) -> u32 {
        let words: Vec<&str> = self.raw_narration.split_whitespace().into_iter().collect();
        words.len() as u32
    }
    pub fn edit(&mut self, label: String, new_text: String) {
        self.label = label;
        self.raw_narration = new_text;
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Attribute {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub description: String,
    pub modifier: u32, // (Ability score - 10) / 2
    pub roll: Outcome
}

impl Attribute {
    pub fn new(id: u32, order_num: u32, label: String, description: String, roll: Roll, critical: u32) -> Result<Attribute, Error> {
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
    pub fn edit(&mut self, new_text: String, change_base_by: u32) { // change the value of the base roll (can increase / decrease atttributes this way)
        self.roll.base_result += change_base_by;
        self.modifier = (self.roll.base_result - 10) / 2 as u32;
        self.description = format!("{} {} ({})", new_text, self.roll.base_result, self.modifier);
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Skill {
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
    pub fn new(id: u32, order_num: u32, label: String, level: u32, skill_level: u32, has_proficiency: bool) -> Result<Skill, Error> {
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

    pub fn get_description(self) -> Result<String, Error> {
        if self.has_proficiency {
            return Ok(format!("{} {}({})", self.label, self.skill_level, self.proficiency));
        }
        Ok(format!("{} {}", self.label, self.skill_level))
    }
    pub fn edit(&mut self) {

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

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Counter {
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

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Table {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub table: HashMap<(u32, u32), String>
}
impl Table {
    pub fn new(id: u32, order_num: u32, label: String, table: Vec<((u32, u32), String)>) -> Result<Table, Error> {
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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