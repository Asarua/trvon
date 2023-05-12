use clap::{Args, Parser};
use thiserror::Error;

use super::Commander;

#[derive(Args, Debug)]
pub struct SetArgs {
  #[arg(long, short, help = "Set a custom registry attribute")]
  attr: Option<String>,
  #[arg(long, short, help = "Set a custom registry value")]
  value: Option<String>,
}

#[derive(Debug, Parser)]
pub struct Set {
  name: String,
  #[command(flatten)]
  args: SetArgs,
}

impl Commander for Set {
  type Error = SetError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    println!("{self:#?}");
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum SetError {}
