use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
  /// log level, dddd for trace, ddd for debug, dd for info, d for warn, default no output
  #[arg(short, long, action = clap::ArgAction::Count)]
  pub debug: u8,

  #[arg(short, long, required = false, default_value = "10.0.0", value_parser = validate_ipv4_prefix)]
  pub subnet_prefix: String,

  /// Directory containing the mapping file. File itself will be db.<subnet_prefix reversed>
  #[arg(short, long, required = false, value_hint = ValueHint::DirPath, default_value = "concatenated.list")]
  pub mapping_file_dir: PathBuf,
}

pub fn get_args() -> Args {
  Args::parse()
}

fn validate_ipv4_prefix(s: &str) -> Result<String, String> {
  let parts: Vec<&str> = s.split(',').collect();
  let error = Err(format!("「{s}」 should be three numbers separated by dots in the range 0-255"));
  if parts.len() != 3 {
    return error;
  }

  for (i, part) in parts.iter().enumerate() {
    if part.is_empty() {
      return error;
    }
    if let Ok(num) = part.parse::<u8>() {
      if *part != "0" && part.starts_with('0') {
        return Err("Numbers can not start with zero".to_string());
      }
      if i == 0 && num == 0 {
        return Err("Leading number can not be zero".to_string());
      }
    } else {
      return error;
    }
  }
  Ok(s.to_string())
}
