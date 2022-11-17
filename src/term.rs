use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{self, backend::CrosstermBackend, Terminal};

pub fn init_term() -> Result<Terminal<tui::backend::CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    // variable
    let mut stdout = io::stdout();
    // Met le fond noir -> rentre dans
    // un autre ecran
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal: Terminal<tui::backend::CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_term(terminal: &mut Terminal<tui::backend::CrosstermBackend<io::Stdout>>) {
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}
