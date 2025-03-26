use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, poll, read},
    style::Color,
};
use player::Player;
use world::World;

use crate::screen::{Location, Screen};

mod player;
mod world;

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Running,
    PlayerDied,
    Quit,
}

#[derive(Debug)]
pub struct Game<'a> {
    screen: &'a mut Screen,
    difficulty: Difficulty,
    status: GameStatus,
    world: World,
    player: Player,
}

impl<'a> Game<'a> {
    pub fn new(screen: &'a mut Screen, difficulty: Difficulty) -> std::io::Result<Self> {
        let dimensions = screen.size;
        Ok(Self {
            screen,
            difficulty,
            status: GameStatus::Running,
            world: World::new(dimensions)?,
            player: Player::new(
                dimensions.0 / 2,
                dimensions.1 - 1,
                Location(dimensions.0, dimensions.1),
            ),
        })
    }

    pub fn initiate(&mut self) -> std::io::Result<()> {
        self.screen.clear(true)?;
        Ok(())
    }

    pub fn terminate(&mut self) -> std::io::Result<()> {
        self.screen.clear(true)?;
        if self.status == GameStatus::PlayerDied {
            self.screen.draw(
                Location(0, 0),
                "Player Died!".to_string(),
                Color::White,
                Color::Black,
            )?;
        }
        Ok(())
    }

    pub fn main_loop(&mut self) -> std::io::Result<()> {
        // todo!()
        while self.status == GameStatus::Running {
            self.process_events()?;
            self.apply_physics()?;
            self.draw()?;
        }
        Ok(())
    }

    fn process_events(&mut self) -> std::io::Result<()> {
        if poll(Duration::from_millis(10))? {
            let evt = read()?;
            match evt {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.status = GameStatus::Quit,
                    KeyCode::Char('w') | KeyCode::Up => self.player.move_up(),
                    KeyCode::Char('s') | KeyCode::Down => self.player.move_down(),
                    KeyCode::Char('a') | KeyCode::Left => self.player.move_left(),
                    KeyCode::Char('d') | KeyCode::Right => self.player.move_right(),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn apply_physics(&mut self) -> std::io::Result<()> {
        if self.player.hit_walls(&self.world) {
            self.status = GameStatus::PlayerDied;
        }
        Ok(())
    }

    fn draw(&mut self) -> std::io::Result<()> {
        self.screen.clear(false)?;

        self.world.draw(self.screen)?;
        // self.screen.draw(
        //     Location(0, 0),
        //     format!("{:?}", &self),
        //     Color::White,
        //     Color::Black,
        // )?;
        self.player.draw(self.screen)?;
        self.screen.flush()?;
        Ok(())
    }
}
