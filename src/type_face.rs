use std::collections::HashMap;

use rust_embed::RustEmbed;
use termion::{color,clear,cursor};

#[derive(RustEmbed)]
#[folder = "fonts/"]
struct Asset;

const BLOCK: &str = "█";

type FontMatrix = Vec<Vec<bool>>;

#[derive(Debug)]
struct FontCharacterID {
    size: usize,
    character: char,
}

pub struct FontLibrary {
    font_map: HashMap<usize, HashMap<char, FontMatrix>>,
}

impl FontLibrary {
    pub fn new() -> FontLibrary {
        let mut font_map: HashMap<usize, _> = HashMap::new();

        for file in Asset::iter() {
            let filename = file.as_ref();
            let fc = get_font_character_id(filename);
            let fm = create_font_matrix(filename);
            if !font_map.contains_key(&fc.size) {
                let fsm = HashMap::<char, FontMatrix>::new();
                font_map.insert(fc.size, fsm);
            }
            let fsm = font_map.get_mut(&fc.size).unwrap();
            fsm.insert(fc.character, fm);
        };
        FontLibrary { font_map }
    }

    fn font_matrix(&self, font_size: usize, character: char) -> &FontMatrix {
        let fsm = self.font_map.get(&font_size).unwrap();
        fsm.get(&character).unwrap()
    }
}

fn get_font_character_id(file: &str) -> FontCharacterID {
    let slash = file.find('/').unwrap();
    let dot = file.find('.').unwrap();

    let font_size: usize = (&file[..slash]).parse().unwrap();
    let char_val = char::from_u32((&file[(slash + 2)..dot]).parse::<u32>().unwrap()).unwrap();

    FontCharacterID { size: font_size, character: char_val }
}

fn create_font_matrix(filename: &str) -> FontMatrix {
    let file = Asset::get(filename).unwrap();
    let encoded = std::str::from_utf8(file.as_ref()).unwrap().as_bytes();
    let mut fm: Vec<Vec<bool>> = Vec::new();
    let mut c: usize = 0;
    while c < encoded.len() {
        let mut line: Vec<bool> = Vec::new();
        while c < encoded.len() && encoded[c] != 10 {
            if encoded[c] == 35 {
                line.push(true);
            } else {
                line.push(false)
            }
            c += 1;
        }
        fm.push(line);
        c += 1;
    }
    fm
}

pub fn display(text: &str, font_size: usize, fl: &FontLibrary) {
    println!("{}", color::Fg(color::Red));
    for l in 0..font_size {
        for c in text.chars() {
            let fm = fl.font_matrix(font_size, c);
            show_line(l, fm);
            print!(" ");
        }
        println!();
    }
}

fn show_line(line: usize, fm: &FontMatrix) {
    let font_line: &Vec<bool> = &fm[line];
    for p in font_line {
        if *p {
            print!("{}", BLOCK);
        } else {
            print!(" ");
        }
    }
}

pub fn clear() {
    // print!("\x1B[2J\x1B[1;1H")
    print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display() {
        let font_library = super::FontLibrary::new();
        super::display("12:34", 5, &font_library);
    }

    #[test]
    fn test_font_matrix() {
        let fm = super::create_font_matrix("5/a53.txt");
        println!("{:?}", fm);
    }

    #[test]
    fn test_font_character() {
        let fc = super::get_font_character_id("5/a46.txt");
        assert_eq!(fc.size, 5);
        assert_eq!(fc.character, '.');
    }

    #[test]
    fn test_load_fonts() {
        let font_library = super::FontLibrary::new();
        let font_size: usize = 5;
        let character = ':';

        let _colon = font_library.font_matrix(font_size, character);
    }
}
