use rust_embed::RustEmbed;
use ansi_term::Colour::Red;

#[derive(RustEmbed)]
#[folder = "fonts/"]
struct Asset;

const BLOCK: &str  = "█";

pub fn show() {
    let a53 =  Asset::get("5/a53.txt").unwrap();
    let line = std::str::from_utf8(a53.as_ref()).unwrap().chars();

    for c in line {
        match c {
            '#' => print!("{}",Red.paint(BLOCK)),
            '.' => print!(" "),
            _default => println!(),
        }
    }
}

#[test]
fn test_show() {
    show();
}
