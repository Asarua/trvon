extern crate serde;

use ini::Ini;
use serde::{Deserialize, Serialize};

use crate::commands::add::Add;
use crate::helper::get_from_ini;

#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Registry {
  pub name: String,
  pub registry: String,
  pub home: Option<String>,
  #[serde(rename = "always-auth")]
  pub always_auth: bool,
  pub email: Option<String>,
  pub _auth: Option<String>,
}

#[macro_export]
macro_rules! registry_to_string {
  ($name:expr, $registry: expr, $home: expr$(,)?) => {
    Registry::new(
      String::from($name),
      String::from($registry),
      Some(String::from($home)),
    )
  };
}

impl Registry {
  pub fn new(name: String, registry: String, home: Option<String>) -> Self {
    Self {
      name,
      registry,
      home,
      email: None,
      always_auth: false,
      _auth: None,
    }
  }
}

impl Registry {
  pub fn into_ini(&self) -> Ini {
    let Registry {
      name,
      registry,
      home,
      email,
      always_auth,
      _auth,
    } = self;
    let mut ini_instance = Ini::new();
    ini_instance
      .with_section(None::<String>)
      .set("name", name)
      .set("registry", registry)
      .set("home", home.clone().unwrap_or_default())
      .set("email", email.clone().unwrap_or_default())
      .set("_auth", _auth.clone().unwrap_or_default())
      .set("always-auth", always_auth.to_string());

    ini_instance
  }

  pub fn lowercase_equal<T: AsRef<str>>(&self, other: &T, parse_key: Option<String>) -> bool {
    match parse_key {
      Some(key) => match key.as_str() {
        "name" => self.name.to_lowercase() == other.as_ref().to_lowercase(),
        "registry" => self.registry.to_lowercase() == other.as_ref().to_lowercase(),
        "home" => {
          self.home.clone().unwrap_or_default().to_lowercase() == other.as_ref().to_lowercase()
        }
        _ => false,
      },
      None => self.name.to_lowercase() == other.as_ref().to_lowercase(),
    }
  }
}

impl From<Ini> for Registry {
  fn from(value: Ini) -> Self {
    let name = get_from_ini(&value, "name", None::<String>, None);
    let registry = get_from_ini(&value, "registry", None::<String>, None);
    let home = get_from_ini(&value, "home", None::<String>, None);

    registry_to_string!(name, registry, home)
  }
}

impl From<Add> for Registry {
  fn from(value: Add) -> Self {
    Self::new(value.name, value.registry, value.home)
  }
}
