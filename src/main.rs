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
mod render;
use render::render;

fn main() -> Result<(), io::Error> {
    // ??
    enable_raw_mode()?;
    // variable 
    let mut stdout = io::stdout();
    // Met le fond noir -> rentre dans
    // un autre ecran
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // 
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut sentence = String::new();

    // render the first frame
    // All the other frame will be rendered after an action
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
            Event::Mouse(_e) => {
                //sentence.push('a');
                //render(&mut terminal, &sentence);

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

pub fn event_handler(code: KeyCode) -> Result<(), io::Error> {



    Ok(())
}

