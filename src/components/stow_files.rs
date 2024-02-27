use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{utils::get_main_layout_chunk, Component, Frame};
use crate::{
  action::Action,
  components::utils::get_top_bar_layout_chunk,
  config::{Config, KeyBindings},
  stow::{get_folders_from_path, StowFile},
};

struct DisplayStowFile<'a> {
  file: StowFile,
  display: ListItem<'a>,
}

impl DisplayStowFile<'_> {
  pub fn new(file: &StowFile) -> Self {
    Self { file: file.clone(), display: create_file_display(file.clone()) }
  }

  pub fn select(&mut self) {
    self.display = create_file_selected_display(self.file.clone());
  }

  pub fn deselect(&mut self) {
    self.display = create_file_display(self.file.clone());
  }
}

#[derive(Default)]
pub struct StowFiles<'a> {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  files: Vec<DisplayStowFile<'a>>,
  selected_index: usize,
}

impl StowFiles<'_> {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Component for StowFiles<'_> {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    match key.code {
      KeyCode::Down => {
        if self.selected_index < self.files.len() - 1 {
          self.files[self.selected_index].deselect();
          self.selected_index = self.selected_index + 1;
          self.files[self.selected_index].select();
        }
      },
      KeyCode::Up => {
        if self.selected_index > 0 {
          self.files[self.selected_index].deselect();
          self.selected_index = self.selected_index - 1;
          self.files[self.selected_index].select();
        }
      },
      _ => {},
    }
    Ok(None)
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {},
      Action::DirUpdated => {
        self.files =
          get_folders_from_path(String::from("path")).iter().map(|file| DisplayStowFile::new(file)).collect();
        self.files[0].select();
      },
      Action::Confirm => {
        return Ok(None);
      },
      Action::Discard => {
        return Ok(None);
      },
      Action::Back => {},
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let files_display = self.create_files_display().block(Block::default().title("Stow").borders(Borders::ALL));
    f.render_widget(files_display, get_main_layout_chunk(area));
    Ok(())
  }
}

impl StowFiles<'_> {
  fn create_files_display(&mut self) -> List {
    List::new(self.files.iter().map(|file| file.display.clone()).collect::<Vec<_>>())
  }
}

fn create_file_display<'a>(file: StowFile) -> ListItem<'a> {
  ListItem::from(file.name).style(Style::new().on_blue())
}

fn create_file_selected_display<'a>(file: StowFile) -> ListItem<'a> {
  ListItem::from(file.name).style(Style::new().on_red())
}
