use std::env;
use std::sync::Arc;
use std::path::PathBuf;
use anyhow::Result;
use tokio_util::sync::CancellationToken;
use ratatui::crossterm::event::{KeyEvent, KeyCode};

#[derive(Debug)]
pub enum AppEvent {
    Input(KeyEvent),
    Terminate
}

pub enum Mode {
    Browse,
    Insert,
}

pub struct Explorer {
    // Termination flag.
    pub exit: bool,
    // A current mode of the application.
    pub mode: Mode,
    // The current path being explored.
    pub cwd: PathBuf
}

impl Explorer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            exit: false,
            mode: Mode::Browse,
            cwd: env::current_dir()?
        })
    }

    // Prepares the explorer for a 'quitting' event.
    pub fn finish(&mut self) {
        self.exit = true;
    }

    pub fn handle_char(&mut self, ch: char, token: &Arc<CancellationToken>) {
        match ch {
            'i' => {
                self.mode = Mode::Insert;
            }
            'q' => {
                self.finish();
                token.cancel();
            }
            _ => {}
        }
    }

    pub fn handle_top_level_event(&mut self, app_event: AppEvent, token: &Arc<CancellationToken>) {
        match app_event {
            AppEvent::Input(key_event) => {
                match key_event.code {
                    KeyCode::Esc => self.mode = Mode::Browse,
                    KeyCode::Char(c) => self.handle_char(c, token),
                    _ => {}
                }
            },
            unsupported => eprintln!("error: caught unsupported app event: {:?}", unsupported)
        }
    }
}