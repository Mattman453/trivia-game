use crate::trivia::section::Section;

mod section;

pub(super) struct Trivia {
    title: String,
    sections: Vec<Section>,
}

impl Trivia {
    fn new(title: String) -> Self {
        let mut sections: Vec<Section> = Vec::new();
        let mut trivia = Trivia{title, sections};
        trivia
    }
}