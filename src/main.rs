use crate::game::{Difficulty, Game};
use crate::screen::Screen;

mod game;
mod screen;

fn main() -> std::io::Result<()> {
    let mut screen = Screen::new()?;
    screen.init()?;

    let mut game = Game::new(&mut screen, Difficulty::Easy)?;

    game.initiate()?;
    game.main_loop()?;
    game.terminate()?;

    screen.restore()?;
    Ok(())
}
