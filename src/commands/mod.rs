extern crate anyhow;

pub mod add;
pub mod current;
pub mod del;
pub mod del_scope;
pub mod home;
pub mod login;
pub mod ls;
pub mod rename;
pub mod set;
pub mod set_hosted_repo;
pub mod set_scope;
pub mod test;
pub mod r#use;

use anyhow::Result;
use colored::Colorize;

pub trait Commander: Sized {
  type Error: std::error::Error;

  fn apply(self) -> Result<(), Self::Error>;

  fn handle_err(err: Self::Error) {
    let err_display = format!("{err}");
    println!("{}", err_display.red().bold());
    std::process::exit(1)
  }

  fn call(self) {
    match self.apply() {
      Ok(()) => (),
      Err(err) => Self::handle_err(err),
    }
  }
}
