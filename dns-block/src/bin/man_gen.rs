use chrono::Local;
use clap::CommandFactory; // Required to access command() method
use clap_mangen::Man;
use dns_block_lib::cli::Args; // Import the cli struct
use roff::{Roff, bold, line_break, roman};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
  // 1. Build the command from the struct
  let cmd = Args::command();

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
  let example = Roff::new()
      .control("SH", ["EXAMPLES"])
      .control("TP", ["2"])
      .control("B", ["Create the rpz.db file from multiple block lists and an allow list:"])
      .text([
        roman("dns-block -dd -lists-file list_of_lists.txt own_list_of_lists.txt -block-file hosts_blocked.txt -allow-file domains.whitelisted pack -bind rpz.db"),
      ])
      .render();

  file.write_all(example.to_string().as_bytes())?;
  Ok(())
}
