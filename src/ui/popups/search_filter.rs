use crate::{app::App, ui::centered_rect};
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let popup_area = centered_rect(55, 14, frame.area());
    frame.render_widget(Clear, popup_area);

    let title = if app.is_in_edit_state {
        " Edit Filter Field [Enter/Esc: Finish] "
    } else {
        " Search & Filters [i/Enter: Edit | s: Save | r: Reset | Esc: Close] "
    };

    let border_color = if app.is_in_edit_state {
        Color::Green
    } else {
        Color::Yellow
    };

    let popup_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color));

    let mut items = vec![];
    let filter = app.get_current_filter();

    let fields = [
        format!("Search Query: {}", app.filter_search_text),
        format!(
            "Sort:         {}",
            filter
                .sort
                .as_ref()
                .and_then(|v| v.first().cloned())
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Default".to_string())
        ),
        format!(
            "Format:       {}",
            filter
                .format
                .map(|f| f.to_string())
                .unwrap_or_else(|| "Any".to_string())
        ),
        format!(
            "Season:       {}",
            filter
                .season
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Any".to_string())
        ),
        format!(
            "Status:       {}",
            filter
                .media_status
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Any".to_string())
        ),
        format!("Year:         {}", app.filter_year_text),
    ];

    for (i, field) in fields.iter().enumerate() {
        let is_selected = i == app.filter_popup_index;
        let mut style = Style::default().fg(Color::White);
        let mut prefix = "   ";

        if is_selected {
            prefix = " > ";
            if app.is_in_edit_state {
                style = Style::default()
                    .bg(Color::Green)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            } else {
                style = Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }
        }

        items.push(ListItem::new(format!("{}{}", prefix, field)).style(style));
    }

    let list = List::new(items).block(popup_block);
    frame.render_widget(list, popup_area);
}
