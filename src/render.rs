use tui::{
    backend::{Backend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, /*Widget,*/ Paragraph},
    Terminal,
};

fn main_block(sentence: &String) -> Paragraph {
    let main_block = Block::default().borders(Borders::ALL);
    let p = Paragraph::new(vec![Spans::from(vec![Span::raw(sentence)])]).block(main_block);
    return p;
}

fn title_block() -> Block<'static> {
   let title_block = Block::default()
       .title("KeyboardRacer")
       .borders(Borders::NONE)
       .style(
           Style::default()
               .fg(Color::Blue)
               .add_modifier(Modifier::BOLD | Modifier::ITALIC),
       );
   return title_block;
}

fn set_vertical_layout<B: Backend>(split: Vec<u16>, frame: &mut tui::Frame<B>) -> Vec<tui::layout::Rect> {
    let mut constraints:Vec<Constraint> = Vec::new();
    for s in split {
        constraints.push(Constraint::Percentage(s));
    }
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(constraints.as_ref())
                .split(frame.size());
            return layout;
}

pub fn render<B: Backend>(terminal: &mut Terminal<B>, sentence: &String) {
    terminal
        .draw(|frame| {
            let layout = set_vertical_layout(vec![5, 80], frame);

            // Render blocks
            frame.render_widget(title_block(), layout[0]);
            frame.render_widget(main_block(sentence), layout[1]);
        })
        .expect("It didn't render. Why ?");
}
