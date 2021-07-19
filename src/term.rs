use termion::{color,clear,cursor};

pub fn get_color(name: &str) -> Box<dyn color::Color> {
    match name {
        "blue" => { Box::new(color::Blue) },
        "cyan" => { Box::new(color::Cyan) },
        "green" => { Box::new(color::Green) },
        "magenta" => { Box::new(color::Magenta) },
        "white" => { Box::new(color::White) },
        "yellow" => { Box::new(color::Yellow) },
        _default => { Box::new(color::Red) },
    }
}

pub fn clear() {
    print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);
}
