use crate::ui::ui;
use crate::{auth, keybinds};
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self};
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::widgets::{ListState, TableState};
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub use crate::anilist::anilist_types::{MediaListSort, MediaListStatus, MediaType};
use crate::anilist::client::AnilistClient;
use crate::app_helper_structs::{
    ActiveBlock, ActivePopup, BrowseCategory, BrowseState, CurrentView, Date, MediaDetails,
    MediaListItem, SearchFilter, TitleLanguage, User, UserMediaDetails, UserMediaList,
};

pub struct App {
    pub active_block: ActiveBlock,
    pub current_view: CurrentView,
    pub user: Option<User>,
    pub sidebar_items: Vec<CurrentView>,
    pub sidebar_state: ListState,

    pub is_loading: bool,
    error_message: Option<String>,

    pub browse_state: BrowseState,
    pub image_picker: Picker,
    pub image_cache: HashMap<i64, StatefulProtocol>,
    pub currently_fetching_image: Option<i64>,

    pub media_details: Option<MediaDetails>,
    pub title_language: TitleLanguage,

    pub language_popup_index: usize,

    pub active_popup: Option<ActivePopup>,

    pub edited_media: Option<UserMediaDetails>,

    pub is_in_edit_state: bool,
    pub edit_popup_index: usize,
    pub edit_is_manga: bool,
    pub edit_start_date_text: String,
    pub edit_end_date_text: String,

    pub filter_popup_index: usize,
    pub filter_search_text: String,
    pub filter_year_text: String,

    pub latest_details_req_id: Arc<AtomicUsize>,
}

