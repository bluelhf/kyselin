use std::{str::FromStr, convert::Infallible, fs::File, error::Error, io::{BufReader, BufRead}};
use rand::{thread_rng, prelude::SliceRandom};
use unicode_segmentation::UnicodeSegmentation;
use unwrap_infallible::UnwrapInfallible;

use crate::error::*;

pub enum Line {
    Question(Question),
    Answer(Answer),
    Comment(String),
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.to_string();
        if line.len() < 3 {
            return Err(LineParseError { line });
        }
    
        return match line[..3].to_uppercase().as_str() {
            "#: " => Ok(Line::Comment(line[3..].to_string())),
            "Q: " => Ok(Line::Question(Question::from_str(&line[3..]).unwrap_infallible())),
            "A: " => Ok(Line::Answer(Answer::from_str(&line[3..]).unwrap_infallible())),
            _ => Err(LineParseError { line }),
        };
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    pub prompt: String
}

impl FromStr for Question {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Question { prompt: s.to_string() })
    }
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub correct: Vec<String>
}

const DELIMETER: &'static str = ", ";
impl FromStr for Answer {
    type Err = Infallible;
    
    /// Parses an answer line into a set of valid answers
    /// Answers are split by DELIMETER, unless the DELIMETER is escaped.
    /// To include a literal \DELIMETER, use \\DELIMETER.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut correct: Vec<String> = Vec::new();
        let mut builder = String::new();
        let mut escape = false;

        for grapheme in s.graphemes(true) {
            if grapheme.ends_with(r"\") {
                escape = !escape;

                // We've just ENABLED escaping, this \ shouldn't be included
                if escape {
                    continue;
                }
            }

            if builder.ends_with(DELIMETER) {
                escape = match escape {
                    true => false,
                    false => {
                        let undelimited = &builder[0..builder.len() - DELIMETER.len()];
                        correct.push(undelimited.to_string());
                        builder.clear();
                        false
                    }
                };
            }

            builder.push_str(grapheme);   

        }

        correct.push(builder);
        Ok(Answer { correct })
    }
}


pub struct QuestionFile {
    pub comments: Vec<String>,
    pub entries: Vec<(Question, Answer)>
}

impl QuestionFile {
    pub fn parse(file: &File) -> Result<Self, Box<dyn Error>> {
        let mut line_num = 1;
        let mut question: Option<Question> = None;
        let mut entries: Vec<(Question, Answer)> = Vec::new();
    
        let mut comments = Vec::new();
    
        for result in BufReader::new(file).lines() {
            let line = result?;
    
            match Line::from_str(line.trim())? {
                Line::Comment(text) => {
                    comments.push(text);
                },
                Line::Question(parsed) => {
                    if let Some(_) = question {
                        return Err(box DoubleQuestionError { line, line_num });
                    }

                    question = Some(parsed).clone();
                },
                Line::Answer(answer) => {
                    if let None = question {
                        return Err(box QuestionlessAnswerError { line, line_num })
                    }
    
                    entries.push((question.clone().unwrap(), answer));
                    question = None;
                }
            }
    
            line_num += 1;
        }

        Ok(QuestionFile { comments, entries })
    }

    pub fn comment_string(&self) -> String {
        self.comments.join("\n")
    }

    pub fn shuffle_entries(&mut self) {
        self.entries.shuffle(&mut thread_rng());
    }
}