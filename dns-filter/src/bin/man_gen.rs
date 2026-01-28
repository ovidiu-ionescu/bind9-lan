use clap::CommandFactory; // Required to access command() method
use dns_filter_lib::cli::Args; // Import the cli struct
use shared::{ManExample, generate_man_page};

fn main() -> std::io::Result<()> {
  // 1. Build the command from the struct
  let cmd = Args::command();
  let examples = vec![ManExample {
    title: "Substitute the client ip addresse using /etc/bind9/zones/db.0.0.10",
    example: "cat dnsquery.log | dns-filter /etc/bind9/zones",
  }];
  generate_man_page(cmd, examples)?;

  Ok(())
}
