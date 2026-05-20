use crate::{
    app::App,
    app_helper_structs::{ActiveBlock, BrowseCategory, CurrentView, MediaDetails},
    ui::content::draw_media_list,
};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

// use ratatui::widgets::Paragraph;

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Fill(1)])
        .split(area);

    if let Some(ref err) = app.error_message {
        let p = Paragraph::new(format!("❌ API error: {}", err))
            .style(Style::default().fg(Color::Red))
            .centered();
        frame.render_widget(p, area);
        return;
    }

    let is_active = app.active_block == ActiveBlock::Details;

    let active_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(Color::DarkGray);
    if let Some(media_details) = &app.media_details {
        let t = media_details.title.clone();
        let p = Paragraph::new(t).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if is_active {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::DarkGray)
                })
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(p, area);
    } else {
        let empty_block = Block::default()
            .borders(Borders::ALL)
            .border_style(if is_active {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .border_type(BorderType::Rounded);
        frame.render_widget(empty_block, area);
    }
}
