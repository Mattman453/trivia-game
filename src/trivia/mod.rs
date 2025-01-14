use crate::trivia::section::Section;

mod section;

/*
This struct is the head of a trivia package. It has 2 components: title and sections. The title
component is a String meant to act as the overall name of the trivia package. The sections component
is a Vector of Section structs where each Section is about a singular topic and contains the
questions related to that topic.
 */
pub(super) struct Trivia {
    title: String,
    sections: Vec<Section>,
}

impl Trivia {
    /*
    Constructor for a Trivia struct. Takes the title of the trivia pack and returns a Trivia struct
    containing an empty vector of sections and the title string.
     */
    fn new(title: String) -> Self {
        let mut sections: Vec<Section> = Vec::new();
        let mut trivia = Trivia { title, sections };
        trivia
    }
}
