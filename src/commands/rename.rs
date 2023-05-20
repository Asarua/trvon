use clap::Parser;
use thiserror::Error;

use crate::{
  constants::{NPMRC_PATH, TRVONRC},
  helper::{
    get_current_registry, get_custom_registries_from_config, is_internal_registry,
    is_registry_not_found, join_home_path, print_success, write_custom_registries,
  },
};

use super::Commander;

#[derive(Debug, Parser)]
pub struct Rename {
  name: String,
  new_name: String,
}

impl Commander for Rename {
  type Error = RenameError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    if is_internal_registry(&self.name) {
      return Err(RenameError::InternalRegistry);
    }

    if is_registry_not_found(&self.name) {
      return Err(RenameError::RegistryNotFound(self.name));
    }

    if self.name.clone() == self.new_name.clone() {
      return Err(RenameError::SameName);
    }

    if !is_registry_not_found(&self.new_name) {
      return Err(RenameError::NewNameExist(self.new_name));
    }

    let mut custom_registries = get_custom_registries_from_config().unwrap_or_default();
    for custom_registry in custom_registries.iter_mut() {
      if custom_registry.lowercase_equal(&self.name, None) {
        (*custom_registry).name = self.new_name.clone()
      }
    }

    if write_custom_registries(
      &custom_registries,
      Some(join_home_path(TRVONRC).unwrap_or_default()),
    )
    .is_err()
    {
      return Err(RenameError::CustomRegistriesWriteFail);
    } else {
      print_success(format!(
        "The registry '{}' has been renamed to '{}'.",
        self.name, self.new_name
      ));

      if get_current_registry()
        .unwrap()
        .lowercase_equal(&self.name, None)
      {
        if let Ok(npmrc_content) = ini::Ini::load_from_file(NPMRC_PATH.as_path()).as_mut() {
          (*npmrc_content)
            .with_section(None::<String>)
            .set("name", self.new_name.clone());

          if npmrc_content.write_to_file(NPMRC_PATH.as_path()).is_err() {
            return Err(RenameError::NpmrcWriteFail);
          } else {
            print_success(format!(
              "Your .npmrc are using '{}', rewrite it to '{}' success",
              self.name, self.new_name
            ))
          }
        }
      }
    }
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum RenameError {
  #[error("You cannot rename the trvon internal registry.")]
  InternalRegistry,
  #[error("The registry '{0}' is not found.")]
  RegistryNotFound(String),
  #[error("The names cannot be the same.")]
  SameName,
  #[error("The new registry name '{0}' is already exist.")]
  NewNameExist(String),
  #[error("The .trvonrc file write fail.")]
  CustomRegistriesWriteFail,
  #[error("The .npmrc file write fail.")]
  NpmrcWriteFail,
}
