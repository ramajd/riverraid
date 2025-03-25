use crossterm::{ExecutableCommand, cursor, terminal};
use std::io::{Stdout, stdout};

#[derive(Debug)]
pub struct Screen {
    screen: Stdout,
    pub size: (u16, u16),
}

impl Screen {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            screen: stdout(),
            size: terminal::size()?,
        })
    }

    pub fn init(&mut self) -> std::io::Result<()> {
        terminal::enable_raw_mode()?;
        self.screen
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::Hide)?;
        Ok(())
    }

    pub fn restore(&mut self) -> std::io::Result<()> {
        self.screen.execute(cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
