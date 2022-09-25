mod lib;
use lib::*;
use std::env;

/// print coloured and styled text from CLI
/// 
/// ## Examples
/// ```bash
/// ctrem "[bold]H[/bold]ello"
/// ```
fn main() {
    let input = env::args().nth(1);
    let debug_c = env::args().nth(2);

    if let Some(x) = input {
        if let Some(y) = debug_c {
            if y.eq("-d") {
                print(x.as_str(), true);
                std::process::exit(0);
            } else {
                println!("Invalid argument");
                std::process::exit(1);
            }
        } else {
            print(x.as_str(), false);
            std::process::exit(0);
        }
    } else {
        println!("Invalid argument");
        std::process::exit(1);
    }
}