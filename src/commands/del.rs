use super::Commander;
use crate::{
  commands::r#use::Use,
  constants::TRVONRC_PATH,
  helper::{
    get_current_registry, get_custom_registries_from_config, is_internal_registry,
    is_registry_not_found, print_success, write_custom_registries,
  },
};
use clap::Parser;
use thiserror::Error;

#[derive(Debug, Parser)]
pub struct Del {
  name: String,
}

impl Commander for Del {
  type Error = DelError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    if is_internal_registry(&self.name) {
      return Err(DelError::InternalRegistry);
    }

    if is_registry_not_found(&self.name) {
      return Err(DelError::RegistryNotFound(self.name));
    }

    let mut custom_registries = get_custom_registries_from_config().unwrap_or_default();
    if let Some(index) = custom_registries
      .iter()
      .position(|cus| cus.lowercase_equal(&self.name, None))
    {
      custom_registries.remove(index);
      print_success(format!(
        "The registry '{}' has been deleted successfully.",
        self.name
      ));

      if let Err(_) = write_custom_registries(&custom_registries, Some(TRVONRC_PATH.as_path())) {
        return Err(DelError::CustomRegistriesWriteFail);
      } else {
        let current_registry = get_current_registry().unwrap();
        if current_registry.lowercase_equal(&self.name, None) {
          Use::call(Use::new("npm"))
        }
      }
    } else {
      return Err(DelError::RegistryNotFound(self.name));
    }
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum DelError {
  #[error("You cannot delete the trvon internal registry.")]
  InternalRegistry,
  #[error("The registry '{0}' is not found.")]
  RegistryNotFound(String),
  #[error("Custom registries write fail.")]
  CustomRegistriesWriteFail,
}
