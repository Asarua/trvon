use clap::Parser;
use thiserror::Error;

use super::Commander;

#[derive(Debug, Parser)]
pub struct DelScope {
  scope_name: String,
}

impl Commander for DelScope {
  type Error = DelScopeError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    println!("{self:#?}");
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum DelScopeError {}
