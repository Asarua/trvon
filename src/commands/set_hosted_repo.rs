use clap::Parser;
use thiserror::Error;

use super::Commander;

#[derive(Debug, Parser)]
pub struct SetHostedRepo {
  name: String,
  repo: String,
}

impl Commander for SetHostedRepo {
  type Error = SetHostedRepoError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    println!("{self:#?}");
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum SetHostedRepoError {}
