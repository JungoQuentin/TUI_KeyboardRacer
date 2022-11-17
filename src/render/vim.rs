use crate::CeQueJeMetDansRender;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    //symbols::Marker,
    text::{Span, Spans},
    widgets::{canvas::*, Block, Borders, Paragraph},
    Terminal,
};

// TODO ailleurs ?
const BLOCK_CHAR: char = '\u{2588}';

fn title_block() -> Block<'static> {
    let title_block = Block::default()
        .title("KeyboardRacer")
        .borders(Borders::NONE)
        .style(
            Style::default()
                .fg(Color::Blue)
                .bg(Color::Black)
                .add_modifier(
                    Modifier::BOLD | Modifier::ITALIC, /*| Modifier::RAPID_BLINK */
                ),
        );
    title_block
}

fn main_block(sentence: &String, display_block: bool) -> Paragraph {
    let main_block = Block::default().borders(Borders::ALL);
    let mut spans = vec![Span::styled(
        sentence,
        Style::default().fg(Color::White).bg(Color::Black), //           .add_modifier(Modifier::BOLD),
    )];
    if display_block {
        spans.push(Span::raw(BLOCK_CHAR.to_string()));
    }
    Paragraph::new(vec![Spans::from(spans)])
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(main_block)
}

fn set_vertical_layout<B: Backend>(
    split: Vec<u16>,
    frame: &mut tui::Frame<B>,
) -> Vec<tui::layout::Rect> {
    let mut constraints: Vec<Constraint> = Vec::new();
    for s in split {
        constraints.push(Constraint::Percentage(s));
    }
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints.as_ref())
        .split(frame.size());
    layout
}

pub fn render<B: Backend>(
    terminal: &mut Terminal<B>,
    render_infos: &CeQueJeMetDansRender,
    _delta: f64,
) {
    terminal
        .draw(|frame| {
            let layout = set_vertical_layout(vec![5, 50, 45], frame);

            // Render blocks
            let main_block = main_block(&render_infos.sentence, true);
            frame.render_widget(title_block(), layout[0]);
            frame.render_widget(main_block, layout[1]);
        })
        .expect("It didn't render. Why ?");
}
