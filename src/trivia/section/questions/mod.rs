use std::fs::File;

pub(super) struct Question {
    question: String,
    answer: String,
    audio: File,
    video: File,
}