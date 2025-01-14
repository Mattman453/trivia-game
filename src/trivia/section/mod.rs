mod questions;

pub(super) struct Section {
    name: String,
    rules: String,
    questions: Vec<questions::Question>,
}
