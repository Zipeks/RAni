use crate::{
    app::App,
    app_helper_structs::{ActiveBlock, CurrentView, MediaTab},
    ui::content::draw_media_list,
};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph; 

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .split(area);

    if app.is_loading {
        let p = Paragraph::new("⏳ Waiting for AniList...").centered();
        frame.render_widget(p, area);
        return;
    }
    if let Some(ref err) = app.error_message {
        let p = Paragraph::new(format!("❌ API error: {}", err))
            .style(Style::default().fg(Color::Red))
            .centered();
        frame.render_widget(p, area);
        return;
    }

    let is_center_active = app.active_block == ActiveBlock::Center;
    
    let (media_items, active_state, active_tab) = match app.current_view {
        CurrentView::BrowseAnime => {
            let items = app.browse_anime.media.as_ref().and_then(|l| l.items.as_deref()).unwrap_or(&[]);
            (items, &mut app.browse_anime.state, MediaTab::Anime)
        }
        CurrentView::BrowseManga => {
            let items = app.browse_manga.media.as_ref().and_then(|l| l.items.as_deref()).unwrap_or(&[]);
            (items, &mut app.browse_manga.state, MediaTab::Manga)
        }
        _ => return,
    };

    draw_media_list::draw(
        frame,
        chunks[0],
        media_items,
        is_center_active,
        active_state,
        active_tab,
    );
}
