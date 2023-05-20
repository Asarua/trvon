use clap::Parser;
use thiserror::Error;

use crate::{
  constants::{NPMRC_PATH, TRVONRC_PATH},
  helper::{
    get_current_registry, get_custom_registries_from_config, is_internal_registry,
    is_registry_not_found, print_success, write_custom_registries,
  },
};

use super::Commander;

#[derive(Debug, Parser)]
pub struct SetHostedRepo {
  name: String,
  repo: String,
}

impl Commander for SetHostedRepo {
  type Error = SetHostedRepoError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    let Self { name, repo } = self;

    if is_internal_registry(&name) {
      return Err(SetHostedRepoError::InternalRegistry);
    }

    if is_registry_not_found(&name) {
      return Err(SetHostedRepoError::RegistryNotFound(name));
    }

    let mut custom_registries = get_custom_registries_from_config().unwrap_or_default();
    for custom_registry in custom_registries.iter_mut() {
      if custom_registry.lowercase_equal(&name, None) {
        (*custom_registry).repository = Some(repo.clone());
      }
    }

    if write_custom_registries(&custom_registries, Some(TRVONRC_PATH.as_path())).is_ok() {
      print_success(format!("Set the {repo} of registry '{name}' successfully."));

      if let Ok(current_registry) = get_current_registry() {
        if current_registry.lowercase_equal(&name, None) {
          let mut npmrc_content =
            ini::Ini::load_from_file(NPMRC_PATH.as_path()).unwrap_or_default();
          npmrc_content
            .with_section(None::<String>)
            .set("repository", repo);
          if npmrc_content.write_to_file(NPMRC_PATH.as_path()).is_ok() {
            print_success("Set repository attribute of npmrc successfully")
          } else {
            return Err(SetHostedRepoError::NpmrcWriteFail);
          }
        }
      } else {
        return Err(SetHostedRepoError::CurrentRegistryError);
      }
    } else {
      return Err(SetHostedRepoError::WriteCustomRegistriesError);
    }

    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum SetHostedRepoError {
  #[error("You cannot set repository of the trvon internal registry.")]
  InternalRegistry,
  #[error("The registry '{0}' is not found.")]
  RegistryNotFound(String),
  #[error("The custom registries file write fail.")]
  WriteCustomRegistriesError,
  #[error("The .npmrc file parse fail.")]
  CurrentRegistryError,
  #[error("The .npmrc file write fail.")]
  NpmrcWriteFail,
}
