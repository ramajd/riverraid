use crossterm::style::Color;

use crate::screen::{Location, Screen};

#[derive(Debug)]
pub struct World {
    pub walls: Vec<(u16, u16)>,
}

impl World {
    pub fn new(dimensions: (u16, u16)) -> std::io::Result<Self> {
        let center = dimensions.0 / 2;
        Ok(World {
            walls: vec![(center - 15, center + 15); dimensions.1 as usize],
        })
    }

    pub(crate) fn draw(&self, screen: &mut Screen) -> std::io::Result<()> {
        for i in (0..self.walls.len()).rev() {
            let left_field = " ".repeat(self.walls[i].0 as usize + 1);
            let right_field = " ".repeat((screen.size.0 - self.walls[i].1) as usize);
            screen.draw(
                Location(0, i as u16),
                left_field,
                Color::Black,
                Color::DarkGreen,
            )?;
            screen.draw(
                Location(self.walls[i].1, i as u16),
                right_field,
                Color::Black,
                Color::DarkGreen,
            )?;
        }
        Ok(())
    }
}
