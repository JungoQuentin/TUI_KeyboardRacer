use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Instant;
use tui::{self, backend::CrosstermBackend, Terminal};
mod render;
use render::render;

fn init_term() -> Result<Terminal<tui::backend::CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    // variable
    let mut stdout = io::stdout();
    // Met le fond noir -> rentre dans
    // un autre ecran
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    //
    let backend = CrosstermBackend::new(stdout);
    let terminal: Terminal<tui::backend::CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    Ok(terminal)
}

pub struct CeQueJeMetDansRender {
    sentence: String,
}

fn restore_term(terminal: &mut Terminal<tui::backend::CrosstermBackend<io::Stdout>>) {
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}

fn delta_time(start: Instant) -> f64 {
    let end = Instant::now();
    let delta = end.duration_since(start).as_secs_f64();
    delta
}
fn main() -> Result<(), io::Error> {
    let mut terminal = init_term()?;
    let mut render_infos = CeQueJeMetDansRender {
        sentence: String::from(""),
    };
    // render the first frame
    // All the other frame will be rendered after an action
    render(&mut terminal, &render_infos, 0.0);
    let mut last_render = std::time::Instant::now();

    loop {
        match read()? {
            Event::Key(e) => {
                if event_handler(e.code, &mut render_infos)? == false {
                    break;
                } else {
                    render(
                        &mut terminal,
                        &render_infos,
                        delta_time(last_render),
                    );
                    last_render = Instant::now();
                }
            }
            _ => (),
        }
    }
    restore_term(&mut terminal);
    Ok(())
}

pub fn event_handler(
    code: KeyCode,
    render_infos: &mut CeQueJeMetDansRender,
) -> Result<bool, io::Error> {
    match code {
        KeyCode::Esc => return Ok(false),
        KeyCode::Char(c) => render_infos.sentence.push(c),
        KeyCode::Backspace => {
            render_infos.sentence.pop();
        }
        _ => (),
    }
    Ok(true)
}