impl App {
    pub fn new() -> App {
        let mut state = ListState::default();
        state.select(Some(0));
        let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks());
        App {
            active_block: ActiveBlock::Sidebar,
            current_view: CurrentView::UserAnime,
            sidebar_items: CurrentView::ALL.to_vec(),
            sidebar_state: state,
            is_loading: false,
            user: None,

            error_message: None,

            browse_state: BrowseState {
                active_filters: HashMap::new(),
                loaded_view: CurrentView::UserAnime,
                media: None,
                state: TableState::default(),
                current_category: BrowseCategory::CategoryOne,
            },
            image_picker: picker,
            image_cache: HashMap::new(),
            currently_fetching_image: None,
            media_details: None,
            title_language: TitleLanguage::UserPreferred,

            active_popup: None,
            language_popup_index: 0,
            edit_popup_index: 0,

            is_in_edit_state: false,
            edited_media: None,
            edit_is_manga: false,
            edit_end_date_text: String::new(),
            edit_start_date_text: String::new(),

            filter_popup_index: 0,
            filter_search_text: String::new(),
            filter_year_text: String::new(),

            latest_details_req_id: Arc::new(AtomicUsize::new(0)),
        }
    }
    pub fn set_error(&mut self, error_message: String) {
        self.active_popup = Some(ActivePopup::Error);
        self.error_message = Some(error_message);
    }
    pub fn get_error(&self) -> Option<String> {
        self.error_message.clone()
    }
    pub fn unset_error(&mut self) {
        self.active_popup = None;
        self.error_message = None;
    }

    pub fn next_sidebar_item(&mut self) {
        let len = self.sidebar_items.len();
        if len == 0 {
            return;
        }
        let current = self.sidebar_state.selected().unwrap_or(0);
        self.sidebar_state.select(Some((current + 1) % len));
    }

    pub fn previous_sidebar_item(&mut self) {
        let len = self.sidebar_items.len();
        if len == 0 {
            return;
        }
        let current = self.sidebar_state.selected().unwrap_or(0);
        self.sidebar_state.select(Some((current + len - 1) % len));
    }

    pub fn authenticated(&mut self, id: i64, name: String, allows_nsfw: Option<bool>) {
        self.user = Some(User {
            id,
            name,
            allows_nsfw,
        })
    }
    pub fn get_current_center_items(&self) -> &[MediaListItem] {
        self.browse_state
            .media
            .as_ref()
            .and_then(|l| l.items.as_deref())
            .unwrap_or(&[])
    }

    pub fn next_center_item(&mut self) {
        let count = self.get_current_center_items().len();
        if count == 0 {
            return;
        }

        let current = self.browse_state.state.selected().unwrap_or(0);
        self.browse_state.state.select(Some((current + 1) % count));
    }

    pub fn previous_center_item(&mut self) {
        let count = self.get_current_center_items().len();
        if count == 0 {
            return;
        }

        let current = self.browse_state.state.selected().unwrap_or(0);
        self.browse_state
            .state
            .select(Some((current + count - 1) % count));
    }

    pub fn fetch_user_media(&mut self, client: AnilistClient, tx: Sender<AppAction>) {
        self.fetch_user_media_list(
            client,
            tx,
            match self.browse_state.current_category {
                BrowseCategory::CategoryOne => Some(MediaListStatus::Current),
                BrowseCategory::CategoryTwo => Some(MediaListStatus::Completed),
                BrowseCategory::CategoryThree => Some(MediaListStatus::Planning),
                _ => None,
            },
            match self.browse_state.current_category {
                BrowseCategory::CategoryTwo => Some(vec![Some(MediaListSort::ScoreDesc)]),
                _ => None,
            },
            Some(
                self.browse_state
                    .media
                    .as_ref()
                    .map_or(1, |m| m.page_info.current_page),
            ),
            Some(
                self.browse_state
                    .media
                    .as_ref()
                    .map_or(25, |m| m.page_info.per_page),
            ),
            match self.current_view {
                CurrentView::UserAnime => MediaType::Anime,
                CurrentView::UserManga => MediaType::Manga,
                _ => unimplemented!(),
            },
        );
    }

    pub fn fetch_user_media_list(
        &mut self,
        client: AnilistClient,
        tx: Sender<AppAction>,
        status: Option<MediaListStatus>,
        sort: Option<Vec<Option<MediaListSort>>>,
        page: Option<i64>,
        per_page: Option<i64>,
        type_: MediaType,
    ) {
        if self.is_loading {
            return;
        }
        let user_id = self.user.as_ref().map(|u| u.id).unwrap_or(0);
        self.is_loading = true;
        self.error_message = None;

        let client_clone = client.clone();
        let tx_clone = tx.clone();
        let status_for_sort = status;

        tokio::spawn(async move {
            let timeout_duration = Duration::from_secs(5);
            let fetch_future =
                client_clone.get_user_media_list(user_id, status, sort, page, per_page, type_);

            let timeout_result = tokio::time::timeout(timeout_duration, fetch_future).await;
            let action: AppAction = Box::new(move |app: &mut App| {
                app.is_loading = false;
                match timeout_result {
                    Ok(Ok(data)) => {
                        let mut clean_list = UserMediaList::from(data);
                        if let Some(MediaListStatus::Current) = status_for_sort
                            && let Some(ref mut items) = clean_list.items
                        {
                            items.sort_by(|a, b| {
                                match (&a.next_airing_episode, &b.next_airing_episode) {
                                    (Some(ep_a), Some(ep_b)) => {
                                        ep_a.time_until_airing.cmp(&ep_b.time_until_airing)
                                    }

                                    (Some(_), None) => std::cmp::Ordering::Less,

                                    (None, Some(_)) => std::cmp::Ordering::Greater,

                                    (None, None) => a
                                        .titles
                                        .get_title(&TitleLanguage::UserPreferred)
                                        .cmp(b.titles.get_title(&TitleLanguage::UserPreferred)),
                                }
                            });
                        };
                        let old_selected = app.browse_state.state.selected();
                        app.browse_state.media = Some(clean_list);

                        if let Some(idx) = old_selected {
                            let len = app.get_current_center_items().len();
                            if len > 0 && idx < len {
                                app.browse_state.state.select(Some(idx));
                            } else {
                                app.browse_state.state.select_first();
                            }
                        } else {
                            app.browse_state.state.select_first();
                        }
                    }
                    Ok(Err(api_error)) => {
                        app.set_error(format!("API error: {}", api_error));
                    }
                    Err(_) => {
                        app.set_error("Server timout".to_string());
                    }
                }
            });
            let _ = tx_clone.send(action);
        });
    }

    pub fn next_center_page(&mut self) {
        if let Some(media) = &mut self.browse_state.media
            && media.page_info.has_next_page.unwrap_or(false)
        {
            media.page_info.current_page += 1;
        }
    }

    pub fn previous_center_page(&mut self) {
        if let Some(media) = &mut self.browse_state.media
            && media.page_info.current_page > 1
        {
            media.page_info.current_page -= 1;
        }
    }

    pub fn open_anilist(&mut self) {
        if let Some(media_details) = &self.media_details {
            match open::that(&media_details.site_url) {
                Ok(()) => {}
                Err(err) => self.set_error(format!("Something went wrong {}", err)),
            };
        }
    }

    pub fn fetch_browse(&mut self, client: AnilistClient, tx: Sender<AppAction>) {
        let filter = self.get_current_filter();
        self.fetch_media(
            client,
            tx,
            Some(
                self.browse_state
                    .media
                    .as_ref()
                    .map_or(1, |m| m.page_info.current_page),
            ),
            Some(
                self.browse_state
                    .media
                    .as_ref()
                    .map_or(25, |m| m.page_info.per_page),
            ),
            {
                match self.current_view {
                    CurrentView::BrowseAnime => MediaType::Anime,
                    CurrentView::BrowseManga => MediaType::Manga,
                    _ => MediaType::Unknown,
                }
            },
            filter,
        );
    }

    pub fn fetch_media(
        &mut self,
        client: AnilistClient,
        tx: Sender<AppAction>,
        page: Option<i64>,
        per_page: Option<i64>,
        media_type: MediaType,
        search_filter: SearchFilter,
    ) {
        if self.is_loading {
            return;
        }
        self.is_loading = true;
        self.unset_error();

        let client_clone = client.clone();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let timeout_duration = Duration::from_secs(5);
            let fetch_future = client_clone.get_media(media_type, search_filter, page, per_page);
            let timeout_result = tokio::time::timeout(timeout_duration, fetch_future).await;

            let action: AppAction = Box::new(move |app: &mut App| {
                app.is_loading = false;

                match timeout_result {
                    Ok(Ok(data)) => {
                        let clean_list = UserMediaList::from(data);
                        let old_selected = app.browse_state.state.selected();

                        app.browse_state.media = Some(clean_list);

                        if let Some(idx) = old_selected {
                            let len = app.get_current_center_items().len();
                            if len > 0 && idx < len {
                                app.browse_state.state.select(Some(idx));
                            } else {
                                app.browse_state.state.select_first();
                            }
                        } else {
                            app.browse_state.state.select_first();
                        }
                    }
                    Ok(Err(api_error)) => {
                        app.set_error(format!("API error: {}", api_error));
                    }
                    Err(_) => {
                        app.set_error("Server timout".to_string());
                    }
                }
            });

            let _ = tx_clone.send(action);
        });
    }
    pub fn clean_media_details(&mut self) {
        self.media_details = None;
    }
    pub fn fetch_media_details(&mut self, client: AnilistClient, tx: Sender<AppAction>) {
        let selected_index = self.browse_state.state.selected();
        let current_items = self.get_current_center_items();

        let Some(idx) = selected_index else {
            return;
        };
        if idx >= current_items.len() {
            return;
        }

        let media_id = current_items[idx].id;

        self.clean_media_details();
        self.is_loading = true;
        self.unset_error();

        let client_clone = client.clone();
        let tx_clone = tx.clone();

        let req_id = self.latest_details_req_id.fetch_add(1, Ordering::SeqCst) + 1;
        let req_id_clone = self.latest_details_req_id.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(150)).await;

            if req_id_clone.load(Ordering::SeqCst) != req_id {
                return;
            }

            let timeout_duration = Duration::from_secs(5);
            let fetch_future = client_clone.get_media_details(media_id);
            let timeout_result = tokio::time::timeout(timeout_duration, fetch_future).await;

            let tx_for_action = tx_clone.clone();
            let action: AppAction = Box::new(move |app: &mut App| {
                app.is_loading = false;

                if app.latest_details_req_id.load(Ordering::SeqCst) != req_id {
                    return;
                }

                match timeout_result {
                    Ok(Ok(data)) => {
                        let media_details = MediaDetails::from(data);

                        let cover_url = &media_details.cover_image;
                        if !cover_url.is_empty() {
                            app.fetch_cover(media_id, cover_url.clone(), tx_for_action);
                        }

                        app.media_details = Some(media_details);
                    }
                    Ok(Err(api_error)) => {
                        app.set_error(format!("API error: {}", api_error));
                    }
                    Err(_) => {
                        app.set_error("Server timeout".to_string());
                    }
                }
            });
            let _ = tx_clone.send(action);
        });
    }

    pub fn fetch_cover(&mut self, media_id: i64, url: String, tx: Sender<AppAction>) {
        if self.image_cache.contains_key(&media_id)
            || self.currently_fetching_image == Some(media_id)
        {
            return;
        }

        self.currently_fetching_image = Some(media_id);
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            if let Ok(response) = reqwest::get(&url).await
                && let Ok(bytes) = response.bytes().await
                && let Ok(dyn_image) = image::load_from_memory(&bytes)
            {
                let action: AppAction = Box::new(move |app: &mut App| {
                    let protocol = app.image_picker.new_resize_protocol(dyn_image);

                    app.image_cache.insert(media_id, protocol);
                    app.currently_fetching_image = None;
                });

                let _ = tx_clone.send(action);
                return;
            }

            let action: AppAction = Box::new(move |app: &mut App| {
                app.currently_fetching_image = None;
            });
            let _ = tx_clone.send(action);
        });
    }

    pub fn get_current_edit_fields(&self) -> Vec<crate::app_helper_structs::CurrentEditField> {
        use crate::app_helper_structs::CurrentEditField;
        let mut fields = vec![CurrentEditField::Status, CurrentEditField::EpisodeProgress];

        if self.edit_is_manga {
            fields.push(CurrentEditField::VolumeProgress);
        }

        fields.extend(vec![
            CurrentEditField::Score,
            CurrentEditField::Rewatch,
            CurrentEditField::StartDate,
            CurrentEditField::EndDate,
            CurrentEditField::Notes,
        ]);
        fields
    }

    pub fn open_edit_popup(&mut self) {
        let selected_item = if let Some(selected_index) = self.browse_state.state.selected() {
            let current_items = self.get_current_center_items();
            if selected_index < current_items.len() {
                Some(current_items[selected_index].clone())
            } else {
                None
            }
        } else {
            None
        };

        let Some(item) = selected_item else {
            return;
        };

        self.edit_is_manga = matches!(item.type_, MediaType::Manga);

        if let Some(details) = &self.media_details
            && let Some(user_details) = &details.user_media_details
        {
            self.edited_media = Some(user_details.clone());
            self.active_popup = Some(ActivePopup::EditMedia);
            self.edit_popup_index = 0;

            let start = user_details.started_at.to_string();
            self.edit_start_date_text = if start == "Unknown" {
                String::new()
            } else {
                start
            };

            let end = user_details.completed_at.to_string();
            self.edit_end_date_text = if end == "Unknown" { String::new() } else { end };
            return;
        }

        let user_media_id = None;
        let progress = item.progress.unwrap_or(0);
        let status = item.status.unwrap_or(MediaListStatus::Current);
        let media_id = item.id;

        self.edited_media = Some(UserMediaDetails {
            user_media_id,
            media_id,
            progress,
            progress_volumes: None,
            repeat: 0,
            started_at: Date::empty(),
            completed_at: Date::empty(),
            score: 0.0,
            status,
            notes: String::new(),
        });

        self.edit_start_date_text = String::new();
        self.edit_end_date_text = String::new();
        self.edit_popup_index = 0;
        self.active_popup = Some(ActivePopup::EditMedia);
    }
    pub fn save_edited_media(&mut self, client: AnilistClient, tx: Sender<AppAction>) {
        let Some(mut edited_media) = self.edited_media.clone() else {
            return;
        };
        if !self.edit_is_manga {
            edited_media.progress_volumes = None;
        }

        let media_id = edited_media.media_id;

        self.is_loading = true;
        let client_clone = client.clone();
        let tx_clone = tx.clone();

        let tx_for_action = tx_clone.clone();
        let client_for_action = client_clone.clone();

        tokio::spawn(async move {
            let res = client_clone.update_entry(&edited_media).await;

            if res.is_ok() {
                client_clone.clear_media_list_cache();
                client_clone.delete_from_details_cache(media_id).await;
            }

            let action: AppAction = Box::new(move |app: &mut App| {
                app.is_loading = false;
                match res {
                    Ok(_) => {
                        app.active_popup = None;
                        app.edited_media = None;

                        app.fetch_media_details(client_for_action.clone(), tx_for_action.clone());

                        app.is_loading = false;

                        match app.current_view {
                            CurrentView::UserAnime | CurrentView::UserManga => {
                                app.fetch_user_media(client_for_action, tx_for_action);
                            }
                            CurrentView::BrowseAnime | CurrentView::BrowseManga => {
                                app.fetch_browse(client_for_action, tx_for_action);
                            }
                        }
                    }
                    Err(e) => {
                        app.set_error(format!("Something went wrong: {}", e));
                    }
                }
            });
            let _ = tx_clone.send(action);
        });
    }
    pub fn fetch_toggle_favourite(&mut self, client: AnilistClient, tx: Sender<AppAction>) {
        if self.is_loading {
            return;
        }
        let Some(media_details) = &self.media_details else {
            return;
        };

        let id = media_details.media_id;
        let media_type = media_details.type_;

        let next_favourite_state = !media_details.is_favourite;

        self.is_loading = true;
        self.unset_error();

        tokio::spawn(async move {
            let timeout_duration = Duration::from_secs(5);

            let (anime_id, manga_id) = {
                match media_type {
                    MediaType::Anime => (Some(id), None),
                    MediaType::Manga => (None, Some(id)),
                    _ => (None, None),
                }
            };

            let fetch_future = client.toggle_favourite(anime_id, manga_id);
            let timeout_result = tokio::time::timeout(timeout_duration, fetch_future).await;

            let is_success = matches!(timeout_result, Ok(Ok(_)));

            if is_success {
                client
                    .update_details_cache_favourite(id, next_favourite_state)
                    .await;
            }

            let action: AppAction = Box::new(move |app: &mut App| {
                app.is_loading = false;
                match timeout_result {
                    Ok(Ok(_data)) => {
                        if let Some(ref mut details) = app.media_details {
                            details.is_favourite = !details.is_favourite;
                        }
                    }
                    Ok(Err(api_error)) => {
                        app.set_error(format!("API error: {}", api_error));
                    }
                    Err(_) => {
                        app.set_error("Server timeout".to_string());
                    }
                }
            });
            let _ = tx.send(action);
        });
    }

    pub fn fetch_delete_media(&mut self, client: AnilistClient, tx: Sender<AppAction>) {
        if self.is_loading {
            return;
        }

        let Some(media_details) = &self.media_details else {
            return;
        };
        let id = media_details.media_id;

        let Some(user_media_details) = &media_details.user_media_details else {
            return;
        };
        let Some(user_media_id) = user_media_details.user_media_id else {
            return;
        };

        self.is_loading = true;
        self.unset_error();

        let client_clone = client.clone();
        let tx_clone = tx.clone();
        let client_for_action = client.clone();
        let tx_for_action = tx.clone();

        tokio::spawn(async move {
            let timeout_duration = Duration::from_secs(5);

            let fetch_future = client_clone.delete_media(user_media_id);
            let timeout_result = tokio::time::timeout(timeout_duration, fetch_future).await;

            let is_success = matches!(timeout_result, Ok(Ok(_)));

            if is_success {
                client_clone.delete_from_details_cache(id).await;
                client_clone.clear_media_list_cache();

                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }

            let action: AppAction = Box::new(move |app: &mut App| {
                app.is_loading = false;
                match timeout_result {
                    Ok(Ok(_data)) => {
                        app.clean_media_details();

                        match app.current_view {
                            CurrentView::UserAnime | CurrentView::UserManga => {
                                app.fetch_user_media(client_for_action, tx_for_action);
                            }
                            CurrentView::BrowseAnime | CurrentView::BrowseManga => {
                                app.fetch_browse(client_for_action, tx_for_action);
                            }
                        }
                    }
                    Ok(Err(api_error)) => {
                        app.set_error(format!("API error: {}", api_error));
                    }
                    Err(_) => {
                        app.set_error("Server timeout".to_string());
                    }
                }
            });
            let _ = tx_clone.send(action);
        });
    }
    pub fn get_current_filter(&mut self) -> SearchFilter {
        let key = (self.current_view, self.browse_state.current_category);
        self.browse_state
            .active_filters
            .entry(key)
            .or_insert_with(|| {
                SearchFilter::default_for(self.browse_state.current_category, self.current_view)
            })
            .clone()
    }

    pub fn get_mut_current_filter(&mut self) -> &mut SearchFilter {
        let key = (self.current_view, self.browse_state.current_category);
        self.browse_state
            .active_filters
            .entry(key)
            .or_insert_with(|| {
                SearchFilter::default_for(self.browse_state.current_category, self.current_view)
            })
    }

    pub fn reset_current_filter(&mut self) {
        let key = (self.current_view, self.browse_state.current_category);
        let default_filter =
            SearchFilter::default_for(self.browse_state.current_category, self.current_view);
        self.browse_state.active_filters.insert(key, default_filter);
    }

    pub fn open_filter_popup(&mut self) {
        let filter = self.get_current_filter();
        self.filter_search_text = filter.search.unwrap_or_default();
        self.filter_year_text = filter.year.map(|y| y.to_string()).unwrap_or_default();
        self.filter_popup_index = 0;
        self.active_popup = Some(ActivePopup::SearchFilter);
    }

    pub fn save_current_filter(&mut self) {
        let query = if self.filter_search_text.trim().is_empty() {
            None
        } else {
            Some(self.filter_search_text.clone())
        };
        let year = self.filter_year_text.trim().parse::<i64>().ok();

        let filter = self.get_mut_current_filter();
        filter.search = query;
        filter.year = year;

        self.active_popup = None;
    }
}

