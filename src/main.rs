use crate::anilist::client::AnilistClient;
use crate::app::{App, run_app};
use crate::auth::clear_user_token;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use std::error::Error;
use std::io;
use std::ops::Deref;
use tracing::error;
use tracing_subscriber::EnvFilter;

mod anilist;
mod app;
mod app_helper_structs;
mod auth;
mod keybinds;
mod ui;
mod utils;

const CLIENT_ID: &str = "40678";
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _guard = init_tracing();

    let token = auth::load_user_token();

    let anilist_token = match token {
        Some(t) => t,
        None => {
            println!("Authorization token not found.");
            println!("Logging in with your browser...");

            let new_token = auth::login_with_browser(CLIENT_ID).await;

            match new_token {
                Ok(s) => {
                    let _ = auth::save_user_token(&s);
                    s
                }
                Err(e) => {
                    error!("Something went wrong during authorization: {}", e);
                    println!("Something went wrong during authorization: {}", e);
                    return Ok(());
                }
            }
        }
    };
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;

    let mut app = App::new();

    let client = AnilistClient::new(Some(anilist_token.deref()))?;

    let (tx, rx) = std::sync::mpsc::channel();

    let _res = run_app(&mut terminal, &mut app, client, tx, &rx);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let log_dir = directories::ProjectDirs::from("", "", "anilist-tui")
        .map(|dirs| {
            if let Some(state) = dirs.state_dir() {
                state.join("logs")
            } else {
                dirs.data_local_dir().join("logs")
            }
        })
        .unwrap_or_else(|| std::path::PathBuf::from("logs"));

    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("rani.log")
        .max_log_files(30)
        .build(&log_dir)
        .expect("Loging system creation failed.");

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    guard
}
