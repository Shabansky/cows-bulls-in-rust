use super::Number;

#[derive(Debug)]
pub struct Player {
    number: Number,
    guesses: Vec<i32>,
}

impl Player {
    pub fn new(number: Number) -> Self {
        Self {
            number: number,
            guesses: vec![],
        }
    }

    pub fn get_number(&self) -> &Number {
        &self.number
    }
}
