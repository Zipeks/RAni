use crate::anilist::AnilistClient;
use std::error::Error;

use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::{EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::crossterm::{event, execute};
use std::{env, io};
use dotenv::dotenv;

mod anilist;
mod app;
mod ui;

// 1. Definiujemy strukturę i używamy makra GraphQLQuery.
// Makro to odczyta pliki schema.graphql oraz query.graphql i wygeneruje
// moduł o nazwie `union_query` (od nazwy zapytania "UnionQuery").
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // enable_raw_mode()?;
    // let mut stderr = io::stderr();
    // execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    //
    // let backend = CrosstermBackend::new(stderr);
    // let mut terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;

    // let mut app = App::new();
    // let res = run_app(&mut terminal, &mut app);

    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    dotenv().ok();
    let anilist_token = env::var("ANILIST_TOKEN").ok();

    let client = AnilistClient::new(anilist_token.as_deref())?;

    let anime_data = client.get_anime(15).await?;

    if let Some(media) = anime_data.media {
        println!("{:?}", media);
        if let Some(title) = media.title {
            println!("{:?}", title);
        }
    }
    let viewer_data = client.get_viewer().await?;

    if let Some(viewer) = viewer_data.viewer {
        println!("{:?}", viewer);
    }

    Ok(())

}
