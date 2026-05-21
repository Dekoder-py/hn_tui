use std::io::Error;

use ratatui::{DefaultTerminal, Frame, crossterm};
use serde::Deserialize;

#[derive(Deserialize)]
struct HNData {
    id: String,
    url: String,
    title: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello", frame.area());
}

fn fetch_hn() -> Result<String, Error> {
    Ok("hi".to_string())
}
