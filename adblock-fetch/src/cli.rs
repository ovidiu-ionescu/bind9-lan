use clap::{Parser, ValueHint};
use std::fs::metadata;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "adblock-fetch", author, version, about, long_about, before_help = "bind9-lan")]
pub struct Args {
  /// log level, dddd for trace, ddd for debug, dd for info, d for warn, default no output
  #[arg(short, long, action = clap::ArgAction::Count)]
  pub debug: u8,

  /// Files containing urls to lists
  #[arg(short, long, required = true, num_args = 1.., value_delimiter = ' ', value_hint = ValueHint::FilePath, value_parser = validate_readable_file)]
  pub lists_file: Vec<PathBuf>,

  /// Name of the output file
  #[arg(short, long, required = false, value_hint = ValueHint::FilePath, default_value = "concatenated.list")]
  pub output: String,

  /// How many times to retry if downloading a list fails
  #[arg(short, long, default_value_t = 3)]
  pub max_retries: u32,
}

pub fn get_args() -> Args {
  Args::parse()
}

fn validate_readable_file(s: &str) -> Result<PathBuf, String> {
  let path = PathBuf::from(s);

  // Check if path exists and is a file
  let meta = metadata(&path).map_err(|_| format!("'{}' does not exist", s))?;

  if !meta.is_file() {
    return Err(format!("'{}' is not a file", s));
  }

  // Check if readable by attempting to open (optional but thorough)
  if std::fs::File::open(&path).is_err() {
    return Err(format!("'{}' is not readable (permission denied)", s));
  }

  Ok(path)
}
