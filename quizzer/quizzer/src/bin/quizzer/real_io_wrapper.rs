use std::{
    fs::File,
    io::{self, Read, Stdin, Stdout, Write},
};

use quizzer::question::Question;

use crate::io_wrapper::IOWrapper;

#[allow(unused)]
pub struct RealIOWrapper {
    reader: Stdin,
    writer: Stdout,
    file: File,
}

#[allow(unused)]
impl RealIOWrapper {
    /// initialize io wrapper
    /// TODO: maybe accept a file name instead of a file struct
    fn new(file: File) -> Self {
        RealIOWrapper {
            reader: io::stdin(),
            writer: io::stdout(),
            file,
        }
    }
}

#[allow(unused)]
impl IOWrapper for RealIOWrapper {
    fn read_line(&mut self) -> Option<String> {
        let mut buf = String::new();
        _ = self
            .reader
            .read_line(&mut buf)
            .expect("error while reading line from the console");
        if buf.trim().is_empty() {
            None
        } else {
            Some(buf)
        }
    }

    fn write(&mut self, message: String) {
        write!(self.writer, "{}", message).expect("error while writing to stdou");
        self.writer.flush();
    }

    fn write_line(&mut self, message: String) {
        writeln!(self.writer, "{}", message).expect("error while write to stdou");
        self.writer.flush();
    }

    fn read_json_file(&mut self) -> Vec<Question> {
        let mut content = String::new();
        self.file.read_to_string(&mut content);
        let questions: Vec<Question> = serde_json::from_str(content.as_str())
            .expect("error while deserializing the question from the json file");
        questions
    }

    fn write_json_file(&mut self, questions: Vec<Question>) {
        let json = serde_json::to_string(&questions)
            .expect("error while serlializing the qeustions to json");
        self.file
            .write_all(json.as_bytes())
            .expect("error while writing derserialized questions to the json file");
    }
}
