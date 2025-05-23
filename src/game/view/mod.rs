use clearscreen::clear;

pub trait ViewControl {
    fn clear(&self);
}

pub struct TerminalControl {}

impl ViewControl for TerminalControl {
    fn clear(&self) {
        println!("Press enter to continue.");

        std::io::stdin().read_line(&mut String::new()).unwrap();
        clear().expect("Failed to clear screen");
    }
}

pub struct MockControl {}

impl ViewControl for MockControl {
    fn clear(&self) {}
}
