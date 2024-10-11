//! # add
//! add a question to a quiz
//! by asking the users couples of questions

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;

use quizzer::question::Question;

/// start the editor
/// # params
/// `file` - file to write and read from
/// `reader` - reader to read from when reading the user input
/// `writer` - writer to write to when asking the user
#[allow(unused)]
pub fn add<R, W>(file: &mut File, reader: &mut R, writer: &mut W) -> anyhow::Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut question_list = Vec::new();
    let mut lines = reader.lines();

    let mut count = 0;
    loop {
        // read question
        write!(writer, "Enter a question:")?;
        writer.flush()?;
        let question = match lines.next() {
            Some(s) => s,
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "can create a question with empty string",
            )),
        }?;

        // read correct answer
        write!(writer, "Enter the correct answer:")?;
        writer.flush()?;
        let correct_answer = match lines.next() {
            Some(s) => s,
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "can create question without correct answer",
            )),
        }?;

        // read incorrect answers
        let mut incorrect_answers: Vec<String> = Vec::new();
        for i in 1..=3 {
            // read answer
            write!(writer, "Enter incorrect answer {}:", i)?;
            writer.flush()?;

            let line = match lines.next() {
                Some(s) => s,
                None => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "can create a answer with empty string",
                )),
            }?;
            incorrect_answers.push(line);
        }

        let question = Question {
            content: question,
            correct_answer,
            incorrect_answers,
        };
        count += 1;

        question_list.push(question);

        writeln!(writer, "Question added successfully.")?;
        writer.flush()?;
        writeln!(writer, "Total questions added {}.", count)?;
        writer.flush()?;

        write!(writer, "Continue ? [Y/n]:")?;
        writer.flush()?;

        if let Some(Ok(yn)) = lines.next() {
            if yn.trim().to_lowercase() == "n" {
                let stringified = serde_json::to_string(&question_list)?;
                file.write_all(stringified.as_bytes());
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn create_empty_question() {
        // create the in and out used for the test
        // note: cursor of string can not be used as writer because
        // the use of string restrict the input to a valid unicode points
        // it's like a subsect of a vector of bytes
        let mut reader_buf = String::from("");
        let mut reader = Cursor::new(&mut reader_buf);
        let mut writer_buf: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(&mut writer_buf);

        let res = add(&mut reader, &mut writer);

        assert!(res.is_err());
    }

    #[test]
    fn create_question_with_no_answers() {
        let mut reader_buf = String::from("how old are you?\n");
        let mut reader = Cursor::new(&mut reader_buf);
        let mut writer_buf: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(&mut writer_buf);

        let res = add(&mut reader, &mut writer);

        assert!(res.is_err());
    }

    #[test]
    fn create_question_with_answers_less_than_4() {
        let mut reader_buf = String::from(
            "\
            how old are you?\n\
            first answer\n\
            second answer\n",
        );
        let mut reader = Cursor::new(&mut reader_buf);
        let mut writer_buf: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(&mut writer_buf);

        let res = add(&mut reader, &mut writer);

        assert!(res.is_err());
    }

    #[test]
    fn create_question_with_4_answers() {
        let mut reader_buf = String::from(
            "\
            how old are you?\n\
            first answer\n\
            second answer\n\
            third answer\n\
            fourth answer\n\
            n\n",
        );
        let mut reader = Cursor::new(&mut reader_buf);
        let mut writer_buf: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(&mut writer_buf);

        let res = add(&mut reader, &mut writer);

        assert!(res.is_ok());
        assert_eq!(
            writer_buf,
            b"\
            Enter a question:\
            Enter the correct answer:\
            Enter incorrect answer 1:\
            Enter incorrect answer 2:\
            Enter incorrect answer 3:\
            Question added successfully.\n\
            Total questions added 1.\n\
            Continue ? [Y/n]:\
            "
        );
    }

    /// .
    /// aösldkf jasödlkf asödlkf
    /// asdölf
    /// asöldkf
    /// aösdlk
    /// faöskdlföalk
    ///
    /// # Panics
    ///
    ///adsöflk asdölfka jsdöflkj
    /// Panics if .
    #[test]
    fn create_multiple_questions_with_4_answers() {
        let mut reader_buf: String = String::from(
            "\
            how old are you?\n\
            first answer\n\
            second answer\n\
            third answer\n\
            fourth answer\n\
            y\n\
            how old are you?\n\
            first answer\n\
            second answer\n\
            third answer\n\
            fourth answer\n\
            n\n\
            ",
        );
        let mut reader = Cursor::new(&mut reader_buf);
        let mut writer_buf: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(&mut writer_buf);

        let res = add(&mut reader, &mut writer);

        println!(
            "writer buf: {}",
            String::from_utf8(writer_buf.clone()).unwrap()
        );
        assert!(res.is_ok());
        assert_eq!(
            writer_buf,
            b"\
            Enter a question:\
            Enter the correct answer:\
            Enter incorrect answer 1:\
            Enter incorrect answer 2:\
            Enter incorrect answer 3:\
            Question added successfully.\n\
            Total questions added 1.\n\
            Continue ? [Y/n]:\
            Enter a question:\
            Enter the correct answer:\
            Enter incorrect answer 1:\
            Enter incorrect answer 2:\
            Enter incorrect answer 3:\
            Question added successfully.\n\
            Total questions added 2.\n\
            Continue ? [Y/n]:\
            "
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
