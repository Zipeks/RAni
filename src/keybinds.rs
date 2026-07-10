use crate::{
    anilist::client::AnilistClient,
    app::{App, AppAction},
    app_helper_structs::{
        ActiveBlock,
        ActivePopup::{self},
        BrowseCategory, CurrentView, TitleLanguage,
    },
};
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use std::sync::mpsc::Sender;
use tracing::info;

pub fn handle_sidebar_events(
    app: &mut App,
    key: KeyEvent,
    client: AnilistClient,
    tx: Sender<AppAction>,
) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => app.next_sidebar_item(),
        KeyCode::Char('k') | KeyCode::Up => app.previous_sidebar_item(),
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            if let Some(selected_idx) = app.sidebar_state.selected() {
                let new_view = app.sidebar_items[selected_idx];

                if new_view != app.current_view {
                    app.current_view = new_view;
                    app.browse_state.current_category = BrowseCategory::CategoryOne;
                    app.browse_state.media = None;
                }
            }
            app.active_block = ActiveBlock::Center;

            match app.current_view {
                CurrentView::UserAnime | CurrentView::UserManga => app.fetch_user_media(client, tx),
                CurrentView::BrowseAnime | CurrentView::BrowseManga => app.fetch_browse(client, tx),
            }
        }
        _ => {}
    }
}

pub fn handle_center_events(
    app: &mut App,
    key: KeyEvent,
    client: AnilistClient,
    tx: Sender<AppAction>,
) {
    match key.code {
        KeyCode::Char('h') | KeyCode::Left | KeyCode::Esc => {
            app.active_block = ActiveBlock::Sidebar;
            app.unset_error();
        }

        KeyCode::Char('[') | KeyCode::Char(']') | KeyCode::BackTab | KeyCode::Tab => {
            if key.code == KeyCode::Char('[') || key.code == KeyCode::BackTab {
                app.browse_state.current_category = app.browse_state.current_category.previous();
            } else {
                app.browse_state.current_category = app.browse_state.current_category.next();
            }

            app.browse_state.media = None;

            let tx_clone = tx.clone();
            match app.current_view {
                CurrentView::UserAnime | CurrentView::UserManga => {
                    app.fetch_user_media(client, tx_clone)
                }
                CurrentView::BrowseAnime | CurrentView::BrowseManga => {
                    app.fetch_browse(client, tx_clone)
                }
            }
        }

        KeyCode::Char('j') | KeyCode::Down => app.next_center_item(),
        KeyCode::Char('k') | KeyCode::Up => app.previous_center_item(),
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
            if let Some(selected_index) = app.browse_state.state.selected() {
                let current_items = app.get_current_center_items();

                if selected_index < current_items.len() {
                    app.fetch_media_details(client, tx);
                    app.active_block = ActiveBlock::Details;
                }
            }
        }
        KeyCode::Char('n') => {
            app.next_center_page();
            match app.current_view {
                CurrentView::BrowseAnime | CurrentView::BrowseManga => app.fetch_browse(client, tx),
                CurrentView::UserAnime | CurrentView::UserManga => app.fetch_user_media(client, tx),
            }
        }
        KeyCode::Char('p') => {
            app.previous_center_page();
            match app.current_view {
                CurrentView::BrowseAnime | CurrentView::BrowseManga => app.fetch_browse(client, tx),
                CurrentView::UserAnime | CurrentView::UserManga => app.fetch_user_media(client, tx),
            }
        }
        KeyCode::Char('t') => {
            app.active_popup = Some(ActivePopup::TitleLanguage);
            app.language_popup_index = TitleLanguage::ALL
                .iter()
                .position(|l| l == &app.title_language)
                .unwrap_or(0);
        }
        KeyCode::Char('f') => match app.current_view {
            CurrentView::BrowseAnime
            | CurrentView::UserAnime
            | CurrentView::BrowseManga
            | CurrentView::UserManga => app.open_filter_popup(),
        },
        KeyCode::Char('r') => {
            let is_user_view = matches!(
                app.current_view,
                CurrentView::UserAnime | CurrentView::UserManga
            );

            if is_user_view {
                app.reset_current_user_filter();
                app.fetch_user_media(client, tx);
            } else {
                app.reset_current_filter();
                app.fetch_browse(client, tx);
            }
        }
        _ => {}
    }
}

