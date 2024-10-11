use quizzer::question::Question;

/// IOWrapper is a trait that defines the interface for the input and output
#[allow(unused)]
pub trait IOWrapper {
    /// read a line from the input
    fn read_line(&mut self) -> Option<String>;

    /// write a line to the output
    fn write(&mut self, message: String);

    /// write a line to the output
    fn write_line(&mut self, message: String);

    /// write a line to the output
    fn read_json_file(&mut self) -> Vec<Question>;

    /// write a line to the output
    fn write_json_file(&mut self, questions: Vec<Question>);
}
