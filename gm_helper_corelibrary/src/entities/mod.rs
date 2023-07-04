#![allow(dead_code)]
use eframe::egui::TextBuffer;
use std::cell::Cell;
use rand::{thread_rng, Rng};
//use std::env::var;
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
    pub proficiency: u32 // 2 + (1/4 * level - 1)
}

impl Skill {
    pub fn new(id: u32, order_num: u32, label: String, level: u32) -> Result<Skill, String> {
        let skill = Skill {
            id,
            order_num,
            label,
            edit: Cell::new(false),
            level, 
            proficiency: 2 + (1/4 * level - 1) as u32
        };
        Ok(skill)
    }
}

#[derive(Clone, Debug)]
struct Counter {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub number: u32
}

#[derive(Clone, Debug)]
struct Table {
    pub edit: Cell<bool>,
    pub id: u32,
    pub order_num: u32,
    pub label: String,
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
    pub attribute: bool,
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
        let mut  base_result: u32 = rolled.iter().sum();
        
        Outcome {
            roll_description,
            base_result,
            max,
            min,
            attribute,
            critical
        }
    }

    pub fn success_of_roll(&self, opposition: &Outcome, difficulty: u32) -> (bool, u32) {
        let difficulty = if opposition.attribute == true
                        {opposition.base_result.clone()} else {difficulty};
        let winner = match opposition.attribute {
            true => if self.critical == 20 {
                self.base_result >= difficulty && self.base_result >= opposition.base_result
            } else {
                self.base_result <= difficulty && self.base_result <= opposition.base_result
            },
            false => if self.critical == 20 {
                self.base_result >= difficulty
            } else {
                self.base_result <= difficulty
            }
        };

        (winner, difficulty)
    }
}
