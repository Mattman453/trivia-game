use std::fs::File;

pub(in super) struct Question {

    question: String,
    answer: String,
    audio: Option<File>,
    video: Option<File>,
}

impl Question {

    fn new(question: String, answer: String) -> Self {

        let mut question = Question {question, answer, audio: None, video: None};
        question
    }

    fn add_audio(&mut self, audio: File) {

        self.audio = Some(audio);
    }

    // TODO: Log failed uses instead of printing to the console
    /*
    This function plays the audio connected to a question. If no audio is attached, a message will
    be printed to the console.
     */
    fn play_audio(&mut self) {

        if self.audio.is_none() {

            println!("No audio attached to question.");
            return;
        }

        // TODO: Add implementation for playing audio files with rodio
    }

    fn add_video(&mut self, video: File) {

        self.video = Some(video);
    }


    fn play_video(&mut self) {
        if self.video.is_none() {

            println!("No video attached to question.");
            return;
        }

        // TODO: Add implementation for playing video files in fltk, in separate window, or different backend
    }
}
