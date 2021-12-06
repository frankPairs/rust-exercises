// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct CsvRecordBuilder {
    content: String,
}

impl CsvRecordBuilder {
    // Create a new builder
    pub fn new() -> Self {
        CsvRecordBuilder {
            content: String::from(""),
        }
    }

    /// Adds an item to the list separated by comma.
    pub fn add(&mut self, val: &str) {
        let chars = ['\n', '"', ','];
        let mut formatted_value = format!(",{}", val);

        if val.contains(&chars[..]) {
            formatted_value = format!(",\"{}\"", val.replace('"', "\"\""));
        }

        self.content.push_str(formatted_value.as_str());
    }

    /// Consumes the builder and returns the comma separated list
    pub fn build(self) -> String {
        if self.content.is_empty() {
            return self.content;
        }

        // We remove the first character as it will be a comma.
        String::from(self.content.get(1..).unwrap())
    }
}
