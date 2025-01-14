use std::fs::File;

/*
This struct is the basic structure of a trivia game. Each Question struct contains a question,
answer, audio, and video. The question and answer components contain the actual information related
to a question. The audio component is an optional File that will play audio for questions that rely
on audio for their answer. The video component is an optional File that will play a video for
questions that require video.
 */
pub(super) struct Question {
    question: String,
    answer: String,
    audio: Option<File>,
    video: Option<File>,
}

impl Question {
    /*
    This function creates a new question with only a question and answer and thus without audio or
    video. Should these need to be added, after creating the question, call add_video or add_audio
    to add them to the question.
     */
    fn new(question: String, answer: String) -> Self {
        let mut question = Question {
            question,
            answer,
            audio: None,
            video: None,
        };
        question
    }

    /*
    Adds audio to a specific question. Takes a File struct as the audio. Supported formats are:
    - MP3 (.mp3)
    - FLAC (.flac)
    - Vorbis (.ogg)
    - WAV (.wav)
     */
    fn add_audio(&mut self, audio: File) {
        self.audio = Some(audio);
    }

    /*
    This function plays the audio connected to a question. If no audio is attached, a message will
    be printed to the console.
     */
    fn play_audio(&mut self) {
        if self.audio.is_none() {
            // TODO: Log failed uses instead of printing to the console
            println!("No audio attached to question.");
            return;
        }

        // TODO: Add implementation for playing audio files with rodio
    }

    /*
    Adds a video file to a question. Video backend not yet implemented or decided upon. Set to take
    a File struct.
     */
    fn add_video(&mut self, video: File) {
        self.video = Some(video);
    }

    /*
    This function plays the video connected to a question. If no video is attached, a message will
    be printed to the console.
     */
    fn play_video(&mut self) {
        if self.video.is_none() {
            // TODO: Update section to print a log file instead of to console
            println!("No video attached to question.");
            return;
        }

        // TODO: Add implementation for playing video files in fltk, in separate window, or different backend
    }
}
