use core::fmt;
use std::io;
use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    id: usize,
    name: String,
    full_path: String,
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
