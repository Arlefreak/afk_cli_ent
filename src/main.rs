extern crate termion;
extern crate clap;
extern crate afk_api;

use clap::{Arg, App};
use termion::{clear, color, cursor};
use termion::color::Fg;
use termion::input::TermRead;
use std::fmt::Display;
use std::io::{Write, stdout, stdin};
use std::{time, thread};

const SPARKS:&'static str="_▁▁▂▃▄▄▅▆▇██";

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

    // test_afk();
    test_colors();
    println!("");
    test();
    println!("");
    // print_projects();
    // read_passwd();
}

// fn test_afk(){
//     afk::afk();
//     println!("{:?}", afk::about::get_entry(2).slug);
//     println!("{:?}", afk::about::get_entries()[0].slug);
//     println!("{:?}", afk::diary::get_post(7).slug);
//     println!("{:?}", afk::diary::get_posts()[0].slug);
//     println!("{:?}", afk::portfolio::get_project(2).slug);
//     println!("{:?}", afk::portfolio::get_projects()[0].slug);
//     println!("{:?}", afk::portfolio::get_project_category(1).slug);
//     println!("{:?}", afk::portfolio::get_project_categories()[0].slug);
// }

fn format_cap<T:Display>(content:T, capacity:i32) -> String{
    match capacity{
        0...5    => format!("{1}{0}", content, Fg(color::Red)),
        5...10   => format!("{1}{0}", content, Fg(color::Red)),
        10...20  |
            20...30  => format!("{1}{0}", content, Fg(color::LightRed)),
        30...40  => format!("{1}{0}", content, Fg(color::Yellow)),
        40...55  => format!("{1}{0}", content, Fg(color::LightYellow)),
        45...65  => format!("{1}{0}", content, Fg(color::Green)),
        65...100 => format!("{1}{0}", content, Fg(color::LightGreen)),
        _        => format!("{1}{0}", content, Fg(color::Black))
    }
}

fn test(){
    for (i,c) in SPARKS.chars().enumerate() {
        let bar = format_cap(c, (i as i32)*10);
        print!("{bar}", bar = &bar);
    }
}

fn test_colors() {
    for cap in 1..20{
        let capacity = cap * 5;
        let bar = SPARKS.chars().nth(10).unwrap_or('x');
        let bar = format_cap(bar, capacity);
        print!("{bar}", bar = &bar);
    }
}

// fn read_passwd(){
//     let stdout = stdout();
//     let mut stdout = stdout.lock();
//     let stdin = stdin();
//     let mut stdin = stdin.lock();

//     stdout.write(b"password: ").unwrap();
//     stdout.flush().unwrap();

//     let pass = stdin.read_passwd(&mut stdout);
//     print!("hola");

//     if let Ok(Some(pass)) = pass {
//         stdout.write(pass.as_bytes()).unwrap();
//         stdout.write(b"\n").unwrap();
//     } else {
//         stdout.write(b"Error\n").unwrap();
//     }
// }

// fn print_projects(){
//     loop{
//         let projects: Vec<afk::portfolio::Project> = afk::portfolio::get_projects();
//         println!("\n{}{}{}{}{}{}", cursor::Hide, clear::All, cursor::Goto(1, 1), color::Fg(color::White), color::Bg(color::Black), projects[0].slug);
//         println!("\n{}{}{}", cursor::Hide, clear::All, color::Bg(color::Black));
//         for (index, project) in projects.iter().enumerate() {
//             println!("\n{}{}{}{}{}.- {}", cursor::Hide, cursor::Goto(1, (index as u16)+1), color::Fg(color::White), color::Bg(color::Black), index,project.slug);
//         }
//         println!("{}{}{}           Arlefreak.com           ", cursor::Goto(1, 1), color::Fg(color::Black),color::Bg(color::White));
//         println!("{}{}{}           Projects                ", cursor::Goto(1, 28), color::Fg(color::Black),color::Bg(color::White));
//         thread::sleep(time::Duration::from_millis(900));
//     }
// }
