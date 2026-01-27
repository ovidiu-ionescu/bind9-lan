use log::info;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
mod cli;

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
  let args = cli::get_args();

  shared::setup_logging(args.debug);
  let downloads = shared::fetch_lists(Some(args.lists_file), args.max_retries).await?;
  //  Prepare the output file
  let mut file = File::create(&args.output).await?;
  for fetch_result in downloads {
    if let Ok(content) = fetch_result.text {
      let sep_line = "#".repeat(80);
      file.write_all(format!("{}\n### {}\n{}\n", sep_line, fetch_result.url, sep_line).as_bytes()).await?;
      file.write_all(content.as_bytes()).await?;
    }
  }
  info!("Finished! Saved all list contents to 「{}」", args.output);
  Ok(())
}
