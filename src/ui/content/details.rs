use crate::{
    app::App,
    app_helper_structs::{ActiveBlock, BrowseCategory, CurrentView, MediaDetails},
    ui::content::draw_media_list,
};
use ratatui::prelude::*;

// use ratatui::widgets::Paragraph;

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App, media_details: MediaDetails) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Fill(1)])
        .split(area);
}
