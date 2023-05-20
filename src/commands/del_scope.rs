use clap::Parser;
use thiserror::Error;

use crate::{constants::NPMRC_PATH, helper::print_success};

use super::Commander;

#[derive(Debug, Parser)]
pub struct DelScope {
  scope_name: String,
}

impl Commander for DelScope {
  type Error = DelScopeError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    let scope_registry_key = format!("{}:registry", self.scope_name);

    if let Ok(ini_content) = ini::Ini::load_from_file(NPMRC_PATH.as_path()).as_mut() {
      (*ini_content)
        .with_section(None::<String>)
        .delete(&scope_registry_key);
      if ini_content.write_to_file(NPMRC_PATH.as_path()).is_err() {
        return Err(DelScopeError::NpmrcWriteFail);
      } else {
        print_success(format!("Delete scope '{scope_registry_key}' success."))
      }
    }
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum DelScopeError {
  #[error("The .npmrc file write fail.")]
  NpmrcWriteFail,
}
