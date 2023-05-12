extern crate clap;

use crate::commands::{self, Commander};
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// List all the registries
  Ls(commands::ls::Ls),
  /// Change current registry
  Use(commands::r#use::Use),
  /// Show current registry name or URL
  Current(commands::current::Current),
  /// Add custom registry
  Add(commands::add::Add),
  /// Set authorize information for a custom registry with a base64 encoded string or username and password
  Login(commands::login::Login),
  /// Set hosted npm repository for a custom registry to publish package
  SetHostedRepo(commands::set_hosted_repo::SetHostedRepo),
  /// Associating a scope with a registry
  SetScope(commands::set_scope::SetScope),
  /// Remove a scope
  DelScope(commands::del_scope::DelScope),
  /// Set a custom registry attribute
  Set(commands::set::Set),
  /// Change custom registry name
  Rename(commands::rename::Rename),
  /// Delete custom registry
  Del(commands::del::Del),
  /// Open the homepage of registry with optional browser
  Home(commands::home::Home),
  /// Show response time for specific or all registries
  Test(commands::test::Test),
}

impl Commands {
  pub fn call(self) {
    match self {
      Self::Ls(cmd) => cmd.call(),
      Self::Use(cmd) => cmd.call(),
      Self::Current(cmd) => cmd.call(),
      Self::Add(cmd) => cmd.call(),
      Self::Login(cmd) => cmd.call(),
      Self::SetHostedRepo(cmd) => cmd.call(),
      Self::SetScope(cmd) => cmd.call(),
      Self::DelScope(cmd) => cmd.call(),
      Self::Set(cmd) => cmd.call(),
      Self::Rename(cmd) => cmd.call(),
      Self::Del(cmd) => cmd.call(),
      Self::Home(cmd) => cmd.call(),
      Self::Test(cmd) => cmd.call(),
    }
  }
}

#[derive(Parser, Debug)]
#[command(name = "rnm", version = env!("CARGO_PKG_VERSION"), bin_name = "rnm")]
pub struct Cli {
  #[clap(subcommand)]
  pub subcmd: Commands,
}

pub fn parse() -> Cli {
  let cli = Cli::parse();
  cli
}
