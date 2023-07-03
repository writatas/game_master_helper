use eframe::egui::TextBuffer;

#[derive(Clone, Debug)]
struct Story {
    pub id: u32,
    pub order_num: u32,
    pub label: String,
    pub raw_narration: String,
}

impl Story {
    pub fn new(id: u32, order_num: u32, label: &str, raw_narration: &str) -> Story {
        Story {
            id, 
            order_num,
            label: label.to_string(), 
            raw_narration: raw_narration.to_string(), 
        } 
    }
    // TODO create a summarizing type to initilize the summary of the Story when created
    pub fn summarize(self) -> String {
        self.raw_narration
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
struct Attribute {}
