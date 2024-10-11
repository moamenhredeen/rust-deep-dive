use std::io::{BufRead, Cursor, Lines, Read, Write};

use quizzer::question::Question;

use crate::io_wrapper::IOWrapper;

/// FakeIOWrapper is a fake implementation of the IOWrapper trait
#[allow(unused)]
pub struct FakeIOWrapper {
    reader: Lines<Cursor<String>>,
    writer: Cursor<Vec<u8>>,
    file: Cursor<Vec<u8>>,
}

#[allow(unused)]
impl FakeIOWrapper {
    fn new(input: &str, file: &str) -> Self {
        FakeIOWrapper {
            reader: Cursor::new(input.to_string()).lines(),
            writer: Cursor::new(Vec::new()),
            file: Cursor::new(file.as_bytes().to_vec()),
        }
    }

    fn form_input(input: &str) -> Self {
        FakeIOWrapper {
            reader: Cursor::new(input.to_string()).lines(),
            writer: Cursor::new(Vec::new()),
            file: Cursor::new(Vec::new()),
        }
    }

    fn from_file(file: &str) -> Self {
        FakeIOWrapper {
            reader: Cursor::new(String::new()).lines(),
            writer: Cursor::new(Vec::new()),
            file: Cursor::new(file.as_bytes().to_vec()),
        }
    }

    fn file_buf(&mut self) -> String {
        let mut buf = String::new();
        self.file.read_to_string(&mut buf);
        buf
    }

    fn writer_buf(&mut self) -> String {
        let mut buf = String::new();
        self.writer.read_to_string(&mut buf);
        buf
    }
}

impl Default for FakeIOWrapper {
    fn default() -> Self {
        Self {
            reader: Cursor::new(String::new()).lines(),
            writer: Default::default(),
            file: Default::default(),
        }
    }
}

#[allow(unused)]
impl IOWrapper for FakeIOWrapper {
    fn read_line(&mut self) -> Option<String> {
        match self.reader.next() {
            Some(res) => {
                let line = res.expect("error occured while reading from reader");
                Some(line)
            }
            None => None,
        }
    }

    fn write(&mut self, message: String) {
        write!(self.writer, "{}", message).expect("error occured while writing to writer buffer");
    }

    fn write_line(&mut self, message: String) {
        writeln!(self.writer, "{}", message).expect("error occured while writing to writer buffer");
    }

    fn read_json_file(&mut self) -> Vec<Question> {
        let mut content = String::new();
        self.file
            .read_to_string(&mut content)
            .expect("error occured while reading from the json file");
        let json: Vec<Question> = serde_json::from_str(content.as_str())
            .expect("error occured while trying to deserialize json file");
        json
    }

    fn write_json_file(&mut self, questions: Vec<Question>) {
        let stringified = serde_json::to_string(&questions)
            .expect("error occured while serializing questions to json");
        write!(self.file, "{}", stringified);
    }
}

#[cfg(test)]
mod test {
    use quizzer::question::Question;
    use serde_json::json;

    use crate::io_wrapper::IOWrapper;

    use super::FakeIOWrapper;

    #[test]
    fn read_user_input() {
        let mut wrapper = FakeIOWrapper::form_input("hello world");
        assert_eq!(wrapper.read_line(), Some("hello world".to_string()));
    }

    #[test]
    fn ask_user_question() {
        let mut wrapper = FakeIOWrapper::default();
        wrapper.write_line("hello world".to_string());
        assert_eq!(wrapper.writer_buf(), "hello world".to_string());
    }

    #[test]
    fn read_json_file() {
        let mut wrapper = FakeIOWrapper::from_file(
            r#"
            [
                {
                    "content":"how old are you?",
                    "correct_answer":"20",
                    "incorrect_answers": [
                        "30",
                        "40",
                        "50"
                    ]
                }
            ]"#,
        );
        wrapper.write_line("hello world".to_string());
        let questions = wrapper.read_json_file();
        assert_eq!(questions.len(), 1);
        assert_eq!(
            questions.get(0).unwrap().content,
            "how old are you?".to_string()
        );
        assert_eq!(
            questions.get(0).unwrap().content,
            "how old are you?".to_string()
        );
        assert_eq!(questions.get(0).unwrap().correct_answer, "20".to_string());
        assert_eq!(questions.get(0).unwrap().incorrect_answers.len(), 3);
        assert_eq!(
            questions.get(0).unwrap().incorrect_answers.get(0).unwrap(),
            &"30".to_string()
        );
        assert_eq!(
            questions.get(0).unwrap().incorrect_answers.get(1).unwrap(),
            &"40".to_string()
        );
        assert_eq!(
            questions.get(0).unwrap().incorrect_answers.get(2).unwrap(),
            &"50".to_string()
        );
    }

    #[test]
    fn write_to_json_file() {
        let mut wrapper = FakeIOWrapper::default();
        wrapper.write_json_file(vec![Question {
            content: "how old are you?".to_string(),
            correct_answer: "20".to_string(),
            incorrect_answers: vec!["30".to_string(), "40".to_string(), "50".to_string()],
        }]);
        assert_eq!(
            wrapper.file_buf(),
            r#"
            [
                {
                    "content":"how old are you?",
                    "correct_answer":"20",
                    "incorrect_answers": [
                        "30",
                        "40",
                        "50"
                    ]
                }
            ]"#
            .to_string()
        )
    }
}
