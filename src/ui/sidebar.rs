use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let sidebar_block = Block::default();

    let inner_sidebar_area = sidebar_block.inner(area);
    frame.render_widget(sidebar_block, area);

    let sidebar_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(inner_sidebar_area);

    let items: Vec<ListItem> = app
        .sidebar_items
        .iter()
        .map(|view| {
            let text = if *view == app.current_view {
                format!("* {}", view.to_string())
            } else {
                format!("  {}", view.to_string())
            };

            ListItem::new(text)
        })
        .collect();

    let list = List::new(items)
        .highlight_symbol("> ".red())
        .highlight_style(Style::default().yellow())
        .repeat_highlight_symbol(true)
        .scroll_padding(1);

    frame.render_stateful_widget(list, sidebar_layout[0], &mut app.sidebar_state);
}
