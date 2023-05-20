use super::Commander;
use crate::helper::{get_current_registry, get_full_registries};
use anyhow::Result;
use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Ls;

impl Commander for Ls {
  type Error = LsError;

  fn apply(self) -> Result<(), Self::Error> {
    let registries = get_full_registries().unwrap();
    let max_length = registries
      .iter()
      .map(|v| v.name.chars().count())
      .max()
      .unwrap_or(0)
      + 4;
    let current_registry = get_current_registry().unwrap();

    println!("");
    registries.iter().for_each(|registry| {
      println!(
        "{} {} {} {}",
        if current_registry.lowercase_equal(&registry.name, None) {
          "*"
        } else {
          " "
        },
        registry.name,
        "-".repeat(max_length - registry.name.chars().count()),
        registry.registry
      )
    });
    println!("");
    Ok(())
  }
}

#[derive(Error, Debug)]
pub enum LsError {}
