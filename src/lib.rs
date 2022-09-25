extern crate regex;
use regex::Regex;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum SgrStyle {
    Default = 0,
    Bold = 1,
    Dim = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Normal = 6,
    Reverse = 7,
    Strike = 9,
}

#[inline]
fn sgr_code(code_text: &str) -> Option<u8> {
    match code_text {
        "[default]" | "[/default]" => Some(SgrStyle::Default as u8),
        "[bold]" | "[/bold]" => Some(SgrStyle::Bold as u8),
        "[dim]" | "[/dim]" => Some(SgrStyle::Dim as u8),
        "[italic]" | "[/italic]" => Some(SgrStyle::Italic as u8),
        "[underline]" | "[/underline]" => Some(SgrStyle::Underline as u8),
        "[blink]" | "[/blink]" => Some(SgrStyle::Blink as u8),
        "[normal]" | "[/normal]" => Some(SgrStyle::Normal as u8),
        "[reverse]" | "[/reverse]" => Some(SgrStyle::Reverse as u8),
        "[strike]" | "[/strike]" => Some(SgrStyle::Strike as u8),
        "[black]" | "[/black]" => Some(30),
        "[red]" | "[/red]" => Some(31),
        "[green]" | "[/green]" => Some(32),
        "[yellow]" | "[/yellow]" => Some(33),
        "[blue]" | "[/blue]" => Some(34),
        "[magenta]" | "[/magenta]" => Some(35),
        "[cyan]" | "[/cyan]" => Some(36),
        "[white]" | "[/white]" => Some(37),
        "[gray]" | "[/gray]" => Some(90),
        "[Red]" | "[/Red]" => Some(91),
        "[Green]" | "[/Green]" => Some(92),
        "[Yellow]" | "[/Yellow]" => Some(93),
        "[Blue]" | "[/Blue]" => Some(94),
        "[Magenta]" | "[/Magenta]" => Some(95),
        "[Cyan]" | "[/Cyan]" => Some(96),
        "[White]" | "[/White]" => Some(97),
        "[*black]" | "[/*black]" => Some(40),
        "[*red]" | "[/*red]" => Some(41),
        "[*green]" | "[/*green]" => Some(42),
        "[*yellow]" | "[/*yellow]" => Some(43),
        "[*blue]" | "[/*blue]" => Some(44),
        "[*magenta]" | "[/*magenta]" => Some(45),
        "[*cyan]" | "[/*cyan]" => Some(46),
        "[*white]" | "[/*white]" => Some(47),
        "[*gray]" | "[/*gray]" => Some(100),
        "[*Red]" | "[/*Red]" => Some(101),
        "[*Green]" | "[/*Green]" => Some(102),
        "[*Yellow]" | "[/*Yellow]" => Some(103),
        "[*Blue]" | "[/*Blue]" => Some(104),
        "[*Magenta]" | "[/*Magenta]" => Some(105),
        "[*Cyan]" | "[/*Cyan]" => Some(106),
        "[*White]" | "[/*White]" => Some(107),
        _ => None,
    }
}

fn cleaner(val: &String) -> String {
    let ure = Regex::new(r"((\x1b\[[\d;]*m)+)").unwrap();
    let ure1 = Regex::new(r"\x1b\[;m(.*\x1b\[;m)").unwrap();

    let mut rv = val.clone();

    for i in ure.captures_iter(val) {
        for j in ure1.captures_iter(&i[1]) {
            rv = rv.replace(&j[1], "");
        }
        rv = rv.replace(&i[1], &i[1].replace("m\x1b[", ";"));
    }
    rv = rv.replace(";;", "");
    rv
}
/// # ctrem 
/// print colored and styled text

/// ## Examples
/// ```rust
/// print("[bold][blink]Hello[/blink], world![/bold]") 
/// ```
/// prints Hello, world! in bold and blinks Hello
/// <br><br>
/// Available style
/// - default
/// - bold 
/// - dim 
/// - italic 
/// - underline 
/// - blink 
/// - normal 
/// - reverse 
/// - strike
/// 
/// Available colors
/// - black
/// - red
/// - green
/// - yellow
/// - blue
/// - magenta
/// - cyan
/// - white
/// - gray

/// foreground is default , for background use ' `*` '

/// ## Examples

/// ```rust
/// print("[bold][blink][red]Hello[red][/blink], world[*blue]![/*blue][/bold]") 
/// ```
pub fn print(val: &str) {
    let mut rv: String = val.to_string().clone();
    let mut codes_stack: Vec<u8> = vec![];
    let ure = Regex::new(r"(\[ ?*+[^\[]* ?\])").unwrap();

    for y in ure.captures_iter(val) {
        match sgr_code(&y[1].replace(" ", "")) {
            Some(ref z) => {
                if y[1].to_string()[1..2].eq("/") {
                    match codes_stack.pop() {
                        Some(_) => {
                            let mut b = String::new();
                            for z in codes_stack.iter() {
                                b.push_str(format!("{z};").as_str());
                            }
                            b.pop();
                            rv = rv.replace(&y[1], format!("\x1b[;m\x1b[{b}m").as_str());
                        }
                        None => {}
                    }
                } else {
                    rv = rv.replace(&y[1], format!("\x1b[{z}m").as_str());
                    codes_stack.push(*z);
                }
            }
            None => {
                let a = y[1].to_string()[2..3].parse::<u8>();
                if let Ok(ref z) = a {
                    rv = rv.replace(&y[1], format!("\x1b[38;5;{z}m").as_str());
                    codes_stack.push(*z);
                }
            }
        };
    }
    println!("{}", cleaner(&rv))
}
