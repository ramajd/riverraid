use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, poll, read},
    style::Color,
};
use world::World;

use crate::screen::Screen;

mod world;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(PartialEq)]
pub enum GameStatus {
    Running,
    Stopped,
}

pub struct Game<'a> {
    screen: &'a mut Screen,
    difficulty: Difficulty,
    status: GameStatus,
    world: World,
}

impl<'a> Game<'a> {
    pub fn new(screen: &'a mut Screen, difficulty: Difficulty) -> std::io::Result<Self> {
        let dimensions = screen.size;
        Ok(Self {
            screen,
            difficulty,
            status: GameStatus::Running,
            world: World::new(dimensions)?,
        })
    }

    pub fn initiate(&mut self) -> std::io::Result<()> {
        self.screen.clear()?;
        Ok(())
    }

    pub fn terminate(&mut self) -> std::io::Result<()> {
        // todo!()
        self.screen.clear()?;
        Ok(())
    }

    pub fn main_loop(&mut self) -> std::io::Result<()> {
        // todo!()
        while self.status == GameStatus::Running {
            // 1. process events
            if poll(Duration::from_millis(100))? {
                let evt = read()?;
                match evt {
                    Event::Key(key) => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.status = GameStatus::Stopped,
                        _ => {}
                    },
                    _ => {}
                }
            }

            // 2. apply physics
            // self.perform_physics()?;

            // 3. draw
            self.draw()?;
        }
        Ok(())
    }

    fn draw(&mut self) -> std::io::Result<()> {
        self.draw_world()?;
        self.screen.flush()?;
        Ok(())
    }

    fn draw_world(&mut self) -> std::io::Result<()> {
        for i in (0..self.world.walls.len()).rev() {
            let left_field = " ".repeat(self.world.walls[i].0 as usize);
            let right_field = " ".repeat((self.screen.size.0 - self.world.walls[i].1) as usize);
            self.screen
                .draw((0, i as u16), left_field, Color::Black, Color::DarkGreen)?;
            self.screen.draw(
                (self.world.walls[i].1, i as u16),
                right_field,
                Color::Black,
                Color::DarkGreen,
            )?;
        }
        Ok(())
    }
}
