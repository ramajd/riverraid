use crate::screen::{Location, Screen};

use super::world::World;

const PLANE: [&str; 3] = [
    " ▟▙ ", // 0
    "▞▜▛▚", // 1
    " ▘▝ ", // 2
            // " ▞▚ ", // 2
];

#[derive(Debug)]
pub struct Player {
    pub location: Location,
    baundry: Location,
}

impl Player {
    pub fn new(col: u16, row: u16, baundry: Location) -> Self {
        Self {
            location: Location(col, row),
            baundry,
        }
    }

    pub fn move_up(&mut self) {
        if self.location.1 >= PLANE.len() as u16 {
            self.location.1 -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.location.1 < self.baundry.1 - 1 {
            self.location.1 += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.location.0 >= 1 {
            self.location.0 -= 1;
        }
    }

    pub fn move_right(&mut self) {
        let plane_width = PLANE[0].chars().count() as u16;
        if self.location.0 <= self.baundry.0 - 1 - plane_width {
            self.location.0 += 1;
        }
    }

    pub fn draw(&self, screen: &mut Screen) -> std::io::Result<()> {
        for (i, line) in PLANE.iter().rev().enumerate() {
            screen.draw(
                Location(self.location.0, self.location.1 - i as u16),
                line.to_string(),
                crossterm::style::Color::White,
                crossterm::style::Color::Blue,
            )?;
        }
        Ok(())
    }

    pub fn hit_walls(&self, world: &World) -> bool {
        let plane_width = PLANE[0].chars().count() as u16;
        world
            .walls
            .get(self.location.1 as usize)
            .map_or(false, |&x| {
                self.location.0 <= x.0 || self.location.0 + plane_width > x.1
            })
    }
}
