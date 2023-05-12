use clap::Parser;
use thiserror::Error;

use super::Commander;

#[derive(Debug, Parser)]
pub struct SetScope {
  scope_name: String,
  url: String,
}

impl Commander for SetScope {
  type Error = SetScopeError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    println!("{self:#?}");
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum SetScopeError {}
