use crate::trivia::section::questions::Question;

mod questions;

pub(super) struct Section {
    name: String,
    rules: String,
    questions: Vec<questions::Question>,
}

impl Section {
    /*
    This function creates a new section struct of a trivia game.
     */
    fn new(name: String, rules: String) -> Self {
        let mut questions: Vec<Question> = Vec::new();
        let mut section: Section = Section {
            name,
            rules,
            questions,
        };
        section
    }
}
