use std::io::{stdout, Read, Write};
use termion::{color, cursor};
use utils::graphics;

pub struct Window <R, W>{
    pub title: String,
    pub width: u16,
    pub height: u16,
    pub stdin: R,
    pub stdout: W,
}

impl<R: Read, W: Write> Window<R, W>{
    pub fn draw(&mut self){
        let width: u16 = self.width as u16;
        let height: u16 = self.height as u16;

        write!(self.stdout, "{}", color::Fg(color::Red)).unwrap();

        write!(self.stdout, "{}{}", cursor::Goto(1, 1), graphics::TOP_LEFT_CORNER).unwrap();
        write!(self.stdout, "{}", cursor::Goto(2, 1)).unwrap();
        self.draw_horizontal_line(graphics::HORIZONTAL_WALL, width - 2);
        write!(self.stdout, "{}{}", cursor::Goto(width, 1), graphics::TOP_RIGHT_CORNER).unwrap();

        for y in 1..height {
            write!(self.stdout, "{}{}", cursor::Goto(1, y as u16 + 1), graphics::VERTICAL_WALL).unwrap();
            write!(self.stdout, "{}{}", cursor::Goto(self.width as u16, y as u16 + 1), graphics::VERTICAL_WALL).unwrap();
        }

        write!(self.stdout, "{}{}", cursor::Goto(1, height), graphics::BOTTOM_LEFT_CORNER).unwrap();
        write!(self.stdout, "{}", cursor::Goto(2, height)).unwrap();
        self.draw_horizontal_line(graphics::HORIZONTAL_WALL, width - 2);
        write!(self.stdout, "{}{}", cursor::Goto(width, height), graphics::BOTTOM_RIGHT_CORNER).unwrap();

        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();
    }

    fn draw_horizontal_line(&mut self, chr: &str, width: u16) {
        for _ in 0..width { self.stdout.write(chr.as_bytes()).unwrap(); }
    }
}