pub fn handle_details_events(
    app: &mut App,
    key: KeyEvent,
    client: AnilistClient,
    tx: Sender<AppAction>,
) {
    match key.code {
        KeyCode::Char('h') | KeyCode::Left => {
            app.active_block = ActiveBlock::Center;
            app.media_details = None;
        }
        KeyCode::Char('j') | KeyCode::Down | KeyCode::Char('k') | KeyCode::Up => {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => app.next_center_item(),
                KeyCode::Char('k') | KeyCode::Up => app.previous_center_item(),
                _ => {}
            }
            if let Some(selected_index) = app.browse_state.state.selected() {
                let current_items = app.get_current_center_items();

                if selected_index < current_items.len() {
                    app.fetch_media_details(client, tx);
                    app.active_block = ActiveBlock::Details;
                }
            }
        }
        KeyCode::Char('e') => {
            app.open_edit_popup();
        }
        KeyCode::Char('f') => {
            app.active_popup = Some(ActivePopup::Favourite);
        }
        KeyCode::Char('d') => {
            app.active_popup = Some(ActivePopup::DeleteMedia);
        }
        KeyCode::Char('o') => {
            app.open_anilist();
        }
        _ => {}
    }
}

pub fn handle_language_popup_events(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.active_popup = None,

        KeyCode::Tab | KeyCode::Down | KeyCode::Char('j') => {
            app.language_popup_index = (app.language_popup_index + 1) % 4;
        }

        KeyCode::BackTab | KeyCode::Up | KeyCode::Char('k') => {
            app.language_popup_index = (app.language_popup_index + 3) % 4;
        }

        KeyCode::Enter => {
            app.title_language =
                crate::app_helper_structs::TitleLanguage::ALL[app.language_popup_index];
            app.active_popup = None;
        }
        _ => {}
    }
}

