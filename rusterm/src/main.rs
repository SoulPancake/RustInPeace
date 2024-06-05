use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("You entered: {}", &args[1]);
    } else {
        println!("Please enter a value as a command-line argument.");
    }
}
