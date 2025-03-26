use crossterm::style::Color;

use crate::screen::Location;

#[derive(Debug)]
pub struct Fuel {
    pub location: Location,
}

const FUEL: [(&str, Color); 4] = [
    ("▟█F█▙", Color::AnsiValue(201)),
    ("██U██", Color::White),
    ("██E██", Color::AnsiValue(201)),
    ("██L██", Color::White),
];

impl Fuel {
    pub fn new(col: u16, row: u16) -> Self {
        Self {
            location: Location(col, row),
        }
    }

    pub fn draw(&self, screen: &mut crate::screen::Screen) -> std::io::Result<()> {
        for (i, line) in FUEL.iter().rev().enumerate() {
            screen.draw(
                Location(self.location.0, self.location.1 - i as u16),
                line.0.to_string(),
                line.1,
                crossterm::style::Color::Blue,
            )?;
        }
        Ok(())
    }
}
