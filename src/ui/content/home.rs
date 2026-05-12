use crate::app::{ActiveBlock, App, CurrentView};
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    if app.is_loading {
        let loading_msg = Paragraph::new("⏳ Waiting for AniList...")
            .centered()
            .style(Style::default().fg(Color::Yellow));

        let margin_top = area.height / 2;
        let centered_area = Rect {
            y: area.y + margin_top,
            height: 1,
            ..area
        };
        frame.render_widget(loading_msg, centered_area);
    } else if let Some(ref err) = app.error_message {
        let err_msg = Paragraph::new(format!("❌ Error: {}", err))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

        frame.render_widget(err_msg, area);
    } else if let Some(ref anime_data) = app.current_media {
        let anime_list: Vec<ListItem> = anime_data
            .page
            .as_ref()
            .and_then(|p| p.media_list.as_ref())
            .map(|list| {
                list.iter()
                    .flatten() 
                    .filter_map(|item| {
                        let title = item
                            .media
                            .as_ref()?
                            .title
                            .as_ref()?
                            .user_preferred
                            .as_ref()?;

                        Some(ListItem::new(Line::from(vec![
                            Span::styled(" ● ", Style::default().fg(Color::Cyan)),
                            Span::raw(title.clone()),
                        ])))
                    })
                    .collect()
            })
            .unwrap_or_default();

        if anime_list.is_empty() {
            frame.render_widget(
                Paragraph::new("Empty list."),
                area,
            );
        } else {
            let list_widget = List::new(anime_list).block(
                Block::default()
                    .title(" Currently watching: ")
                    .borders(Borders::NONE)
                    .border_type(BorderType::Rounded),
            );
            frame.render_widget(list_widget, area);
        }
    }
}
