use super::Number;

#[derive(Debug)]
pub struct Player {
    name: String,
    number: Number,
    guesses: Vec<i32>,
}

impl Player {
    pub fn new(name: String, number: Number) -> Self {
        Self {
            name: name,
            number: number,
            guesses: vec![],
        }
    }

    pub fn get_number(&self) -> &Number {
        &self.number
    }
}
