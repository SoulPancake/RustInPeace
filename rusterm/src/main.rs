use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&["Say hello", "Ping", "Check internet speed", "Exit"])
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => println!("ฅ^•ﻌ•^ฅ Hello!"),
            1 => println!("Pinging..."),
            2 => println!("Checking internet speed..."),
            3 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
