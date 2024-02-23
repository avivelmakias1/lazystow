use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn get_top_bar_layout_chunk(area: Rect) -> Rect {
  let chunks = Layout::default()
    .direction(ratatui::layout::Direction::Vertical)
    .margin(1)
    .constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
    .split(area);
  chunks[0]
}
