use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;

use futures::future::join_all;
use log::{error, info, warn};
use num_format::{Locale, ToFormattedString};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;

use std::sync::OnceLock;

type BoxError = Box<dyn std::error::Error>;

pub struct FetchResult {
  pub url: String,
  pub text: reqwest::Result<String>,
}

static CLIENT: OnceLock<ClientWithMiddleware> = OnceLock::new();

fn get_client() -> ClientWithMiddleware {
  let client = CLIENT.get().expect("Client not initialized");
  // We clone the client (it's backed by an Arc, so it's cheap)
  client.clone()
}

/// Resolves a list of list urls by downloading them all
pub async fn fetch_lists(lists_file: Option<Vec<PathBuf>>, max_retries: u32) -> Result<Vec<FetchResult>, BoxError> {
  let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries);
  let client = ClientBuilder::new(Client::new()).with(RetryTransientMiddleware::new_with_policy(retry_policy)).build();
  if CLIENT.set(client).is_err() {
    warn!("Client already initialized");
  }

  let lists_files = match lists_file {
    Some(l) => {
      if l.is_empty() {
        return Ok(Vec::new());
      } else {
        l
      }
    }
    None => return Ok(Vec::new()),
  };

  let file_results = join_all(lists_files.iter().map(get_url_list)).await;
  let mut urls = HashSet::new();

  for result in file_results {
    // 1. Handle the Result (stop if there's an error)
    let inner_set = result?;

    for url in inner_set {
      // 2. Try to insert. If it returns false, it was a duplicate.
      if !urls.insert(url.clone()) {
        warn!("「{}」is a duplicate, skipping it", url);
      }
    }
  }

  // 1. Create a list of futures (one for each request)
  let tasks = urls.into_iter().map(download_url);

  // 2. Execute all tasks in parallel
  info!("Fetching URLs...");
  let results: Vec<Result<FetchResult, BoxError>> = join_all(tasks).await;

  let mut ret = Vec::with_capacity(results.len());
  // 4. Filter for successful results and concatenate
  info!("Validate downloads");
  let mut total_hosts = 0;
  for fetch_result in results.into_iter().flatten() {
    match fetch_result.text {
      Ok(ref content) => {
        let number_of_valid_lines = number_of_valid_lines(content);
        if number_of_valid_lines > 0 {
          total_hosts += number_of_valid_lines;
          info!("{:>12} valid hosts in 「{}」", number_of_valid_lines.to_formatted_string(&Locale::en_NL), fetch_result.url);
          ret.push(fetch_result);
        } else {
          warn!("Url 「{}」 returned no valid content", fetch_result.url);
        }
      }
      Err(e) => warn!("Error fetching a url 「{}」: {}", fetch_result.url, e),
    }
  }

  info!(
    "Finished downloading. We got {} hosts in total but they most likely contain duplicates.",
    total_hosts.to_formatted_string(&Locale::en_NL)
  );
  Ok(ret)
}

fn extract_dns_block_url(content: &str) -> Option<&str> {
  let first_line = content.lines().next()?.trim();
  println!("first line: {first_line}");
  let prefix = "#!dns-block ";
  if content.starts_with(prefix) { first_line[prefix.len()..].split_whitespace().next() } else { None }
}

#[cfg(test)]
mod test1 {
  #[test]
  fn extract_dns_block_url() {
    let content = "#!dns-block https://v.firebog.net/hosts/lists.php?type=nocross";
    assert_eq!(Some("https://v.firebog.net/hosts/lists.php?type=nocross"), super::extract_dns_block_url(content));
  }
}

async fn download_url(url: String) -> Result<FetchResult, BoxError> {
  let res = match get_client().get(&url).send().await {
    Ok(response) => response,
    Err(e) => {
      error!("✗ Could not download 「{}」, error: 「{}」", &url, e);
      return Err(Box::new(e) as BoxError);
    }
  };
  let text = res.text().await;
  info!("✓ downloaded「{url}」");
  Ok(FetchResult { url, text })
}

async fn get_url_list(filename: &PathBuf) -> Result<HashSet<String>, BoxError> {
  let mut content = read_to_string(filename)?;

  // if the file has a link url at the start, we try to download a fresh instance
  if let Some(url) = extract_dns_block_url(&content) {
    info!("File 「{}」 has a refresh url in the first line, try to download that", filename.display());
    if let Ok(r) = download_url(url.to_string()).await
      && let Ok(res) = r.text
    {
      content = res;
    }
  }
  let mut urls = HashSet::<String>::new();
  content.lines().for_each(|line| {
    let cleaned = match line.split('#').next() {
      Some(it) => it,
      None => return,
    }
    .trim();
    if !cleaned.is_empty() && !urls.insert(cleaned.to_string()) {
      warn!("「{}」 is a duplicate", cleaned);
    }
  });
  if urls.is_empty() { Err("No urls found in file".into()) } else { Ok(urls) }
}

/// return the number of valid lines
/// zero would indicate an invalid list
fn number_of_valid_lines(content: &str) -> usize {
  content
    .lines()
    .filter_map(|line| {
      let cleaned = line.split('#').next()?.trim();
      if cleaned.is_empty() { None } else { Some(cleaned.to_string()) }
    })
    .filter(|line| {
      match line.split(' ').next_back() {
        Some(host) => match addr::parse_dns_name(host) {
          Ok(_) => Some(host),
          Err(_) => None,
        },
        _ => None,
      }
      .is_some()
    })
    .count()
}
