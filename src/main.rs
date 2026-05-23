use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{self, event::{Event, KeyCode}},
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

struct State {
    stories: Vec<Story>,
    selected: usize,
}


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let stories: Vec<Story> = fetch_hn();
    let mut state = State { stories, selected: 0 };
    ratatui::run(|terminal| app(terminal, &mut state))?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal, state: &mut State) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        match crossterm::event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }
                if key.code == KeyCode::Char('j') {
                    if state.selected != 0 {
                        state.selected -= 1;
                    }
                }
                if key.code == KeyCode::Char('k') {
                    if state.stories.get(state.selected + 1).is_some() {
                        state.selected += 1;
                    }
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, state: &mut State) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(50)])
        .split(frame.area());

    frame.render_widget(format!("{}", state.stories[state.selected].title), layout[0]);
    frame.render_widget(
        format!("{}", state.stories[state.selected].url.as_deref().unwrap_or("(no url)")),
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
