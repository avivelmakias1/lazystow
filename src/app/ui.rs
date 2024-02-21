use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};
use tui_textarea::TextArea;

use super::App;

pub fn render(f: &mut Frame, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(size);
    let block = Block::default().title("Folder").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Content").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    if app.config_dir_popup {
        //let textarea = TextArea::new(vec![app.current_dir.to_string()]);
        let block = Block::default()
            .title("Edit Directory")
            .borders(Borders::ALL);
        let mut textarea = TextArea::default();
        textarea.insert_str(app.current_dir.to_string());
        textarea.set_block(block);
        let area = build_input_centered_rect(size);
        f.render_widget(textarea.widget(), area);
    }
}

fn build_input_centered_rect(r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(3),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
