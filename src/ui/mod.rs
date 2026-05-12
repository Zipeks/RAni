mod header;
mod sidebar;
mod content;

use ratatui::prelude::*;
use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(frame.area());

    header::draw(frame, chunks[0], app);


}