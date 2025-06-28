use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::io::{self, Write};

fn validate_pid(pid: &str) -> bool {
    pid.len() == 10 && pid.chars().all(|c| c.is_ascii_digit())
}

fn extract_xml_value(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    
    xml.find(&start_tag)
        .and_then(|start| {
            xml[start + start_tag.len()..]
                .find(&end_tag)
                .map(|end| xml[start + start_tag.len()..start + start_tag.len() + end].to_string())
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=======================");
    println!("Pretendo PID Analyzer");
    println!("=======================\n");

    let pid = loop {
        print!("Enter the PID: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            println!("Error: No PID entered.");
            continue;
        }

        if !validate_pid(input) {
            println!("No Valid PID. Please enter a valid 10-digit PID.");
            continue;
        }

        break input.to_string();
    };

    let client = Client::new();
    let url = format!("http://account.pretendo.cc/v1/api/miis?pids={}", pid);
    
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Nintendo-Client-ID",
        HeaderValue::from_static("a2efa818a34fa16b8afbc8a74eba3eda"),
    );
    headers.insert(
        "X-Nintendo-Client-Secret",
        HeaderValue::from_static("c91cdb5658bd4954ade78533a339cf9a"),
    );

    println!("Sending request to the server...");

    let response = client
        .get(&url)
        .headers(headers)
        .send()?
        .text()?;

    if let (Some(pnid), Some(name)) = (
        extract_xml_value(&response, "user_id"),
        extract_xml_value(&response, "name")
    ) {
        println!("\nRequest successful.\n");
        println!("PNID: {}", pnid);
        println!("Mii Name: {}", name);
    } else {
        println!("Error: Could not find user_id or name in response");
        println!("Raw response for debugging:\n{}", response);
    }

    println!("\nPress Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new())?;

    Ok(())
}
