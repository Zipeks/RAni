use crate::{app::App, ui::centered_rect};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let popup_area = centered_rect(60, 18, frame.area());

    frame.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .title(" Favourite ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red));

    let favourite_p = Paragraph::new(vec![
        Line::from(" Do you want to change favourite status? "),
        Line::from(" y/n "),
    ])
    .block(popup_block)
    .wrap(Wrap { trim: false });

    frame.render_widget(favourite_p, popup_area);
}
