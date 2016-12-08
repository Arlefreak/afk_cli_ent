extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub mod utils;
pub mod portfolio;
pub mod about;
pub mod diary;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utils() {
        // assert_eq!(4, add_two(2));
        println!("called utlis");
    }

    #[test]
    fn portfolio() {
        // assert_eq!(4, add_two(2));
        println!("called portfolio");
    }

    #[test]
    fn about() {
        // assert_eq!(4, add_two(2));
        println!("called about");
    }

    #[test]
    fn diary() {
        // assert_eq!(4, add_two(2));
        println!("called diary");
    }
}
