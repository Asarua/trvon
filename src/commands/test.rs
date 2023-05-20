use super::Commander;
use clap::Parser;
use colored::Colorize;
use std::{
  cell::Cell,
  sync::{Arc, Mutex},
  thread::{self, JoinHandle},
};
use thiserror::Error;

use crate::helper::{get_current_registry, get_now_timestamp};
use crate::{
  helper::{get_full_registries, is_registry_not_found},
  registry::Registry,
};

#[derive(Debug, Parser)]
pub struct Test {
  name: Option<String>,
}

impl Commander for Test {
  type Error = TestError;

  fn apply(self) -> anyhow::Result<(), Self::Error> {
    let mut registries = get_full_registries().unwrap_or_default();

    let Self { name } = self;
    if let Some(inner_registry) = name.clone() {
      if is_registry_not_found(&inner_registry) {
        return Err(TestError::TestRegistryNotFound(inner_registry));
      } else {
        registries = registries
          .into_iter()
          .filter(|registry| registry.lowercase_equal(&inner_registry, None))
          .collect::<Vec<Registry>>();
      }
    }

    let timeout_seconds = 5;
    let command_test_results = Arc::new(Mutex::new(Cell::new(
      Vec::<CommandTestResult>::with_capacity(registries.len()),
    )));
    let mut handles = Vec::<JoinHandle<()>>::with_capacity(registries.len());

    for i in 0..registries.len() {
      let current_registry = registries.get(i).unwrap().clone();
      let command_test_results_copy = Arc::clone(&command_test_results);

      let handle = thread::spawn(move || {
        let mut command_test_result = CommandTestResult::new(current_registry.name);
        let start_time = get_now_timestamp();
        let client = reqwest::blocking::Client::builder()
          .timeout(std::time::Duration::from_secs(timeout_seconds))
          .build()
          .unwrap();

        if let Ok(response) = client
          .get(format!("{}trvon", current_registry.registry))
          .send()
        {
          let end_time = get_now_timestamp();
          if response.status().is_success() {
            command_test_result.success = true
          } else if response.status().as_u16() == 408 {
            command_test_result.is_timeout = true
          }

          command_test_result.time = (end_time - start_time) as u16;
        }

        let mut mut_results = command_test_results_copy.lock().unwrap();
        mut_results.get_mut().push(command_test_result);
      });
      handles.push(handle);
    }

    for i in handles {
      i.join().unwrap();
    }

    let mut container = command_test_results.lock().unwrap();
    let results = container.get_mut();
    let fastest = results
      .iter()
      .filter(|result| result.success)
      .map(|result| result.time)
      .min()
      .unwrap_or_default();

    let current_registry = get_current_registry().unwrap();
    let error_msg = format!(
      "{}",
      " (Fetch error, if this is your private registry, please ignore)".red()
    );
    let timeout_msg = format!(
      "{}",
      format!(" (Fetch timeout over {timeout_seconds} s)").yellow()
    );
    let length = results
      .iter()
      .map(|result| result.name.chars().count())
      .max()
      .unwrap_or_default()
      + 3;

    for result in results {
      let CommandTestResult {
        name: current_name,
        success,
        time,
        is_timeout,
      } = result;
      let is_fastest = fastest == *time;
      let prefix = if current_registry.lowercase_equal(current_name, None) {
        format!("{}", " *".green())
      } else {
        String::from("  ")
      };
      let mut suffix = if is_fastest && name.clone().is_none() {
        format!("{}", format!("{} ms", time).bright_green())
      } else {
        if *is_timeout {
          String::from("timeout")
        } else {
          format!("{time} ms")
        }
      };

      if !(*success) {
        suffix.push_str(
          if *is_timeout {
            timeout_msg.clone()
          } else {
            error_msg.clone()
          }
          .as_str(),
        )
      }

      let mut print_info = String::new();
      print_info.push_str(&prefix);
      print_info.push_str(&current_name);
      print_info.push(' ');
      print_info.push_str(&"-".repeat(length - current_name.chars().count()));
      print_info.push(' ');
      print_info.push_str(&suffix);

      println!("{print_info}")
    }

    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum TestError {
  #[error("The test registry {0} not found.")]
  TestRegistryNotFound(String),
}

#[derive(Clone, Debug)]
struct CommandTestResult {
  name: String,
  success: bool,
  time: u16,
  is_timeout: bool,
}

impl CommandTestResult {
  fn new(name: String) -> Self {
    CommandTestResult {
      name,
      success: false,
      time: 0,
      is_timeout: false,
    }
  }
}
