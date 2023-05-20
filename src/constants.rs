extern crate once_cell;

use crate::{helper::join_home_path, registry::Registry, registry_to_string};
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static NPMRC: &'static str = ".npmrc";
pub static TRVONRC: &'static str = ".trvonrc";

pub static NPMRC_PATH: Lazy<PathBuf> = Lazy::new(|| join_home_path(NPMRC).unwrap());
pub static TRVONRC_PATH: Lazy<PathBuf> = Lazy::new(|| join_home_path(TRVONRC).unwrap());

pub const DEFAULT_REGISTRIES: Lazy<Vec<Registry>> = Lazy::new(|| {
  vec![
    registry_to_string!(
      "npm",
      "https://registry.npmjs.org/",
      "https://www.npmjs.org"
    ),
    registry_to_string!(
      "yarn",
      "https://registry.yarnpkg.com/",
      "https://yarnpkg.com"
    ),
    registry_to_string!(
      "tencent",
      "https://mirrors.cloud.tencent.com/npm/",
      "https://mirrors.cloud.tencent.com/npm/"
    ),
    registry_to_string!("cnpm", "https://r.cnpmjs.org/", "https://cnpmjs.org"),
    registry_to_string!(
      "taobao",
      "https://registry.npmmirror.com/",
      "https://npmmirror.com",
    ),
    registry_to_string!(
      "npmMirror",
      "https://skimdb.npmjs.com/registry/",
      "https://skimdb.npmjs.com/"
    ),
  ]
});