pub fn handle_error_popup_events(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => app.unset_error(),
        _ => {}
    }
}
pub fn handle_favourite_popup_events(
    app: &mut App,
    key: KeyEvent,
    client: AnilistClient,
    tx: Sender<AppAction>,
) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('n') => app.active_popup = None,
        KeyCode::Char('y') | KeyCode::Enter => {
            app.fetch_toggle_favourite(client, tx);
            app.active_popup = None
        }
        _ => {}
    }
}
pub fn handle_delete_media_popup_events(
    app: &mut App,
    key: KeyEvent,
    client: AnilistClient,
    tx: Sender<AppAction>,
) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('n') => app.active_popup = None,
        KeyCode::Char('y') | KeyCode::Enter => {
            app.fetch_delete_media(client, tx);
            app.active_popup = None
        }
        _ => {}
    }
}
pub fn handle_edit_media_popup_events(
    app: &mut App,
    key: KeyEvent,
    client: AnilistClient,
    tx: Sender<AppAction>,
) {
    use crate::app_helper_structs::CurrentEditField;
    use ratatui::crossterm::event::KeyCode;

    let fields = app.get_current_edit_fields();
    let current_field = fields[app.edit_popup_index];

    if app.is_in_edit_state {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                app.is_in_edit_state = false;
            }
            KeyCode::Backspace => {
                if let Some(media) = &mut app.edited_media {
                    match current_field {
                        CurrentEditField::EpisodeProgress => media.progress /= 10,
                        CurrentEditField::VolumeProgress => {
                            if let Some(v) = media.progress_volumes {
                                media.progress_volumes = Some(v / 10);
                            }
                        }
                        CurrentEditField::Score => media.score = (media.score / 10.0).trunc(),
                        CurrentEditField::Rewatch => media.repeat /= 10,
                        CurrentEditField::StartDate => {
                            app.edit_start_date_text.pop();
                        }
                        CurrentEditField::EndDate => {
                            app.edit_end_date_text.pop();
                        }
                        CurrentEditField::Notes => {
                            media.notes.pop();
                        }
                        CurrentEditField::Status => {}
                    }
                }
            }
            KeyCode::Char(c) => {
                if let Some(media) = &mut app.edited_media {
                    match current_field {
                        CurrentEditField::EpisodeProgress => {
                            if let Some(digit) = c.to_digit(10) {
                                media.progress = (media.progress * 10 + (digit as i64)).clamp(
                                    0,
                                    app.media_details.as_ref().unwrap().total.unwrap_or(20000),
                                );
                            }
                        }
                        CurrentEditField::VolumeProgress => {
                            if let Some(digit) = c.to_digit(10) {
                                let vols = media.progress_volumes.unwrap_or(0);
                                media.progress_volumes = Some(
                                    vols * 10
                                        + (digit as i64).clamp(
                                            0,
                                            app.media_details
                                                .as_ref()
                                                .unwrap()
                                                .volumes
                                                .unwrap_or(20000),
                                        ),
                                );
                            }
                        }
                        CurrentEditField::Score => {
                            if let Some(digit) = c.to_digit(10) {
                                media.score = (media.score * 10.0 + digit as f64).clamp(0.0, 100.0);
                            }
                        }
                        CurrentEditField::Rewatch => {
                            if let Some(digit) = c.to_digit(10) {
                                media.repeat = (media.repeat * 10 + (digit as i64)).clamp(0, 10000);
                            }
                        }
                        CurrentEditField::StartDate => app.edit_start_date_text.push(c),
                        CurrentEditField::EndDate => app.edit_end_date_text.push(c),
                        CurrentEditField::Notes => media.notes.push(c),
                        CurrentEditField::Status => {}
                    }
                }
            }
            _ => {}
        }
    } else {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.active_popup = None;
            }
            KeyCode::Char('s') => {
                fn parse_date(date_str: &str) -> crate::app_helper_structs::Date {
                    let parts: Vec<&str> = date_str.split('-').collect();
                    let year = parts.first().and_then(|s| s.parse().ok());
                    let month = parts.get(1).and_then(|s| s.parse().ok());
                    let day = parts.get(2).and_then(|s| s.parse().ok());
                    crate::app_helper_structs::Date { year, month, day }
                }

                if let Some(media) = &mut app.edited_media {
                    media.started_at = parse_date(&app.edit_start_date_text);
                    media.completed_at = parse_date(&app.edit_end_date_text);
                }

                app.save_edited_media(client, tx);
            }
            KeyCode::Enter | KeyCode::Char('i') => {
                app.is_in_edit_state = true;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
                app.edit_popup_index = (app.edit_popup_index + 1) % fields.len();
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::BackTab => {
                app.edit_popup_index = (app.edit_popup_index + fields.len() - 1) % fields.len();
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') => {
                if let Some(media) = &mut app.edited_media {
                    let step = if key.code == KeyCode::Right || key.code == KeyCode::Char('l') {
                        1
                    } else {
                        -1
                    };

                    match current_field {
                        CurrentEditField::Status => {
                            if step > 0 {
                                media.status = media.status.next();
                            } else {
                                media.status = media.status.previous();
                            }
                        }
                        CurrentEditField::EpisodeProgress => {
                            media.progress = (media.progress + step).clamp(
                                0,
                                app.media_details.as_ref().unwrap().total.unwrap_or(20000),
                            )
                        }
                        CurrentEditField::VolumeProgress => {
                            let mut vols = media.progress_volumes.unwrap_or(0);
                            vols = (vols + step).clamp(
                                0,
                                app.media_details.as_ref().unwrap().volumes.unwrap_or(20000),
                            );
                            media.progress_volumes = Some(vols);
                        }
                        CurrentEditField::Score => {
                            media.score = (media.score + step as f64).clamp(0.0, 100.0)
                        }
                        CurrentEditField::Rewatch => {
                            media.repeat = (media.repeat + step).clamp(0, 100000)
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn handle_filter_popup_events(
    app: &mut App,
    key: KeyEvent,
    client: AnilistClient,
    tx: Sender<AppAction>,
) {
    use crate::app_helper_structs::{
        CurrentView, MediaFormat, MediaListSort, MediaListStatus, MediaSeason, MediaSort,
        MediaStatus, cycle_option,
    };

    let is_user_view = matches!(
        app.current_view,
        CurrentView::UserAnime | CurrentView::UserManga
    );
    let max_fields = if is_user_view { 4 } else { 6 };

    if app.is_in_edit_state {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                app.is_in_edit_state = false;
            }
            KeyCode::Backspace => {
                if !is_user_view {
                    match app.filter_popup_index {
                        0 => {
                            app.filter_search_text.pop();
                        }
                        5 => {
                            app.filter_year_text.pop();
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Char(c) => {
                if !is_user_view {
                    match app.filter_popup_index {
                        0 => {
                            app.filter_search_text.push(c);
                        }
                        5 => {
                            if c.is_ascii_digit() {
                                app.filter_year_text.push(c);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    } else {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.active_popup = None;
            }
            KeyCode::Char('s') => {
                if is_user_view {
                    app.save_current_user_filter();
                    app.fetch_user_media(client, tx);
                } else {
                    app.save_current_filter();
                    app.fetch_browse(client, tx);
                }
                app.active_popup = None;
            }
            KeyCode::Char('r') => {
                if is_user_view {
                    app.reset_current_user_filter();
                    app.save_current_user_filter();
                    app.fetch_user_media(client, tx);
                } else {
                    app.reset_current_filter();
                    app.save_current_filter();
                    app.fetch_browse(client, tx);
                }
                app.active_popup = None;
            }
            KeyCode::Enter | KeyCode::Char('i') => {
                if !is_user_view && (app.filter_popup_index == 0 || app.filter_popup_index == 5) {
                    app.is_in_edit_state = true;
                }
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
                app.filter_popup_index = (app.filter_popup_index + 1) % max_fields;
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::BackTab => {
                app.filter_popup_index = (app.filter_popup_index + max_fields - 1) % max_fields;
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') => {
                let step = if key.code == KeyCode::Right || key.code == KeyCode::Char('l') {
                    1
                } else {
                    -1
                };
                let popup_index = app.filter_popup_index;

                if is_user_view {
                    let filter = app.get_mut_current_user_filter();

                    match popup_index {
                        0 => {
                            let current_sort = filter
                                .sort
                                .as_ref()
                                .and_then(|v| v.first().cloned())
                                .unwrap_or(MediaListSort::UpdatedTimeDesc);

                            let next_sort = if step > 0 {
                                current_sort.next()
                            } else {
                                current_sort.previous()
                            };
                            filter.sort = Some(vec![next_sort]);
                        }
                        1 => {
                            filter.format = cycle_option(&filter.format, &MediaFormat::ALL, step);
                        }
                        2 => {
                            filter.status =
                                cycle_option(&filter.status, &MediaListStatus::ALL, step);
                        }
                        3 => {
                            filter.favourites_only = !filter.favourites_only;
                        }
                        _ => {}
                    }
                } else {
                    let filter = app.get_mut_current_filter();

                    match popup_index {
                        1 => {
                            let current_sort = filter
                                .sort
                                .as_ref()
                                .and_then(|v| v.first().cloned())
                                .flatten()
                                .unwrap_or(MediaSort::PopularityDesc);

                            let next_sort = if step > 0 {
                                current_sort.next()
                            } else {
                                current_sort.previous()
                            };
                            filter.sort = Some(vec![Some(next_sort)]);
                        }
                        2 => {
                            filter.format = cycle_option(&filter.format, &MediaFormat::ALL, step);
                        }
                        3 => {
                            filter.season = cycle_option(&filter.season, &MediaSeason::ALL, step);
                        }
                        4 => {
                            filter.status = cycle_option(&filter.status, &MediaStatus::ALL, step);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
