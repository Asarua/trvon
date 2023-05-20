use std::collections::BTreeMap;

use crate::{
  constants::{NPMRC_PATH, TRVONRC_PATH},
  helper::{
    get_current_registry, get_custom_registries_from_config, is_internal_registry,
    is_registry_not_found, print_success, write_custom_registries,
  },
};

use super::Commander;
use clap::{Args, Parser};
use colored::Colorize;
use thiserror::Error;

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
    let Self { name, args } = self;

    if is_internal_registry(&name) {
      return Err(SetError::InternalRegistry);
    }

    if is_registry_not_found(&name) {
      return Err(SetError::RegistryNotFound(name));
    }

    let SetArgs { attr, value } = args;

    if attr.clone().unwrap_or_default().as_str() == "repository" {
      println!(
        "{}",
        format!(
          "Use the {} command to set repository.",
          "trvon set-hosted-repo <name> <repo>".green()
        )
      );
      return Ok(());
    }

    let mut custom_registries = get_custom_registries_from_config().unwrap_or_default();
    if let Some(registry) = custom_registries
      .iter_mut()
      .find(|registry| registry.lowercase_equal(&name, None))
    {
      let mut map = if registry.attrs.is_some() {
        registry.attrs.clone().unwrap()
      } else {
        BTreeMap::new()
      };
      map.insert(attr.clone().unwrap_or_default(), value.clone());
      (*registry).attrs = Some(map);

      if write_custom_registries(&custom_registries, Some(TRVONRC_PATH.as_path())).is_ok() {
        print_success(format!(
          "Set attribute '{}={}' of the registry '{name}' successfully.",
          attr.clone().unwrap_or_default(),
          value.clone().unwrap_or_default()
        ));
      }
    }

    if let Ok(current_registry) = get_current_registry() {
      if current_registry.lowercase_equal(&name, None) {
        let mut current_npmrc = ini::Ini::load_from_file(NPMRC_PATH.as_path()).unwrap_or_default();
        current_npmrc
          .with_section(None::<String>)
          .set(attr.unwrap_or_default(), value.unwrap_or_default());
        if current_npmrc.write_to_file(NPMRC_PATH.as_path()).is_err() {
          return Err(SetError::NpmrcWriteFail);
        }
      }
    }

    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum SetError {
  #[error("You cannot set attribute of the trvon internal registry.")]
  InternalRegistry,
  #[error("The registry '{0}' is not found.")]
  RegistryNotFound(String),
  #[error("The .npmrc file write fail.")]
  NpmrcWriteFail,
}
