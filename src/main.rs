use ratatui::{
    DefaultTerminal, Frame, crossterm,
    layout::{Constraint, Layout},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let stories: Vec<Story> = fetch_hn();
    loop {
        terminal.draw(|frame| render(frame, &stories))?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, stories: &Vec<Story>) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(50)])
        .split(frame.area());

    frame.render_widget(format!("{}", stories[0].title), layout[0]);
    frame.render_widget(
        format!("{}", stories[0].url.as_deref().unwrap_or("(no url)")),
        layout[1],
    );
}

fn fetch_hn() -> Vec<Story> {
    let client = reqwest::blocking::Client::new();
    let ids: Vec<u64> = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .expect("Failed to fetch from hacker news")
        .json()
        .expect("Failed to parse story ids");

    let mut stories: Vec<Story> = vec![];

    for (_, id) in ids.iter().take(20).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
        let story: Story = client
            .get(&url)
            .send()
            .expect("Failed to fetch story")
            .json()
            .expect("Failed to parse story");

        stories.push(story);
    }

    stories
}
