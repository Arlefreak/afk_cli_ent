extern crate clap;
extern crate termion;
extern crate afk_cli;

use clap::{Arg, App};
use afk_cli::window;
use std::io::{stdout, Write};
use termion::{async_stdin, clear, cursor};
use termion::raw::IntoRawMode;

fn main(){
    let matches = App::new("afk-ent")
        .about("A cli-ent for arlefreak.com")
        .version("0.1.0")
        .author("Marioc hi@arlefreak.com")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("setup")
             .short("S")
             .long("setup")
             .takes_value(false)
             .help("Get api Token"))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
    init(80,30);
}

fn init(width: u16, height: u16) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin();

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    let mut window = window::Window {
        title: "arlefreak".to_string(),
        width: width,
        height: height,
        stdin: stdin,
        stdout: stdout,
    };

    window.draw();
}
