use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;

use clap::{Parser, ValueHint};
use futures::future::join_all;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

type BoxError = Box<dyn std::error::Error>;

struct FetchResult {
  url: String,
  text: reqwest::Result<String>,
}

#[derive(Parser, Debug)]
struct Args {
  #[arg(short, long, required = true, value_hint = ValueHint::FilePath)]
  files: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
  let args = Args::parse();

  let urls: HashSet<String> =
    args.files.iter().map(|filename| get_url_list(filename)).try_fold(HashSet::new(), |mut acc, set| {
      let set = set?;
      if acc.is_empty() {
        acc = set;
      } else {
        for s in set {
          if !acc.insert(s.clone()) {
            eprintln!("「{}」is a duplicate", s);
          }
        }
      }
      Ok::<HashSet<String>, BoxError>(acc)
    })?;

  let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
  let client = ClientBuilder::new(Client::new()).with(RetryTransientMiddleware::new_with_policy(retry_policy)).build();

  // 1. Create a list of futures (one for each request)
  // We clone the client (it's backed by an Arc, so it's cheap)
  let tasks = urls.into_iter().map(|url| {
    let client = client.clone();
    async move {
      let res = client.get(&url).send().await?;
      let text = res.text().await;
      Ok(FetchResult { url, text })
    }
  });

  // 2. Execute all tasks in parallel
  println!("Fetching URLs...");
  let results: Vec<Result<FetchResult, BoxError>> = join_all(tasks).await;

  // 3. Prepare the output file
  let mut file = File::create("concatenated_results.txt").await?;

  // 4. Filter for successful results and concatenate
  for result in results {
    match result {
      Ok(fetch_result) => match fetch_result.text {
        Ok(content) => {
          if has_valid_content(&content) {
            let sep_line = "#".repeat(80);
            file.write_all(format!("{}\n### {}\n{}\n", sep_line, fetch_result.url, sep_line).as_bytes()).await?;
            file.write_all(content.as_bytes()).await?;
          } else {
            eprintln!("Url 「{}」 returned no valid content", fetch_result.url);
          }
        }
        Err(e) => eprintln!("Error fetching a url 「{}」: {}", fetch_result.url, e),
      },
      Err(e) => eprintln!("Error fetching a URL: {}", e),
    }
  }

  println!("Finished! Results saved to concatenated_results.txt");
  Ok(())
}

fn get_url_list(filename: &PathBuf) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
  let content = read_to_string(filename)?;
  let mut urls = HashSet::<String>::new();
  content.lines().for_each(|line| {
    let cleaned = match line.split('#').next() {
      Some(it) => it,
      None => return,
    }
    .trim();
    if !cleaned.is_empty() && !urls.insert(cleaned.to_string()) {
      eprintln!("「{}」 is a duplicate", cleaned);
    }
  });
  if urls.is_empty() { Err("No urls found in file".into()) } else { Ok(urls) }
}

/// check if the list content contains any valid lines
/// this would indicate an invalid list
fn has_valid_content(content: &str) -> bool {
  content
    .lines()
    .filter_map(|line| {
      let cleaned = line.split('#').next()?.trim();
      if cleaned.is_empty() { None } else { Some(cleaned.to_string()) }
    })
    .any(|line| {
      match line.split(' ').next_back() {
        Some(host) => match addr::parse_dns_name(host) {
          Ok(_) => Some(host),
          Err(_) => None,
        },
        _ => None,
      }
      .is_some()
    })
}
