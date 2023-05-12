use std::path::PathBuf;

use super::Commander;
use crate::{
  constants::{NPMRC, RNMRC_PATH},
  helper::{get_current_registry, get_full_registries, is_have_registry, read_file_from_home},
};
use anyhow::Result;
use clap::Parser;
use ini::Ini;
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Use {
  name: String,
}

impl Use {
  pub fn new<S: Into<String>>(name: S) -> Self {
    Use { name: name.into() }
  }
}

impl Commander for Use {
  type Error = UseError;

  fn apply(self) -> Result<(), Self::Error> {
    let name = self.name;
    if !is_have_registry(name.clone()) {
      return Err(Self::Error::RegistryNotFound(name));
    }
    let current_registry = get_current_registry().unwrap();
    if current_registry.lowercase_equal(&name, None) {
      return Ok(());
    }

    let registries = get_full_registries().unwrap();
    if let Some(registry_instance) = registries
      .iter()
      .find(|registry| registry.lowercase_equal(&name, None))
    {
      if let Ok(current_npmrc_content_raw) = read_file_from_home(PathBuf::from(NPMRC)) {
        if let Ok(current_npmrc_content) = Ini::load_from_file(current_npmrc_content_raw) {
          let mut ini_content = registry_instance.into_ini();

          for (prop_key, prop_value) in current_npmrc_content.iter() {
            for (key, value) in prop_value.iter() {
              ini_content.set_to(prop_key, key.into(), value.into())
            }
          }

          if ini_content.write_to_file(RNMRC_PATH.as_path()).is_err() {
            return Err(UseError::IoError {
              r#type: "write".into(),
            });
          }
        } else {
          return Err(UseError::NpmrcParseError);
        }
      } else {
        return Err(UseError::IoError {
          r#type: "read".into(),
        });
      }
    } else {
      return Err(UseError::RegistryNotFound(name));
    }

    Ok(())
  }
}

#[derive(Error, Debug)]
pub enum UseError {
  #[error("name {0}'s registry is not found!")]
  RegistryNotFound(String),
  #[error(".npmrc file {r#type} fail!")]
  IoError { r#type: String },
  #[error(".npmrc file parse fail!")]
  NpmrcParseError,
}
