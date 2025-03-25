use crate::game::{Difficulty, Game};
use crate::screen::Screen;

mod game;
mod screen;

fn main() -> std::io::Result<()> {
    let mut sc = Screen::new()?;
    sc.init()?;

    let mut game = Game::new(&mut sc, Difficulty::Easy)?;

    game.initiate()?;
    game.main_loop()?;
    game.terminate()?;

    sc.restore()?;
    Ok(())
}
