use crate::{
    app::App,
    app_helper_structs::{ActiveBlock, MediaStatus, Season, TitleLanguage},
};
use ratatui::{prelude::*, widgets::*};
use ratatui_image::StatefulImage;

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let is_active = app.active_block == ActiveBlock::Details;

    let details_block = Block::default()
        .borders(Borders::ALL)
        .border_style(if is_active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .border_type(BorderType::Rounded)
        .title(" Details ").padding(Padding::proportional(1));

    let inner_details_area = details_block.inner(area);
    frame.render_widget(details_block, area);

    if let Some(ref err) = app.error_message {
        let p = Paragraph::new(format!("❌ API error: {}", err))
            .style(Style::default().fg(Color::Red))
            .centered();
        frame.render_widget(p, inner_details_area);
        return;
    } else if let Some(media_details) = &app.media_details {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(14),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .split(inner_details_area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .spacing(2)
            .constraints([
                Constraint::Length(25),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(vertical_chunks[0]);

        let image_area = top_chunks[0];

        let media_id = app
            .browse_state
            .state
            .selected()
            .and_then(|idx| app.get_current_center_items().get(idx))
            .map(|item| item.id)
            .unwrap_or(0);

        if let Some(image_protocol) = app.image_cache.get_mut(&media_id) {
            let image_widget = StatefulImage::default();
            frame.render_stateful_widget(image_widget, image_area, image_protocol);
        } else if app.currently_fetching_image == Some(media_id) {
            frame.render_widget(Paragraph::new("⏳").centered(), image_area);
        } else {
            frame.render_widget(Paragraph::new("").centered(), image_area);
        }

        let info_area = top_chunks[1];

        let season_str = media_details.season.to_string();

        let status_str = media_details.media_status.to_string();

        let total_str = media_details
            .total
            .map(|t| t.to_string())
            .unwrap_or_else(|| "?".to_string());

        let label_style = Style::default().fg(Color::DarkGray);

        let info_lines = vec![
            Line::from(media_details.titles.get_title(&app.title_language)).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from(Span::styled(
                format!(
                    "{}, {}, {}",
                    media_details.titles.get_title(&TitleLanguage::Native),
                    media_details.titles.get_title(&TitleLanguage::Romaji),
                    media_details.titles.get_title(&TitleLanguage::English)
                ),
                label_style,
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Status:  ", label_style),
                Span::raw(status_str),
            ]),
            Line::from(vec![
                Span::styled("Season:   ", label_style),
                Span::raw(format!("{} {}", season_str, media_details.season_year)),
            ]),
            Line::from(vec![
                Span::styled("Episodes: ", label_style),
                Span::raw(total_str),
            ]),
            Line::from(vec![
                Span::styled("Score:   ", label_style),
                Span::raw(format!("{} / 100", media_details.average_score)),
            ]),
        ];

        let info_paragraph = Paragraph::new(info_lines).wrap(Wrap { trim: true });
        frame.render_widget(info_paragraph, info_area);

        if let Some(user_media_detials) = &media_details.user_media_details {
            let user_info_lines = vec![
                Line::from(""),
                Line::from(""),
                Line::from(Span::styled(
                    format!("Your status: {}", user_media_detials.status.to_string()),
                    label_style,
                )),
                Line::from(Span::styled(
                    format!("Progress: {}", user_media_detials.progress),
                    label_style,
                )),
                Line::from(vec![
                    Span::styled("Your score: ", label_style),
                    Span::raw(format!("{}", user_media_detials.score)),
                ]),
            ];

            let user_info_paragraph = Paragraph::new(user_info_lines).wrap(Wrap { trim: true });
            frame.render_widget(user_info_paragraph, top_chunks[2]);
        }

        let desc_area = vertical_chunks[2];

        let clean_desc = media_details
            .description
            .replace("<br>\n", "\n")
            .replace("<br>", "\n")
            .replace("<i>", "")
            .replace("</i>", "")
            .replace("<b>", "")
            .replace("</b>", "");

        let description: Vec<Line> = clean_desc.lines().map(|t| Line::from(t)).collect();

        let desc_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Description ");

        let p = Paragraph::new(description)
            .block(desc_block)
            .scroll((0, 0))
            .wrap(Wrap { trim: false });

        frame.render_widget(p, desc_area);
    }
}
