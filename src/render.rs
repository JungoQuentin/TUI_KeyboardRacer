use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, /*Widget,*/ Paragraph},
    Terminal,
};

pub fn render<B: Backend>(terminal: &mut Terminal<B>, sentence: &String) {
    terminal
        .draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let title_block = Block::default()
                .title("KeyboardRacer")
                .borders(Borders::NONE)
                .style(
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD | Modifier::ITALIC),
                );

            let job_block = Block::default().borders(Borders::ALL);

            let p = Paragraph::new(vec![Spans::from(vec![Span::raw(sentence)])]).block(job_block);

            f.render_widget(title_block, layout[0]);
            f.render_widget(p, layout[1]);
        })
        .unwrap();
}