pub type AppAction = Box<dyn FnOnce(&mut App) + Send>;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    client: AnilistClient,
    tx: Sender<AppAction>,
    rx: &Receiver<AppAction>,
) -> io::Result<bool>
where
    std::io::Error: From<<B as Backend>::Error>,
{
    spawn_initial_viewer_fetch(client.clone(), tx.clone());

    loop {
        terminal.draw(|f| ui(f, app))?;

        while let Ok(action) = rx.try_recv() {
            action(app);
        }

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            if let Some(active_popup) = &app.active_popup {
                match active_popup {
                    ActivePopup::TitleLanguage => keybinds::handle_language_popup_events(app, key),
                    ActivePopup::Error => keybinds::handle_error_popup_events(app, key),
                    ActivePopup::EditMedia => keybinds::handle_edit_media_popup_events(
                        app,
                        key,
                        client.clone(),
                        tx.clone(),
                    ),
                    ActivePopup::Favourite => keybinds::handle_favourite_popup_events(
                        app,
                        key,
                        client.clone(),
                        tx.clone(),
                    ),
                    ActivePopup::DeleteMedia => keybinds::handle_delete_media_popup_events(
                        app,
                        key,
                        client.clone(),
                        tx.clone(),
                    ),
                    ActivePopup::SearchFilter => {
                        keybinds::handle_filter_popup_events(app, key, client.clone(), tx.clone())
                    }
                }
                continue;
            }

            if key.code == KeyCode::Char('q') {
                return Ok(true);
            }

            match app.active_block {
                ActiveBlock::Sidebar => {
                    keybinds::handle_sidebar_events(app, key, client.clone(), tx.clone())
                }
                ActiveBlock::Center => {
                    keybinds::handle_center_events(app, key, client.clone(), tx.clone())
                }
                ActiveBlock::Details => {
                    keybinds::handle_details_events(app, key, client.clone(), tx.clone())
                }
            }
        }
    }
}

fn spawn_initial_viewer_fetch(client: AnilistClient, tx: Sender<AppAction>) {
    let client_clone = client.clone();
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        let timeout_duration = Duration::from_secs(2);
        let fetch_future = client_clone.get_basic_viewer();

        let timeout_result = tokio::time::timeout(timeout_duration, fetch_future).await;
        let action: AppAction = Box::new(move |app: &mut App| match timeout_result {
            Ok(Ok(data)) => {
                if let Some(viewer) = data.viewer {
                    let allows_nsfw = viewer.options.and_then(|o| o.display_adult_content);

                    app.authenticated(viewer.id, viewer.name, allows_nsfw);
                }
            }
            Ok(Err(data)) => {
                app.set_error(format!("Error connecting to Anilist: {}", data));
                _ = auth::clear_user_token();
            }
            Err(data) => app.set_error(format!("Error connecting to Anilist: {}", data)),
        });

        let _ = tx_clone.send(action);
    });
}
