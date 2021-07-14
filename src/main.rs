// use term;
use std::time::Duration;
use std::thread::sleep;
use termion::{cursor};
mod type_face;
mod count_down;

fn main() {
    let font_lib = type_face::FontLibrary::new();
    let a_second = Duration::from_secs(1);
    let mut timer = count_down::Timer::new(120);
    timer.start();
    loop {
        type_face::clear();
        type_face::display(timer.to_string().as_str(), 5, &font_lib);
        if timer.as_secs() == 0 {
            break;
        }
        sleep(a_second);
    }
    print!("{}", cursor::Show);
}
