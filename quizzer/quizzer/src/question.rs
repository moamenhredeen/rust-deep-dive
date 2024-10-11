use serde::{Deserialize, Serialize};

/// Question is a struct that represents a question
#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    /// content is the question
    pub content: String,

    /// correct_answer is the correct answer
    pub correct_answer: String,

    /// incorrect_answers is a vector of incorrect answers
    pub incorrect_answers: Vec<String>,
}

mod test {
    use super::*;

    #[test]
    fn create_question() {
        let question = Question {
            content: String::from("how old are you?"),
            correct_answer: String::from("I am 18 years old"),
            incorrect_answers: vec![
                String::from("I am 17 years old"),
                String::from("I am 19 years old"),
                String::from("I am 20 years old"),
            ],
        };

        assert_eq!(question.content, "how old are you?");
        assert_eq!(question.correct_answer, "I am 18 years old");
        assert_eq!(question.incorrect_answers.len(), 3);
    }
}
