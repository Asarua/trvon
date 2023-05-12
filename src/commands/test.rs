use clap::Parser;
use thiserror::Error;

use super::Commander;

#[derive(Debug, Parser)]
pub struct Test {
  registry: Option<String>,
}

impl Commander for Test {
  type Error = TestError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    println!("{self:#?}");
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum TestError {}
