use std::{rc::Rc, slice::RChunks};

use ratatui::layout::{Constraint, Direction, Layout, Rect};

fn default_layout(area: Rect) -> Rc<[Rect]> {
  Layout::default()
    .direction(ratatui::layout::Direction::Vertical)
    .margin(1)
    .constraints([Constraint::Length(3), Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
    .split(area)
}

pub fn get_top_bar_layout_chunk(area: Rect) -> Rect {
  default_layout(area)[0]
}

pub fn get_main_layout_chunk(area: Rect) -> Rect {
  default_layout(area)[1]
}
