use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use regex::Regex;

fn main() -> io::Result<()> {
    // 1. Configuration: Set your subnet prefix here
    let subnet_prefix = "10.0.0"; 
    let mut ip_map = HashMap::new();

    // 2. Parse the PTR records from db.0.0.0
    let mapping_file = File::open("db.0.0.10")?;
    let reader = BufReader::new(mapping_file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Typical PTR format: [octet] IN PTR [hostname]
        if let Some(ptr_idx) = parts.iter().position(|&r| r == "PTR") {
            if ptr_idx > 0 && parts.len() > ptr_idx + 1 {
                let last_octet = parts[0];
                let hostname = parts[ptr_idx + 1].trim_end_matches('.');
                let full_ip = format!("{}.{}", subnet_prefix, last_octet);
                
                ip_map.insert(full_ip, hostname.to_string());
            }
        }
    }

    // 3. Setup Stream Processing
    let ip_regex = Regex::new(r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line?;
        let result = ip_regex.replace_all(&line, |caps: &regex::Captures| {
            let ip = &caps[0];
            // If the IP is in our PTR map, swap it; otherwise, keep the IP
            ip_map.get(ip).cloned().unwrap_or_else(|| ip.to_string())
        });

        writeln!(handle, "{}", result)?;
    }

    Ok(())
}

