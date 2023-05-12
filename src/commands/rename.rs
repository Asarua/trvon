use clap::Parser;
use thiserror::Error;

use super::Commander;

#[derive(Debug, Parser)]
pub struct Rename {
  name: String,
  new_name: String,
}

impl Commander for Rename {
  type Error = RenameError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    println!("{self:#?}");
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum RenameError {}
