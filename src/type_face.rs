use ansi_term::Colour::Red;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "fonts/"]
struct Asset;

const BLOCK: &str = "█";

type FontMatrix = Vec<Vec<bool>>;

pub fn create_font_matrix(encoded: &[u8]) -> FontMatrix {
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

fn show(fm: &FontMatrix) {
    for line in fm {
        for c in line {
            if *c {
                print!("{}",BLOCK);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_show() {
        let a53 = super::Asset::get("5/a53.txt").unwrap();
        let content = std::str::from_utf8(a53.as_ref()).unwrap().as_bytes();
        let fm = super::create_font_matrix(content);
        super::show(&fm);
    }

    #[test]
    fn test_font_matrix() {
        let a53 = super::Asset::get("5/a53.txt").unwrap();
        let content = std::str::from_utf8(a53.as_ref()).unwrap().as_bytes();
        println!("{:?}", content);
        let fm = super::create_font_matrix(content);
        println!("{:?}", fm)
    }
}
