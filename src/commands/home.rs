use crate::helper::{get_full_registries, is_registry_not_found};
use clap::Parser;
use thiserror::Error;
use webbrowser::{open_browser, Browser};

use super::Commander;

#[derive(Debug, Parser)]
pub struct Home {
  name: String,
  #[arg(value_enum)]
  browser: Option<Browser>,
}

impl Commander for Home {
  type Error = HomeError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    if is_registry_not_found(&self.name) {
      return Err(HomeError::RegistryNotFound(self.name));
    }

    let registries = get_full_registries().unwrap_or_default();
    let registry = registries
      .iter()
      .find(|registry| registry.lowercase_equal(&self.name, None))
      .unwrap();

    if registry.home.is_none() {
      return Err(HomeError::HomePageNotFound(self.name));
    }

    let browser = if self.browser.is_some() {
      self.browser.unwrap()
    } else {
      Browser::Default
    };

    if open_browser(browser, registry.home.clone().unwrap().as_str()).is_err() {
      return Err(HomeError::OpenBrowserError(browser.to_string()));
    }

    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum HomeError {
  #[error("The registry '{0}' is not found.")]
  RegistryNotFound(String),
  #[error("The homepage of registry '{0}' is not found.")]
  HomePageNotFound(String),
  #[error("Open {0} browser fail.")]
  OpenBrowserError(String),
}
