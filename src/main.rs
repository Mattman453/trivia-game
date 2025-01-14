/*
Entry point of the program. Sets up managers and overarching design.
 */
use crate::trivia::Trivia;
use crate::trivia::section::Section;

pub mod trivia;

fn main() {
    let mut trivia = Trivia::new("Test Pack".to_string());
    let mut section = Section::new("Section 1".to_string(), "Their Our Know Rules".to_string());
    trivia.add_section(section);

}
