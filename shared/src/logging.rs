use env_logger::Builder;
use log::LevelFilter;

pub fn setup_logging(level: u8) {
  let level_filter = match level {
    0 => LevelFilter::Error,
    1 => LevelFilter::Warn,
    2 => LevelFilter::Info,
    3 => LevelFilter::Debug,
    _ => LevelFilter::Trace,
  };
  let mut builder = Builder::from_default_env();
  // if the RUST_LOG var is not defined we use the debug level
  if std::env::var("RUST_LOG").is_err() {
    builder.filter_level(level_filter);
  }
  builder.format_timestamp_secs().init();
}
