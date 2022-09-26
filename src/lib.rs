use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PALETTE_RE: Regex = Regex::new(r"\[( ?[^\[]+ ?)\]").unwrap();
    static ref DOUBLE_PALETTE_RE: Regex = Regex::new(r"(\x1b\[[\d;]*m)+").unwrap();
    static ref EMPTY_PALETTE_RE: Regex = Regex::new(r"\x1b\[;m(.*\x1b\[;m)").unwrap();
}

fn end_palette(pre: &str, palette_stack: &mut Vec<String>) -> String {
    if pre.eq("//") {
        palette_stack.clear();
        return "\x1b[;m".to_string();
    }
    let mut rv = String::from(";");
    let mut temp;
    for z in palette_stack.iter() {
        temp = parser_args(z);
        rv.push_str(format!("{};", source_palette(temp.0, temp.1).unwrap()).as_str());
    }
    rv.pop();
    format!("\x1b[;m\x1b[{rv}m")
}

fn source_palette(code_text: String, args: Vec<String>) -> Option<String> {
    match code_text.as_str() {
        //styles
        "default" => Some("0".to_string()),
        "bold" => Some("1".to_string()),
        "dim" => Some("2".to_string()),
        "italic" => Some("3".to_string()),
        "underline" => Some("4".to_string()),
        "blink" => Some("5".to_string()),
        "invert" => Some("7".to_string()),
        "hide" => Some("8".to_string()),
        "strike" => Some("9".to_string()),
        //backgound

        // 4bit (8) colour
        "black" => Some("30".to_string()),
        "red" => Some("31".to_string()),
        "green" => Some("32".to_string()),
        "yellow" => Some("33".to_string()),
        "blue" => Some("34".to_string()),
        "magenta" => Some("35".to_string()),
        "cyan" => Some("36".to_string()),
        "white" => Some("37".to_string()),

        "*black" => Some("40".to_string()),
        "*red" => Some("41".to_string()),
        "*green" => Some("42".to_string()),
        "*yellow" => Some("43".to_string()),
        "*blue" => Some("44".to_string()),
        "*magenta" => Some("45".to_string()),
        "*cyan" => Some("46".to_string()),
        "*white" => Some("47".to_string()),

        // 8bit(256) color
        "color" | "*color" => {
            let x = args
                .first()
                .expect("`color palette` Invalid number of arguments (requied 1)")
                .parse::<u8>()
                .expect("Invalid color code (0-255)");
            if code_text.starts_with('*') {
                return Some(format!("48;5;{x}"));
            }
            Some(format!("38;5;{x}"))
        }
        //24bit (16777216) color
        "rgb" | "*rgb" => {
            let get_args_rgb = |x: usize| {
                args.get(x)
                    .expect("`rgb palette` Invalid number of arguments (requied 3)")
                    .parse::<u8>()
                    .expect("Invalid color code (0-255)")
            };
            if code_text.starts_with('*') {
                return Some(format!(
                    "38;2;{};{};{}",
                    get_args_rgb(0),
                    get_args_rgb(1),
                    get_args_rgb(2)
                ));
            }
            Some(format!(
                "48;2;{};{};{}",
                get_args_rgb(0),
                get_args_rgb(1),
                get_args_rgb(2)
            ))
        }
        _ => None,
    }
}

fn parser_args(palette: &str) -> (String, Vec<String>) {
    let mut args = palette
        .split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    let q = args[0]
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let fnr = q[0].to_string();
    match fnr.starts_with('#') | fnr.starts_with("*#") {
        true => {
            if fnr.len() >= 9 {
                panic!("Invalid HEX color (0-F)");
            }
            args.clear();
            let hex_u8 = |x| u8::from_str_radix(x, 16).expect("Invalid HEX color (0-F)");

            if fnr.starts_with("*#") {
                args.push(hex_u8(&fnr[2..4]).to_string());
                args.push(hex_u8(&fnr[4..6]).to_string());
                args.push(hex_u8(&fnr[6..8]).to_string());
                return ("*rgb".to_string(), args);
            }
            args.push(hex_u8(&fnr[1..3]).to_string());
            args.push(hex_u8(&fnr[3..5]).to_string());
            args.push(hex_u8(&fnr[5..7]).to_string());
            ("rgb".to_string(), args)
        }

        false => {
            match q.get(1) {
                Some(r) => {
                    args.remove(0);
                    args.insert(0, r.to_string());
                }
                None => {
                    args.clear();
                }
            }
            (fnr, args)
        }
    }
}

