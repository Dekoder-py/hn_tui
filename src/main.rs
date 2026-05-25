use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyModifiers},
    },
    layout::{self, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Paragraph, Row, Table, Wrap},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
    kids: Option<Vec<usize>>,
}

#[derive(Deserialize)]
struct Comment {
    text: String,
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
                if key.code == KeyCode::Char('c') {
                    let comment = fetch_comments(&state.stories[state.selected]);
                    if comment.is_some() {
                        println!("{}", comment.unwrap_or(Comment {text: "".to_string(), by: "".to_string()}).text);
                    }
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, state: &mut State) {
    let outer_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(frame.area());

    if state.show_help {
        let help_header = Row::new(["Key", "Action"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let help_rows = [
            Row::new(["?", "Show/Hide Help"]),
            Row::new(["j", "Scroll down"]),
            Row::new(["k", "Scroll up"]),
            Row::new(["o", "Open URL"]),
            Row::new(["q", "Quit"]),
        ];

        let widths = [Constraint::Percentage(20), Constraint::Percentage(80)];

        let help_table = Table::new(help_rows, widths)
            .header(help_header)
            .column_spacing(1)
            .style(Color::Cyan);

        frame.render_widget(help_table, outer_layout[1]);
    }

    let inner_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(5),
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(outer_layout[0]);

    let story = &state.stories[state.selected];

    let story_p = Paragraph::new(format!(
        "{}, by {}. (HN Score: {})",
        story.title, story.by, story.score
    ))
    .wrap(Wrap { trim: false });

    let link_p = Paragraph::new(format!("{}", story.url.as_deref().unwrap_or("(no url)")))
        .wrap(Wrap { trim: false });

    frame.render_widget(story_p, inner_layout[1]);

    frame.render_widget(link_p, inner_layout[2]);
}

fn fetch_hn() -> Vec<Story> {
    let client = reqwest::blocking::Client::new();
    println!("Fetching Hacker News stories... please wait :)");
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

fn fetch_comments(story: &Story) -> Option<Comment> {
    if story.kids.is_some() {
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://hacker-news.firebaseio.com/v0/item/{}.json",
            story.kids.as_deref().unwrap()[0]
        );

        let comment: Comment = client
            .get(&url)
            .send()
            .expect("Failed to fetch comment")
            .json()
            .expect("Failed to parse comment");
        Some(comment)
    } else {
        None
    }
}
