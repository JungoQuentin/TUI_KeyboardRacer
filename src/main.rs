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

fn render<B: Backend>(terminal: &mut Terminal<B>, sentence: &String) {
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

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut sentence = String::new();

    // programme loop
    render(&mut terminal, &sentence);
    loop {
        match read()? {
            Event::Key(e) => {
                match e.code {
                    KeyCode::Char(c) => sentence.push(c),
                    KeyCode::Esc => break,
                    KeyCode::Backspace => {
                        sentence.pop();
                    }
                    _ => (),
                }
                render(&mut terminal, &sentence);
            }
            _ => (),
        }
    }

    // restore terminal when the loop is breaked
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
