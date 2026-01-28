use chrono::Local;
use clap_mangen::Man;
use roff::{Roff, roman};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct ManExample<'a> {
  pub title: &'a str,
  pub example: &'a str
}

pub fn generate_man_page(cmd: clap::Command, examples: Vec<ManExample>) -> std::io::Result<()> {
  // 1. Build the command from the struct

  // 2. Define output path (e.g., current directory or dist folder)
  // get first argument as output directory if provided
  let args: Vec<String> = std::env::args().collect();
  let out_dir = if args.len() > 1 { PathBuf::from(&args[1]) } else { PathBuf::from(".") };
  let file_path = out_dir.join(format!("{}.1", cmd.get_name()));
  let mut file = File::create(file_path)?;

  // 3. Render the Man page
  let now = Local::now();
  let date_str = now.format("%B %Y").to_string();
  Man::new(cmd).source("bind9-lan").date(&date_str).manual("User Manual").render(&mut file)?;

  // 4. Add an EXAMPLES section using roff crate
  let mut roff = Roff::new();
    roff.control("SH", ["EXAMPLES"]);
  for example in examples {
    roff.control("TP", ["2"])
    .control("B", [example.title])
    .text([roman(example.example)]);
  }

  file.write_all(roff.render().to_string().as_bytes())?;
  Ok(())
}

