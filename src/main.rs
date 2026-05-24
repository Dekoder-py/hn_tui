use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyModifiers},
    },
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::Paragraph,
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
    show_help: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let stories: Vec<Story> = fetch_hn();
    let mut state = State {
        stories,
        selected: 0,
        show_help: true,
    };
    ratatui::run(|terminal| app(terminal, &mut state))?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal, state: &mut State) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        match crossterm::event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q')
                    || ((key.code == KeyCode::Char('c') || key.code == KeyCode::Char('d'))
                        && key.modifiers.contains(KeyModifiers::CONTROL))
                {
                    break Ok(());
                }
                if key.code == KeyCode::Char('k') {
                    if state.selected != 0 {
                        state.selected -= 1;
                    }
                }
                if key.code == KeyCode::Char('j') {
                    if state.stories.get(state.selected + 1).is_some() {
                        state.selected += 1;
                    }
                }
                if key.code == KeyCode::Char('o') && state.stories[state.selected].url.is_some() {
                    open::that(state.stories[state.selected].url.as_deref().unwrap())?;
                }
                if key.code == KeyCode::Char('?') {
                    state.show_help = !state.show_help;
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, state: &mut State) {
    if state.show_help {
        let outer_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(80),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ])
            .split(frame.area());

        frame.render_widget(Paragraph::new("Hacker News TUI\n Help: \n '?' to hide/show help. \n 'j'/'k' to scroll. \n 'o' to open url.").cyan(), outer_layout[1]);
    }

    let outer_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(5), Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(outer_layout[0]);

    let story = &state.stories[state.selected];

    frame.render_widget(
        format!(
            "  {}, by {}. (HN Score: {})",
            story.title, story.by, story.score
        ),
        inner_layout[1],
    );

    frame.render_widget(
        format!("  {}", story.url.as_deref().unwrap_or("(no url)")),
        inner_layout[2],
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
