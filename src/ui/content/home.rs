use crate::{
    anilist::{GetCurrentMedia, get_current_media},
    app::{ActiveBlock, App, CurrentView},
};
use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
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
        let raw_list: Vec<_> = anime_data
            .page
            .as_ref()
            .and_then(|p| p.media_list.as_ref())
            .map(|l| l.iter().flatten().collect())
            .unwrap_or_default();

        let header_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);

        let mut list_items = Vec::new();

        let anime_entries: Vec<_> = raw_list
            .iter()
            .filter(|m| {
                m.media.as_ref().map_or(false, |med| {
                    med.type_ == Some(get_current_media::MediaType::ANIME)
                })
            })
            .collect();

        if !anime_entries.is_empty() {
            list_items.push(ListItem::new(
                Line::from("── ANIME ──────────────────").style(header_style),
            ));
            for entry in anime_entries {
                let title = extract_title(entry);
                list_items.push(ListItem::new(format!("  {}", title)));
            }
        }

        let manga_entries: Vec<_> = raw_list
            .iter()
            .filter(|m| {
                m.media.as_ref().map_or(false, |med| {
                    med.type_ == Some(get_current_media::MediaType::MANGA)
                })
            })
            .collect();

        if !manga_entries.is_empty() {
            if !list_items.is_empty() {
                list_items.push(ListItem::new(""));
            }

            list_items.push(ListItem::new(
                Line::from("── MANGA ──────────────────").style(header_style),
            ));
            for entry in manga_entries {
                let title = extract_title(entry);
                list_items.push(ListItem::new(format!("  {}", title)));
            }
        }

        let list_widget = List::new(list_items)
            .block(
                Block::default()
                    .title(" Currently Watching & Reading ")
                    .borders(Borders::NONE)
                    .border_style(if app.active_block == ActiveBlock::Center {
                        Style::default().fg(Color::Cyan)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }),
            )
            .highlight_symbol(">> ")
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list_widget, area, &mut app.current_media_state);
    }
}

fn extract_title(entry: &get_current_media::GetCurrentMediaPageMediaList) -> String {
    entry
        .media
        .as_ref()
        .and_then(|m| m.title.as_ref())
        .and_then(|t| t.user_preferred.clone())
        .unwrap_or_else(|| "Unknown Title".to_string())
}
