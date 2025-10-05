use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use std::{io, time::Duration};
use dotenv::dotenv;
use std::env;

mod app;
mod ui;
mod chain;
mod constants;

use app::App;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let private_keys = env::var("PRIVATE_KEYS").expect("PRIVATE_KEYS harus di set di .env");
    let private_keys_vec: Vec<String> = private_keys.split(',').map(String::from).collect();

    let mut app = App::new(private_keys_vec);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char(c) = key.code {
                    app.on_key(c);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
