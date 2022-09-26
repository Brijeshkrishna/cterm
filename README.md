# Ctrem 
#### print colored and styled text


## CLI Examples
```bash
$ctrem "[bold]H[/]ello"
```

## Rust Example
```rust
extern crate ctrem;

print("[bold][blink]Hello[/], world![/]") 
```
prints Hello, world! in bold and blinks Hello
<br><br>
Available style
- Default
- Bold 
- Dim 
- Italic 
- Underline 
- Blink 
- Normal 
- Reverse 
- Strike

Available colors

- Black
- Red
- Green
- Yellow
- Blue
- Magenta
- Cyan
- White

Supported 8bit (256) also 24 bit (16.7 million) colors

Foreground is default , for background use ' `*` '


Cterm supported different styles and colors
styles or colors palette must be inclosed in [palette] and ends with [/]

 - For bold  text use `[bold]some text[/]`

 - For underline  text use `[underline] some text[/]`

 - For RGB colored text use `[rgb 25,55,55]some text[/]` or `[#b366b1]some text[/]`

 - For 4bit colors use `[color 4]some text[/]`

_command palette must end , use `[//]` to end all_

Foreground is default , for background use ' `*` '

## Rust Example Colors

```rust
use ctrem::*;
fn main(){
    cprintln("[bold][blink][red]Hello[//],[green]world[*blue]![//]"); 
}
```