fn start_palette(pre: &str) -> Option<String> {
    let (fnr, args) = parser_args(pre);
    if let Some(z) = source_palette(fnr, args) {
        return Some(format!("\u{1b}[{z}m"));
    }
    None
}

fn palette_cleaner(val: &str) -> String {
    let mut rv = val.to_owned();

    for i in DOUBLE_PALETTE_RE.captures_iter(val) {
        for j in EMPTY_PALETTE_RE.captures_iter(&i[0]) {
            rv = rv.replace(&j[1], "");
        }
        rv = rv.replace(&i[0], &i[0].replace("m\x1b[", ";"));
    }
    rv.replace(";;", "")
}
/// format the palette
/// returns the string of format styled and color
macro_rules! cformat {
    ($($arg:tt)*) => {
        crate::lib::cformat(format!($($arg)*).as_str())
    };
}

macro_rules! cprint {
    ($($arg:tt)*) => {{
        print!("{}",cformat!($($arg)*));
    }};
}

macro_rules! cprintln {

    ($($arg:tt)*) => {{
        cprint!($($arg)*);
        print!("\n");
    }};
}

/// # cformat
/// retunrs colored and styled text
///
/// cterm supported different styles and colors
/// styles or colors palette must be inclosed in [palette] and ends with [/]
///
/// For bold  text use `[bold]some text[/]`
///
/// For underline  text use `[underline] some text[/]`
///
/// For RGB colored text use `[rgb 25,55,55]some text[/]` or `[#b366b1]some text[/]`
///
/// For 4bit colors use `[color 4]some text[/]`
///
/// _command palette must end , use `[//]` to end all_

/// ## Examples
/// ```rust
/// use cterm::cformat;
/// let rv:String = cformat("[bold][blink]Hello[/], world![/]");
/// ```
/// return Hello, world! in bold and blinks Hello (encoded)
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
/// Available colors (4bit colors)
///
/// - black
/// - red
/// - green
/// - yellow
/// - blue
/// - magenta
/// - cyan
/// - white
/// - gray
///
/// Supported 8bit (256) also 24 bit (16.7 million) colors
///
/// Foreground is default , for background use ' `*` '
///
/// _Use cprintln or cprint to print_
///
pub fn cformat(val: &str) -> String {
    let mut rv_palette: String = val.to_owned();
    let mut palette_stack: Vec<String> = vec![];
    let mut temp;

    for y in PALETTE_RE.captures_iter(val) {
        temp = y[1].to_string();
        match temp.starts_with('/') {
            true => match palette_stack.pop() {
                Some(_) => {
                    let r = end_palette(temp.as_str(), &mut palette_stack);
                    rv_palette = rv_palette.replace(format!("[{}]", &y[1]).as_str(), r.as_str());
                }
                None => {
                    if temp.eq("//") {
                        rv_palette = rv_palette.replace(format!("[{}]", &y[1]).as_str(), "\x1b[;m");
                    } else {
                        panic!("Invalid `Palette`, Nothing to end ( why [/] ?)");
                    }
                }
            },
            false => {
                let r = start_palette(&temp);
                if let Some(palette) = r {
                    palette_stack.push(temp.to_string());
                    rv_palette =
                        rv_palette.replace(format!("[{}]", &y[1]).as_str(), palette.as_str());
                }
            }
        }
    }

    if !palette_stack.is_empty() {
        panic!("`{palette_stack:?}` Palette have not ended ( use `[//]` to end all palette or use `[/]` to end the previous palette )");
    }
    palette_cleaner(rv_palette.as_str())
}

/// # cprint
/// print the colored and styled text
/// ```rust
/// use cterm::cprint;
/// ```
/// ## Examples
/// ```rust
/// cprint("[bold][blink][red]Hello[red][/], world[/]![//]");
/// ```
/// ```rust
/// cprint("[bold][blink][#b366b1]Hello again[//]");
/// ```
pub fn cprint(val: &str) {
    print!("{}", cformat(val));
}

/// # cprintln
/// print the colored and styled text with newline
/// ```rust
/// use cterm::cprintln;
/// ```
/// ## Examples
/// ```rust
/// cprintln("[bold][blink][red]Hello[red][/blink], world[*blue]![/*blue][/bold]");
/// ```
/// ```rust
/// cprintln!("[bold][blink][#b366b1]Hello again[//]");
/// ```
pub fn cprintln(val: &str) {
    println!("{}", cformat(val));
}
