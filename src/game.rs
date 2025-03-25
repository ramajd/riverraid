use crate::screen::Screen;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Game<'a> {
    screen: &'a mut Screen,
    difficulty: Difficulty,
}

impl<'a> Game<'a> {
    pub fn new(screen: &'a mut Screen, difficulty: Difficulty) -> std::io::Result<Self> {
        Ok(Self {
            screen,
            difficulty,
        })
    }

    pub fn initiate(&mut self) -> std::io::Result<Self> {
        todo!()
    }

    pub fn terminate(&mut self) -> std::io::Result<Self> {
        todo!()
    }

    pub fn main_loop(&mut self) -> std::io::Result<Self> {
        todo!()
    }
}
