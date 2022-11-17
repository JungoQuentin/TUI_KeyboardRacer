mod mloop;
mod render;
mod term;
mod utils;
use mloop::main_loop;
use render::render;
use std::io;
use term::{init_term, restore_term};

// TODO ou mettre ?
pub struct CeQueJeMetDansRender {
    sentence: String,
}

fn main() -> Result<(), io::Error> {
    let mut terminal = init_term()?;
    let render_infos = CeQueJeMetDansRender {
        sentence: String::from(""),
    };
    // render the first frame
    render(&mut terminal, &render_infos, 0.0);
    main_loop(render_infos, &mut terminal)?;
    // TODO plus besoin de render_infos en vrai ?
    restore_term(&mut terminal);
    Ok(())
}
