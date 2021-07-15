use std::process;
use std::thread::sleep;
// use term;
use std::time::Duration;

use clap::{App, Arg};
use duration_str;
use termion::cursor;

mod type_face;
mod count_down;

fn main() {
    let mut font_size: usize = 7;
    let mut duration: Duration = Duration::from_secs(24 * 60);

    let matches = App::new("timer")
        .version("1.0")
        .about("A simple Pomodoro timer.")
        .arg(
            Arg::with_name("font")
                .short("f")
                .long("font")
                .value_name("SIZE")
                .help("Sets the font size (5, 6, 7, 20)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .value_name("DURATION")
                .help("Sets the duration of the timer (2m, 15m+30, ...)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .value_name("COLOR")
                .help("Sets the duration of the timer (blue, cyan, green, red, white)")
                .takes_value(true),
        )
        .get_matches();

    if let Some(o) = matches.value_of("font") {
        font_size = o.parse::<usize>().expect("font size must be integer");
    }

    if let Some(o) = matches.value_of("time") {
        duration = duration_str::parse(o).expect("invalid duration string");
    }

    ctrlc::set_handler(move || {
        print!("{}", cursor::Show);
        process::exit(0x0);
    })
        .expect("Error setting Ctrl-C handler");

    let font_lib = type_face::FontLibrary::new();
    let a_second = Duration::from_secs(1);
    let mut timer = count_down::Timer::new(duration.as_secs());
    timer.start();
    loop {
        type_face::clear();
        type_face::display(timer.to_string().as_str(), font_size, &font_lib);
        if timer.as_secs() == 0 {
            break;
        }
        sleep(a_second);
    }
    print!("{}", cursor::Show);
}
