use clap::CommandFactory; // Required to access command() method
use dns_block_lib::cli::Args; // Import the cli struct
use shared::{ManExample, generate_man_page};

fn main() -> std::io::Result<()> {
  // 1. Build the command from the struct
  let cmd = Args::command();
  let examples = vec![ManExample {
    title: "Create the rpz.db file from multiple block lists and an allow list:",
    example: "dns-block -dd -lists-file list_of_lists.txt own_list_of_lists.txt -block-file hosts_blocked.txt -allow-file domains.whitelisted pack -bind rpz.db",
  }];
  generate_man_page(cmd, examples)?;

  Ok(())
}
