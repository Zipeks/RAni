use crate::anilist::get_anime::{self, MediaSeason};
use crate::anilist::get_current_media::{self, MediaListStatus};
use crate::app::MediaType::Anime;
use crate::app::Season::ANY;
use crate::ui::ui;
use chrono::{Datelike, Utc};
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::widgets::ListState;
use std::io;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
#[derive(PartialEq)]
pub enum ActiveBlock {
    Sidebar,
    Center,
    Details,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentView {
    Home,
    Browse,
    Profile,
}

impl CurrentView {
    pub const ALL: [CurrentView; 3] =
        [CurrentView::Home, CurrentView::Browse, CurrentView::Profile];

    pub fn to_string(&self) -> &'static str {
        match &self {
            CurrentView::Home => "Home",
            CurrentView::Browse => "Browse",
            CurrentView::Profile => "Profile",
        }
    }
}

#[derive(Clone)]
pub struct User {
    id: i64,
    name: String,
    allows_nsfw: Option<bool>,
}

impl User {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct App {
    pub active_block: ActiveBlock,
    pub current_view: CurrentView,
    pub search_settings: SearchSettings,
    pub user: Option<User>,
    pub sidebar_items: Vec<CurrentView>,
    pub sidebar_state: ListState,
    pub is_loading: bool,
    pub error_message: Option<String>,

    pub current_anime: Option<get_anime::ResponseData>,
    pub current_media: Option<get_current_media::ResponseData>,
}

impl App {
    pub fn new() -> App {
        let mut state = ListState::default();
        state.select(Some(0));

        App {
            active_block: ActiveBlock::Sidebar,
            current_view: CurrentView::Home,
            search_settings: SearchSettings {
                search_input: String::from(""),
                media_year: Utc::now().year(),
                media_season: ANY,
                media_type: Anime,
            },
            sidebar_items: CurrentView::ALL.to_vec(),
            sidebar_state: state,
            is_loading: false,
            user: None,

            error_message: None,
            current_anime: None,
            current_media: None,
        }
    }
    pub fn next_sidebar_item(&mut self) {
        let i = match self.sidebar_state.selected() {
            Some(i) => {
                if i >= self.sidebar_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.sidebar_state.select(Some(i));
    }

    pub fn previous_sidebar_item(&mut self) {
        let i = match self.sidebar_state.selected() {
            Some(i) => {
                if i <= 0 {
                    self.sidebar_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.sidebar_state.select(Some(i));
    }

    pub fn authenticated(&mut self, id: i64, name: String, allows_nsfw: Option<bool>) {
        self.user = Some(User {
            id,
            name,
            allows_nsfw,
        })
    }

    pub fn get_current_view(&self) -> &CurrentView {
        &self.current_view
    }
}
pub enum MediaType {
    Anime,
    Manga,
}
pub enum Season {
    WINTER,
    SPRING,
    SUMMER,
    FALL,
    ANY,
}
pub struct SearchSettings {
    search_input: String,
    media_type: MediaType,
    media_year: i32,
    media_season: Season,
}

pub type AppAction = Box<dyn FnOnce(&mut App) + Send>;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    client: crate::anilist::AnilistClient,
    tx: Sender<AppAction>,
    rx: &Receiver<AppAction>,
) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    {
        let client_clone = client.clone();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let result = client_clone.get_basic_viewer().await;

            let action: AppAction = Box::new(move |app: &mut App| match result {
                Ok(data) => {
                    if let Some(viewer) = data.viewer {
                        if let Some(options) = viewer.options {
                            app.authenticated(
                                viewer.id,
                                viewer.name,
                                options.display_adult_content,
                            );
                        }
                    }
                }
                Err(_) => {}
            });
            let _ = tx_clone.send(action);
        });
    }

    loop {
        terminal.draw(|f| ui(f, app))?;

        while let Ok(action) = rx.try_recv() {
            action(app);
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match app.active_block {
                    ActiveBlock::Sidebar => match key.code {
                        KeyCode::Char('l') | KeyCode::Enter => {
                            if let Some(selected_idx) = app.sidebar_state.selected() {
                                app.current_view = app.sidebar_items[selected_idx];
                            }

                            app.active_block = ActiveBlock::Center;

                            match app.current_view {
                                CurrentView::Home => {
                                    if app.current_media.is_none() && !app.is_loading {
                                        let user_id = app.user.as_ref().map(|u| u.id).unwrap_or(0);

                                        app.is_loading = true;
                                        app.error_message = None;

                                        let client_clone = client.clone();
                                        let tx_clone = tx.clone();

                                        tokio::spawn(async move {
                                            let result =
                                                client_clone.get_current_media(user_id).await;

                                            let action: AppAction =
                                                Box::new(move |app: &mut App| {
                                                    app.is_loading = false;
                                                    match result {
                                                        Ok(data) => app.current_media = Some(data),
                                                        Err(e) => {
                                                            app.error_message = Some(e.to_string())
                                                        }
                                                    }
                                                });

                                            let _ = tx_clone.send(action);
                                        });
                                    }
                                }
                                CurrentView::Browse => {
                                    // app.is_loading = true;
                                    // app.error_message = None;

                                    // let client_clone = client.clone();
                                    // let tx_clone = tx.clone();

                                    // tokio::spawn(async move {
                                    //     let result = client_clone.get_anime(1).await;

                                    //     let action: AppAction = Box::new(move |app: &mut App| {
                                    //         app.is_loading = false;
                                    //         match result {
                                    //             Ok(data) => app.current_anime = Some(data),
                                    //             Err(e) => app.error_message = Some(e.to_string()),
                                    //         }
                                    //     });

                                    //     let _ = tx_clone.send(action);
                                    // });
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Char('j') | KeyCode::Down => app.next_sidebar_item(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous_sidebar_item(),
                        _ => {}
                    },

                    ActiveBlock::Center => match key.code {
                        KeyCode::Char('h') | KeyCode::BackTab | KeyCode::Esc => {
                            app.active_block = ActiveBlock::Sidebar
                        }
                        _ => {}
                    },

                    _ => {}
                }

                if let KeyCode::Char('q') = key.code {
                    return Ok(true);
                }
            }
        }
    }
}
