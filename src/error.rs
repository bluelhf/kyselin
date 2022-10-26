use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct LineParseError {
    pub line: String
}

impl Error for LineParseError {}

impl fmt::Display for LineParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse the following as a comment (#: text), question (Q: text), or answer (A: text): '{}'", self.line)
    }
}

#[derive(Debug, Clone)]
pub struct QuestionlessAnswerError {
    pub line_num: u32,
    pub line: String
}

impl Error for QuestionlessAnswerError {

}

impl fmt::Display for QuestionlessAnswerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "answer without preceding question on line {}: '{}'",
                self.line_num, self.line)
    }
}

#[derive(Debug, Clone)]
pub struct DoubleQuestionError {
    pub line_num: u32,
    pub line: String
}

impl Error for DoubleQuestionError {

}

impl fmt::Display for DoubleQuestionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "question right after a question on line {}: '{}'",
                self.line_num, self.line)
    }
}