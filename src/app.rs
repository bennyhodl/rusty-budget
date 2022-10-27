use std::error;
use tui::backend::Backend;
use tui::layout::{ Layout, Direction, Constraint, Alignment};
use tui::style::{Color, Style};
use tui::terminal::{Frame};
use tui::widgets::{Block, Borders, BorderType, Paragraph};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Rusty Budget")
            .title_alignment(Alignment::Center);
        frame.render_widget(block, frame.size());

        let inner_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(2)
            .split(frame.size());

        let left_block = Block::default()
            .title("Categories")
            .style(Style::default().bg(Color::Cyan));
        frame.render_widget(left_block, inner_chunk[0]);

        let right_block = Block::default()
            .title("Trnasactions")
            .style(Style::default().bg(Color::Green));
        frame.render_widget(right_block, inner_chunk[1]);

        let left_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .margin(1)
            .split(inner_chunk[0]);

        let p1 = Paragraph::new("Hi")
            .style(Style::default().fg(Color::Red));
        frame.render_widget(p1, left_chunk[0]);

        let p2 = Paragraph::new("Ben")
            .style(Style::default().fg(Color::Red));
        frame.render_widget(p2, left_chunk[1]);

        // let chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints([Constraint::Percentage(10), Constraint::Percentage(50), Constraint::Percentage(10)])
        //     .margin(1)
        //     .split(frame.size());
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples
        // let block = Block::default().style(Style::default().fg(Color::White).bg(Color::Black));
        // frame.render_widget(block, frame.size());
        // let paragraph1 = Paragraph::new("hey,howareya").style(Style::default().bg(Color::Blue));
        // frame.render_widget(paragraph1, chunks[0]);
        // let paragraph2 = Paragraph::new("sup,dude");
        // frame.render_widget(paragraph2, chunks[1]);
        // let paragraph3 = Paragraph::new("sup, fabio");
        // frame.render_widget(paragraph3, chunks[2]);
        // let paragraph2 = Paragraph::new("sup,wes");
        // frame.render_widget(paragraph2, chunks[3]);
    }
}
