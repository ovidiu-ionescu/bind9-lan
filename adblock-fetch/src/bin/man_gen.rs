use adblock_fetch_lib::cli::Args; // Import the cli struct
use clap::CommandFactory; // Required to access command() method
use shared::{ManExample, generate_man_page};

fn main() -> std::io::Result<()> {
  // 1. Build the command from the struct
  let cmd = Args::command();
  let examples = vec![ManExample {
    title: "Fetch the content of the lists of lists and put the output in concatenated.list",
    example: "adblock_fetch -dd -lists-file list_of_lists.txt own_list_of_lists.txt --output concatenated.list",
  }];
  generate_man_page(cmd, examples)?;

  Ok(())
}
