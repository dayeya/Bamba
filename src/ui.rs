use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::Style;
use crate::core::{Explorer, Mode};

pub fn render(frame: &mut Frame, bamba: &Explorer) {
    let mode_text = match bamba.mode {
        Mode::Browse => "MODE: Browse (Press 'i' to Insert, 'q' to Quit)",
        Mode::Insert => "MODE: Insert (Press 'Esc' to Browse)",
    };

    let block = Block::default()
        .title(format!("Current Directory: {} ", bamba.cwd.display()))
        .borders(Borders::ALL)
        .border_style(Style::new());

    let paragraph = Paragraph::new(mode_text)
        .block(block)
        .style(Style::new());

    frame.render_widget(paragraph, frame.area());
}
