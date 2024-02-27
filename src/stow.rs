use core::fmt;
use std::io;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct StowFile {
  id: usize,
  pub name: String,
  pub full_path: String,
}

#[derive(Error, Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "failed to parse folder")
  }
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("error reading folder {0}")]
  ReadFolderError(#[from] io::Error),

  #[error("error parsing folder {0}")]
  ParseFolderError(#[from] ParseError),
}

pub fn get_folders_from_path(path: String) -> Vec<StowFile> {
  vec![
    StowFile { id: 1, name: String::from("nvim"), full_path: String::from("/Users/aviv/dotfiles/nvim") },
    StowFile { id: 2, name: String::from("tmux"), full_path: String::from("/Users/aviv/dotfiles/tmux") },
    StowFile { id: 3, name: String::from("alacritty"), full_path: String::from("/Users/aviv/dotfiles/alacritty") },
  ]
}
