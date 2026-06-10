use crate::{app::App, ui::centered_rect};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let popup_area = centered_rect(60, 7, frame.area());

    frame.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .title(Line::from(" Delete media ").centered())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let delete_p = Paragraph::new(vec![
        Line::from(" Do you want to delete this media from your list? ").centered(),
        Line::from(" Y/N ").centered(),
    ])
    .block(popup_block)
    .wrap(Wrap { trim: false });

    frame.render_widget(delete_p, popup_area);
}
