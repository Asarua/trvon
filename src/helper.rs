use crate::{
  constants::{DEFAULT_REGISTRIES, NPMRC, TRVONRC, TRVONRC_PATH},
  registry::Registry,
};
use anyhow::Result;
use colored::Colorize;
use ini::Ini;
use std::{
  collections::HashSet,
  fmt::Display,
  fs,
  path::{Path, PathBuf},
  time::{SystemTime, UNIX_EPOCH},
};

pub fn join_home_path<S: AsRef<Path>>(path: S) -> Result<PathBuf> {
  if let Some(home) = dirs::home_dir() {
    Ok(home.join(path))
  } else {
    panic!("Home dir found fail.")
  }
}

pub fn read_file(path: PathBuf) -> Result<String> {
  if let Ok(file) = fs::read_to_string(path) {
    Ok(file)
  } else {
    Ok(String::new())
  }
}

pub fn read_file_from_home(path: PathBuf) -> Result<String> {
  let file = read_file(join_home_path(path)?)?;
  Ok(file)
}

pub fn get_current_registry() -> Result<Registry> {
  let ini_current = read_file_from_home(PathBuf::from(NPMRC))?;
  let ini = Ini::load_from_str(ini_current.as_str())?;
  Ok(Registry::from(ini))
}

pub fn get_custom_registries_from_config() -> Result<Vec<Registry>> {
  let custom_registries_content = read_file_from_home(PathBuf::from(TRVONRC))?;
  if let Ok(custom_registries) = serde_json::from_str::<Vec<Registry>>(&custom_registries_content) {
    Ok(custom_registries)
  } else {
    Ok(vec![])
  }
}

pub fn get_full_registries() -> Result<Vec<Registry>> {
  let registries: HashSet<Registry> = HashSet::from_iter(
    DEFAULT_REGISTRIES
      .iter()
      .map(|r| r.clone())
      .chain(get_custom_registries_from_config()?)
      .collect::<Vec<Registry>>(),
  );
  Ok(registries.into_iter().collect::<Vec<Registry>>())
}

pub fn is_have_registry(name: &String) -> bool {
  let full_registries = get_full_registries().unwrap_or_default();
  let have = full_registries
    .iter()
    .any(|registry| registry.lowercase_equal(name, None));
  have
}

pub fn is_internal_registry(name: &String) -> bool {
  DEFAULT_REGISTRIES
    .iter()
    .any(|registry| registry.lowercase_equal(name, None))
}

pub fn is_registry_not_found(name: &String) -> bool {
  !get_full_registries()
    .unwrap_or_default()
    .iter()
    .any(|registry| registry.lowercase_equal(name, None))
}

pub fn write_custom_registries<P: AsRef<Path>>(
  registries: &Vec<Registry>,
  path: Option<P>,
) -> Result<()> {
  let content = serde_json::to_string(registries)?;
  let path = if let Some(path) = path {
    PathBuf::from(path.as_ref())
  } else {
    TRVONRC_PATH.clone()
  };
  std::fs::write(path, content)?;
  Ok(())
}

pub fn get_from_ini<'a, S>(
  instance: &'a Ini,
  key: &str,
  section: Option<S>,
  default: Option<&'a str>,
) -> &'a str
where
  S: Into<String>,
{
  let v = Ini::get_from_or(instance, section, key, default.unwrap_or(""));
  v
}

pub fn print_success<S: Display>(content: S) {
  println!("{}", format!("{}  {}", "Success".green(), content));
}

pub fn get_now_timestamp() -> u128 {
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_millis();
  timestamp
}
