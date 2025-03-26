use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveTo},
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear},
};
use std::io::{Stdout, Write, stdout};

#[derive(Debug, Clone, Copy)]
pub struct Location(pub u16, pub u16);

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
        terminal::disable_raw_mode()?;
        self.screen.execute(cursor::Show)?.execute(ResetColor)?;
        Ok(())
    }

    pub fn clear(&mut self, immediate: bool) -> std::io::Result<()> {
        self.screen.queue(Clear(terminal::ClearType::All))?;
        if immediate {
            self.screen.flush()?;
        }
        Ok(())
    }

    pub(crate) fn draw(
        &mut self,
        location: Location,
        data: String,
        foreground: crossterm::style::Color,
        background: crossterm::style::Color,
    ) -> std::io::Result<()> {
        let Location(col, row) = location;
        self.screen
            .queue(MoveTo(col, row))?
            .queue(SetForegroundColor(foreground))?
            .queue(SetBackgroundColor(background))?
            .queue(Print(data))?;
        Ok(())
    }

    pub(crate) fn flush(&mut self) -> std::io::Result<()> {
        self.screen.flush()?;
        Ok(())
    }
}
