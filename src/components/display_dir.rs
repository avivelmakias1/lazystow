use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  components::utils::get_top_bar_layout_chunk,
  config::{Config, KeyBindings},
};

#[derive(Default)]
pub struct DisplayDir {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  current_dir: String,
  insert_mode: bool,
  before_last_insert_dir: String,
}

impl DisplayDir {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Component for DisplayDir {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    if self.insert_mode {
      match key.code {
        KeyCode::Char(c) => self.current_dir.push(c),
        _ => {},
      }
    }
    Ok(None)
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {},
      Action::UpdateDir => {
        self.before_last_insert_dir = self.current_dir.clone();
        self.insert_mode = true;
      },
      Action::Confirm => {
        self.insert_mode = false;
        return Ok(Some(Action::Back));
      },
      Action::Discard => {
        self.current_dir = self.before_last_insert_dir.clone();
        self.insert_mode = false;
        return Ok(Some(Action::Back));
      },
      Action::Back => {
        self.insert_mode = false;
      },
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let block = Paragraph::new(self.current_dir.clone()).block(Block::default().title("Folder").borders(Borders::ALL));
    f.render_widget(block, get_top_bar_layout_chunk(area));
    Ok(())
  }
}
