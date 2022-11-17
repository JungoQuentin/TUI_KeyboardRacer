use crate::{render, utils::delta_time, CeQueJeMetDansRender};
use crossterm::event::{read, Event, KeyCode};
use std::io;
use std::time::Instant;

enum Scene {
    Menu,
    Dactilo,
    Vim
}

pub fn main_loop(
    mut render_info: CeQueJeMetDansRender,
    terminal: &mut tui::Terminal<tui::backend::CrosstermBackend<io::Stdout>>,
) -> Result<(), io::Error> {
    // All the other frame will be rendered after an action
    let mut last_render = std::time::Instant::now();
    loop {
        match read()? {
            Event::Key(e) => {
                if event_handler(e.code, &mut render_info)? == false {
                    break;
                } else {
                    render(terminal, &render_info, delta_time(last_render));
                    last_render = Instant::now();
                }
            }
            _ => (),
        }
    }
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
