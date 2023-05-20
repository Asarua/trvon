use base64::Engine;
use clap::{Args, Parser};
use thiserror::Error;

use crate::{
  constants::TRVONRC_PATH,
  helper::{
    get_custom_registries_from_config, is_internal_registry, is_registry_not_found, print_success,
    write_custom_registries,
  },
};

use super::Commander;

#[derive(Args, Debug)]
pub struct LoginArgs {
  #[arg(long, short = 'a', help = "Set is always auth")]
  always_auth: bool,
  #[arg(long, short = 'u', help = "Your user name for this registry")]
  username: Option<String>,
  #[arg(long, short = 'p', help = "Your password for this registry")]
  password: Option<String>,
  #[arg(long, short = 'e', help = "Your email for this registry")]
  email: Option<String>,
}

#[derive(Debug, Parser)]
pub struct Login {
  #[arg(help = "The name of the user")]
  name: String,
  #[arg(help = "A base64 encoded string")]
  base64: Option<String>,
  #[command(flatten)]
  args: LoginArgs,
}

impl Commander for Login {
  type Error = LoginError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    let Self { name, base64, args } = self;
    let LoginArgs {
      always_auth,
      username,
      email,
      password,
    } = args;

    if is_internal_registry(&name) {
      return Err(LoginError::InternalRegistry);
    }

    if is_registry_not_found(&name) {
      return Err(LoginError::RegistryNotFound(name));
    }

    let mut custom_registries = get_custom_registries_from_config().unwrap_or_default();
    let mut registry = custom_registries
      .iter_mut()
      .find(|registry| registry.lowercase_equal(&name, None))
      .unwrap();

    if base64.is_some() {
      (*registry)._auth = Some(base64.unwrap())
    } else if username.is_some() && password.is_some() {
      (*registry)._auth = Some(
        base64::engine::general_purpose::STANDARD
          .encode(format!("{}:{}", username.unwrap(), password.unwrap()).as_bytes()),
      )
    } else {
      return Err(LoginError::AuthInfoMiss);
    }

    if always_auth {
      (*registry).always_auth = true
    }

    if email.is_some() && !email.as_ref().unwrap().trim().is_empty() {
      (*registry).email = email
    }

    if let Err(_) = write_custom_registries(&custom_registries, Some(TRVONRC_PATH.as_path())) {
      return Err(LoginError::WriteCustomRegistriesError);
    } else {
      print_success(format!(
        "Set the authorization information of the registry '{name}' success."
      ));
    }

    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum LoginError {
  #[error("You cannot set authorization information of the trvon internal registry.")]
  InternalRegistry,
  #[error("The registry '{0}' is not found.")]
  RegistryNotFound(String),
  #[error("Authorization information in base64 format or username & password is required")]
  AuthInfoMiss,
  #[error("The custom registries file write fail.")]
  WriteCustomRegistriesError,
}
