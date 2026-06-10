use crate::{app::App, app_helper_structs::ActiveBlock};

use ratatui::{prelude::*, widgets::*};

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let binds = match app.active_block {
        ActiveBlock::Sidebar => vec!["Up: k", "Down: j", "Right: l"],
        ActiveBlock::Center => vec![
            "Up: k",
            "Down: j",
            "Sidebar: h",
            "Details: l",
            "Next page: n",
            "Prev page: p",
            "Category: ]/Tab",
        ],
        ActiveBlock::Details => vec![
            "Up: k",
            "Down: j",
            "Center: h",
            "Edit status: e",
            "Toggle favourite: f",
            "Delete media: d",
            "Open anilist: o",
        ],
    };

    let joined_text = binds.join(" | ");

    let info = vec![Span::raw("  "), Span::raw(joined_text)];

    let keybinds_info = Paragraph::new(Line::from(info)).left_aligned();
    frame.render_widget(keybinds_info, area);
}
