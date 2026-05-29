use crate::app_helper_structs::CurrentEditField;
use crate::{app::App, ui::centered_rect};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let popup_area = centered_rect(55, 16, frame.area());
    frame.render_widget(Clear, popup_area);

    let title = if app.is_in_edit_state {
        " Edit Mode [Enter/Esc: Exit] "
    } else {
        " Normal Mode [i/Enter: Edit | s: Save | q: Quit] "
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

    if let Some(edited) = &app.edited_media {
        let fields = app.get_current_edit_fields();

        for (i, field) in fields.iter().enumerate() {
            let is_selected = i == app.edit_popup_index;

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

            let text = match field {
                CurrentEditField::Status => format!("Status:  {}", edited.status.to_string()),
                CurrentEditField::EpisodeProgress => {
                    let lbl = if app.edit_is_manga {
                        "Chapters:"
                    } else {
                        "Episodes:"
                    };
                    format!("{lbl} {}", edited.progress)
                }
                CurrentEditField::VolumeProgress => {
                    format!("Volumes:  {}", edited.progress_volumes.unwrap_or(0))
                }
                CurrentEditField::Score => format!("Score:    {}", edited.score),
                CurrentEditField::Rewatch => {
                    let lbl = if app.edit_is_manga {
                        "Reread:  "
                    } else {
                        "Rewatch: "
                    };
                    format!("{lbl} {}", edited.repeat)
                }
                CurrentEditField::StartDate => format!("Start:    {}", app.edit_start_date_text),
                CurrentEditField::EndDate => format!("End:      {}", app.edit_end_date_text),
                CurrentEditField::Notes => format!("Notes:    {}", edited.notes),
            };

            items.push(ListItem::new(format!("{}{}", prefix, text)).style(style));
        }

        items.push(ListItem::new(""));
        items.push(
            ListItem::new(" (h/l to quickly change values in Normal Mode) ")
                .style(Style::default().fg(Color::DarkGray)),
        );
    }

    let list = List::new(items).block(popup_block);
    frame.render_widget(list, popup_area);
}
