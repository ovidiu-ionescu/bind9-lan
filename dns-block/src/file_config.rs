use std::{env, fs, path::PathBuf};

/// Determine the config directory path
fn get_base_dir() -> String {
  env::var("DNS_BLOCK_CONFIG_DIR").unwrap_or_else(|_| "/etc/dns-block".to_string())
}

fn list_files_in_directory(subdir: Option<&str>, extension: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
  let mut full_path = PathBuf::from(get_base_dir());
  if let Some(subdir) = subdir {
    full_path.push(subdir);
  }

  let files: Vec<PathBuf> = fs::read_dir(full_path)?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .filter(|path| path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(extension))
    .collect();

  Ok(files)
}

pub fn get_lists_files() -> Option<Vec<PathBuf>> {
  let res = list_files_in_directory(Some("lists_of_lists.d"), "txt").ok();
  res.filter(|v| !v.is_empty())
}

pub fn get_block_files() -> Option<Vec<PathBuf>> {
  let res = list_files_in_directory(Some("block_files.d"), "txt").ok();
  res.filter(|v| !v.is_empty())
}

pub fn get_allow_file() -> Option<PathBuf> {
  // 1. Determine the directory path
  let mut full_path = PathBuf::from(get_base_dir());
  full_path.push("domains.whitelisted");
  if full_path.is_file() { Some(full_path) } else { None }
}
