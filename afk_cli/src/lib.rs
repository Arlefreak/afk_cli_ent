extern crate termion;
extern crate afk_api;

pub mod utils;
pub mod window;

#[cfg(test)]
mod tests {
    use utils;
    use termion::{clear, color, cursor};
    use afk_api::portfolio::project;
    use std::{time, thread};

    #[test]
    fn sparks() {
        for (i,c) in utils::SPARKS.chars().enumerate() {
            let bar = utils::format_cap(c, (i as i32)*10);
            print!("{bar}", bar = &bar);
        }
    }

    #[test]
    fn test_colors() {
        for cap in 1..20{
            let capacity = cap * 5;
            let bar = utils::SPARKS.chars().nth(10).unwrap_or('x');
            let bar = utils::format_cap(bar, capacity);
            print!("{bar}", bar = &bar);
        }
    }

    #[test]
    fn test_projects(){
        let projects: Vec<project::Project> = project::get_projects();
        println!("\n{}{}{}{}{}{}", cursor::Hide, clear::All, cursor::Goto(1, 1), color::Fg(color::White), color::Bg(color::Black), projects[0].slug);
        println!("\n{}{}{}", cursor::Hide, clear::All, color::Bg(color::Black));
        for (index, project) in projects.iter().enumerate() {
            println!("\n{}{}{}{}{}.- {}", cursor::Hide, cursor::Goto(1, (index as u16)+1), color::Fg(color::White), color::Bg(color::Black), index,project.slug);
        }
        println!("{}{}{}           Arlefreak.com           ", cursor::Goto(1, 1), color::Fg(color::Black),color::Bg(color::White));
        println!("{}{}{}           Projects                ", cursor::Goto(1, 28), color::Fg(color::Black),color::Bg(color::White));
        thread::sleep(time::Duration::from_millis(900));
    }
}
