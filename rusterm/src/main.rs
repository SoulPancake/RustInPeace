use dialoguer::{theme::ColorfulTheme, Select, Input};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use ping_rs::{PingOptions, send_ping};
use dns_lookup::{lookup_host};

fn main() {
    let top_domains = [
        "google.com",
        "facebook.com",
        "youtube.com",
        "twitter.com",
        "amazon.com",
        "linkedin.com",
        "instagram.com",
        "pinterest.com",
        "reddit.com",
        "wikipedia.org",
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&["Say hello", "Ping", "Check internet speed", "Exit"])
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => println!("ฅ^•ﻌ•^ฅ Hello!"),
            1 => {
                let ping_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a domain to ping:")
                    .items(&top_domains)
                    .default(0)
                    .interact()
                    .unwrap();

                let domain = if ping_selection < top_domains.len() {
                    top_domains[ping_selection].to_string()
                } else {
                    let custom_domain: String = Input::new()
                        .with_prompt("Enter a domain to ping:")
                        .interact()
                        .unwrap();
                    custom_domain
                };

                let ips = match lookup_host(&domain) {
                    Ok(ips) => ips,
                    Err(_) => {
                        eprintln!("Failed to lookup host {}", domain);
                        continue;
                    }
                };

                let addr = ips.into_iter().next().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

                let data = [1, 2, 3, 4]; // ping data
                let timeout = Duration::from_secs(1);
                let options = PingOptions { ttl: 128, dont_fragment: true };

                match send_ping(&addr, timeout, &data, Some(&options)) {
                    Ok(reply) => {
                        println!("Reply from {}: bytes={} time={}ms TTL={}", reply.address, data.len(), reply.rtt, options.ttl)
                    }
                    Err(e) => println!("Ping to {} failed: {:?}", domain, e),
                }
            }
            2 => println!("Checking internet speed..."),
            3 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}
