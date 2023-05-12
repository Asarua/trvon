use crate::{
  constants::RNMRC_PATH,
  helper::{
    get_custom_registries_from_config, get_full_registries, print_success, write_custom_registries,
  },
  registry::Registry,
};
use clap::Parser;
use colored::Colorize;
use serde::Serialize;
use thiserror::Error;

use super::Commander;

#[derive(Debug, Parser, Serialize, Clone)]
pub struct Add {
  pub name: String,
  pub registry: String,
  pub home: Option<String>,
}

impl Commander for Add {
  type Error = AddError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    let full_registries = get_full_registries().unwrap();
    if full_registries.iter().any(|registry| {
      registry.lowercase_equal(&self.name, None)
        || registry.lowercase_equal(&self.registry, Some("registry".into()))
    }) {
      return Err(AddError::ExistRegistry);
    } else {
      let mut custom_registries = get_custom_registries_from_config().unwrap_or_default();
      custom_registries.push(Registry::from(self.clone()));
      if let Err(_) = write_custom_registries(&custom_registries, Some(RNMRC_PATH.as_path())) {
        return Err(AddError::WriteCustomRegistriesError);
      } else {
        print_success(format!(
          "Add registry {} success, run {} command to use {} registry.",
          self.name,
          format!("{}", format!("rnm use {}", self.name).green()),
          self.name
        ))
      }
    }

    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum AddError {
  #[error("The registry name or url is already included in the nrm registries. Please make sure that the name and url are unique.")]
  ExistRegistry,
  #[error("The custom registries file write fail.")]
  WriteCustomRegistriesError,
}
