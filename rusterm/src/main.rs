use dialoguer::{theme::ColorfulTheme, Select, Input};

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

                println!("Pinging {}...", domain);
            },
            2 => println!("Checking internet speed..."),
            3 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}
