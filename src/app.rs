use crate::anilist::get_anime::MediaSeason;
use crate::app::MediaType::Anime;
use crate::app::Season::ANY;
use chrono::{Datelike, Utc};

use crate::ui::ui;
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::{EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::{event, execute};
use std::io;
pub enum ActiveBlock {
    Menu,
    List,
    Details,
}
pub enum CurrentView {
    Home,
    Profile,
}

impl CurrentView {
    pub fn to_string(&self) -> String {
        match &self {
            CurrentView::Home => String::from("Home"),
            CurrentView::Profile => String::from("Profile"),
        }
    }
}
pub struct User {
    name: String,
    allows_nsfw: Option<bool>,
}

impl User {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct App {
    pub active_block: ActiveBlock,
    pub current_view: CurrentView,
    pub search_settings: SearchSettings,
    pub previous_state: Box<Option<App>>,
    pub user: Option<User>,
    pub status: Option<String>,
}

impl App {
    pub fn new() -> App {
        App {
            active_block: ActiveBlock::Menu,
            current_view: CurrentView::Home,
            search_settings: SearchSettings {
                search_input: String::from(""),
                media_year: Utc::now().year(),
                media_season: ANY,
                media_type: Anime,
            },
            previous_state: Box::new(None),
            user: None,
            status: None,
        }
    }
    pub fn authenticated(&mut self, name: String, allows_nsfw: Option<bool>) {
        self.user = Some(User { name, allows_nsfw })
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

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.active_block {
                ActiveBlock::Menu => match key.code {
                    KeyCode::Char('l') | KeyCode::Enter => app.active_block = ActiveBlock::List,
                    _ => {}
                },
                ActiveBlock::List => match key.code {
                    KeyCode::Char('h') | KeyCode::BackTab => app.active_block = ActiveBlock::Menu,
                    _ => {}
                },
                _ => {}
            }
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                _ => {}
            }
        }
    }
}
