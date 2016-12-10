pub mod graphics;

use termion::color;
use termion::color::Fg;
use std::fmt::Display;

pub const SPARKS:&'static str="_▁▁▂▃▄▄▅▆▇██";

pub fn format_cap<T:Display>(content:T, capacity:i32) -> String{
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
