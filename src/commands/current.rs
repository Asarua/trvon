use crate::helper::{get_current_registry, get_full_registries};

use super::Commander;
use clap::Parser;
use colored::Colorize;
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Current {
  #[arg(long, short = 'u', help = "Show the registry URL instead of the name")]
  show_url: bool,
}

impl Commander for Current {
  type Error = CurrentError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    let current_registry = get_current_registry().unwrap_or_default();
    if let Some(registry) = get_full_registries().unwrap().iter().find(|registry| {
      registry.lowercase_equal(&current_registry.name, None)
        || registry.lowercase_equal(&current_registry.registry, Some("registry".into()))
    }) {
      println!(
        "You are using {} registry.",
        if self.show_url {
          registry.registry.clone()
        } else {
          registry.name.clone()
        }
        .green()
      )
    } else {
      println!(
        "Your current registry({}) is not included in the trvon registries.",
        current_registry.registry.blue()
      );
      println!(
        "Use the {} command to add your registry.",
        "trvon add <registry> <url> [home]".green()
      )
    }
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum CurrentError {}
