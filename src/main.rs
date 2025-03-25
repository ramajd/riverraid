use crate::screen::Screen;

mod screen;

fn main() -> std::io::Result<()> {
    let mut screen = Screen::new()?;
    screen.init()?;

    screen.restore()?;
    Ok(())
}
