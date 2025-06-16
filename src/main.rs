mod ui;
mod core;

use std::sync::Arc;
use std::io::{self, Write};
use anyhow::Result;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;
use tokio::sync::mpsc::{self, Sender, Receiver};
use ratatui::crossterm::event::{self, Event, DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{ 
    enable_raw_mode,
    disable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen
};
use ratatui::crossterm::execute;
use ratatui::prelude::{Terminal, Backend, CrosstermBackend};

use crate::core::{Explorer, AppEvent};

async fn handle_input_loop(tx: Sender<AppEvent>, token: Arc<CancellationToken>) -> Result<()> {
    loop {
        if token.is_cancelled() { 
            tx.send(AppEvent::Terminate).await?;
            break Ok(());
        }
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    tx.send(AppEvent::Input(key)).await?;
                }
                other => eprintln!("Received non-key event: {:?}", other),
            }
        }
    }
}

async fn run<B: Backend>(
    root_terminal: &mut Terminal<B>, 
    explorer: &mut Explorer, 
    rx: &mut Receiver<AppEvent>,
    token: Arc<CancellationToken>
) -> Result<()> {
    while !explorer.exit {
        root_terminal.draw(|root_frame| ui::render(root_frame, explorer))?;
        match rx.recv().await {
            Some(app_event) => explorer.handle_top_level_event(app_event, &token),
            None => explorer.finish()
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = mpsc::channel(100);
    let mut explorer: Explorer = Explorer::new()?;

    let tok = CancellationToken::new();
    let app_tok = Arc::new(tok);

    let input_loop_task = tokio::spawn(handle_input_loop(tx, Arc::clone(&app_tok)));

    let run_result = run(&mut terminal, &mut explorer, &mut rx, Arc::clone(&app_tok)).await;

    app_tok.cancel();
    _ = tokio::join!(input_loop_task);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, EnableMouseCapture)?;
    io::stdout().flush()?;
    terminal.show_cursor()?;

    run_result
}