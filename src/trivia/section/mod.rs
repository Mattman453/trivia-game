use crate::trivia::section::questions::Question;

pub mod questions;

/*
This struct contains information related to a section of a trivia game. It is designed around
containing all questions related to a specific topic such as "Racing" or "Music". The name component
is the title of the section in String format. The rules component is a String that contains the
rules pertaining to a topic. The questions component is a Vector of Questions that are related to a
topic. Information on the Question struct can be found in the "questions" folder.
 */
pub struct Section {
    name: String,
    rules: String,
    questions: Vec<Question>,
}

impl Section {
    /*
    This function creates a new section struct of a trivia game.
     */
    pub(crate) fn new(name: String, rules: String) -> Self {
        let mut questions: Vec<Question> = Vec::new();
        let mut section: Section = Section {
            name,
            rules,
            questions,
        };
        section
    }

    /*
    This function adds a Question struct to the questions vector of a Section, expanding the set.
     */
    fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }
}
