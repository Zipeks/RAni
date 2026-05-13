use crate::app::{ActiveBlock, App, CurrentView};
use ratatui::{prelude::*, widgets::*};

mod home;
pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .borders(Borders::LEFT)
        .border_type(BorderType::Plain)
        .border_style(if app.active_block == ActiveBlock::Center {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        });

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    match app.current_view {
        CurrentView::Home => {
            home::draw(frame, inner_area, app);
        }
        _ => {}
    }
    // if app.is_loading {
    //     let loading_msg = Paragraph::new("⏳ Pobieranie z AniList...")
    //         .centered()
    //         .style(Style::default().fg(Color::Yellow));

    //     let margin_top = inner_area.height / 2;
    //     let centered_area = Rect {
    //         y: inner_area.y + margin_top,
    //         height: 1,
    //         ..inner_area
    //     };
    //     frame.render_widget(loading_msg, centered_area);
    // } else if let Some(ref err) = app.error_message {
    //     let err_msg = Paragraph::new(format!("❌ Błąd: {}", err))
    //         .wrap(Wrap { trim: true })
    //         .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

    //     frame.render_widget(err_msg, inner_area);
    // } else if let Some(ref anime_data) = app.current_anime {
    //     if let Some(ref media) = anime_data.media {
    //         let title = media
    //             .title
    //             .as_ref()
    //             .and_then(|t| t.romaji.clone())
    //             .unwrap_or_else(|| "Nieznany tytuł".to_string());

    //         let id = media.id;

    //         let text = vec![
    //             Line::from(vec![
    //                 Span::raw("Tytuł: "),
    //                 Span::styled(
    //                     title,
    //                     Style::default()
    //                         .fg(Color::White)
    //                         .add_modifier(Modifier::BOLD),
    //                 ),
    //             ]),
    //             Line::from(""),
    //             Line::from(format!("ID w bazie: {}", id)),
    //             // Line::from(format!("Status: {}", media.status...)),
    //         ];

    //         let details_widget = Paragraph::new(text)
    //             .wrap(Wrap { trim: true })
    //             .scroll((0, 0));

    //         frame.render_widget(details_widget, inner_area);
    //     } else {
    //         let not_found = Paragraph::new("Nie znaleziono anime pod tym ID.").centered();
    //         frame.render_widget(not_found, inner_area);
    //     }
    // } else {
    //     let empty_msg = Paragraph::new("Wciśnij [Enter] w menu, by załadować dane.")
    //         .centered()
    //         .style(Style::default().fg(Color::DarkGray));

    //     frame.render_widget(empty_msg, inner_area);
    // }
}
