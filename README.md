# ctrem 
### print colored and styled text


## CLI Examples
```bash
$ctrem "[bold]H[/bold]ello"
```

## Rust Example
```rust
extern crate ctrem;

print("[bold][blink]Hello[/blink], world![/bold]") 
```
prints Hello, world! in bold and blinks Hello
<br><br>
Available style
- default
- bold 
- dim 
- italic 
- underline 
- blink 
- normal 
- reverse 
- strike

Available colors

- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white
- gray

foreground is default , for background use ' `*` '

## Rust Example Colors

```rust
extern crate ctrem;

print("[bold][blink][red]Hello[red][/blink], world[*blue]![/*blue][/bold]") 
```