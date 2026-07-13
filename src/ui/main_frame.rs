use crate::{
    app::App,
    app_helper_structs::ActiveBlock,
    ui::{
        content::{browse, details},
        sidebar,
    },
};
use ratatui::{prelude::*, widgets::*};
pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_main_area = main_block.inner(area);
    frame.render_widget(main_block, area);

    let stack_len = app.view_stack.len();

    let constraints = match app.active_block {
        ActiveBlock::Sidebar | ActiveBlock::Center => [
            Constraint::Length(20),
            Constraint::Fill(6),
            Constraint::Fill(2),
        ],
        ActiveBlock::Details => {
            if stack_len > 1 {
                [
                    Constraint::Length(0),
                    Constraint::Percentage(45),
                    Constraint::Percentage(55),
                ]
            } else {
                [
                    Constraint::Length(0),
                    Constraint::Fill(3),
                    Constraint::Fill(4),
                ]
            }
        }
    };

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(inner_main_area);

    sidebar::draw(frame, center[0], app);

    if stack_len > 1 && app.active_block == ActiveBlock::Details {
        details::draw(frame, center[1], app, stack_len - 2, false);

        details::draw(frame, center[2], app, stack_len - 1, true);
    } else {
        browse::draw(frame, center[1], app);

        if stack_len > 0 {
            let is_details_active = app.active_block == ActiveBlock::Details;
            details::draw(frame, center[2], app, 0, is_details_active);
        } else {
            let empty_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .border_type(BorderType::Rounded)
                .title(" Details ");
            frame.render_widget(empty_block, center[2]);
        }
    }
}
