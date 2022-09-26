#[macro_use]
mod lib;
use std::env;

/// print coloured and styled text from CLI
///
/// ## Examples
/// ```bash
/// ctrem "[bold]H[/]ello"
/// ```
fn main() {
    let input = env::args().nth(1);
    let debug_c = env::args().nth(2);

    if let Some(x) = input {
        if let Some(y) = debug_c {
            if y.eq("-d") {
                println!("{:?}", cformat!("{x}[//]"));
                std::process::exit(0);
            } else {
                println!("Invalid argument");
                std::process::exit(1);
            }
        } else {
            cprintln!("{x}[//]");
            std::process::exit(0);
        }
    } else {
        println!("ctrem : Invalid argument\nExample");
        print!(r#"   $ ctrem "[bold]H[/bold]ello""#);
        println!("");
        std::process::exit(1);
    }
}
