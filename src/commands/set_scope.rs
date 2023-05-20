use clap::Parser;
use thiserror::Error;

use crate::{constants::NPMRC_PATH, helper::print_success};

use super::Commander;

#[derive(Debug, Parser)]
pub struct SetScope {
  scope_name: String,
  url: String,
}

impl Commander for SetScope {
  type Error = SetScopeError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    let scope_registry_key = format!("{}:registry", self.scope_name);

    if let Ok(ini_content) = ini::Ini::load_from_file(NPMRC_PATH.as_path()).as_mut() {
      (*ini_content)
        .with_section(None::<String>)
        .set(scope_registry_key, self.url);
      if ini_content.write_to_file(NPMRC_PATH.as_path()).is_err() {
        return Err(SetScopeError::NpmrcWriteFail);
      } else {
        print_success("Set repository attribute of npmrc successfully")
      }
    }
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum SetScopeError {
  #[error("The .npmrc file write fail.")]
  NpmrcWriteFail,
}
