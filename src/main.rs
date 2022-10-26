#![feature(box_syntax)]
#![feature(if_let_guard)]

use std::convert::Infallible;
use std::error::Error;
use std::{fmt, io};
use std::fs::File;
use std::io::{BufReader, BufRead, stdin, stdout, Write};
use std::path::Path;
use crossterm::cursor::MoveTo;
use crossterm::{execute, event};
use crossterm::style::{Print, SetForegroundColor, Color};
use crossterm::terminal::{Clear, ClearType::All};
use crossterm::style::Color::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use unicode_segmentation::UnicodeSegmentation;
use unwrap_infallible::UnwrapInfallible;
use std::str::FromStr;

use crate::Correctness::*;

#[derive(Debug, Clone)]
struct LineParseError {
    line: String
}

impl Error for LineParseError {}

impl fmt::Display for LineParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse the following as a question (Q: text) or an answer (A: text): '{}'", self.line)
    }
}

#[derive(Debug, Clone)]
struct QuestionlessAnswerError {
    line_num: u32,
    line: String
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
struct DoubleQuestionError {
    line_num: u32,
    line: String
}

impl Error for DoubleQuestionError {

}

impl fmt::Display for DoubleQuestionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "question right after a question on line {}: '{}'",
                self.line_num, self.line)
    }
}

enum Line {
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
struct Question {
    prompt: String
}


#[derive(Debug, Clone)]
struct Answer {
    correct: Vec<String>
}

enum Correctness {
    Correct,
    Incorrect
}

impl FromStr for Question {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Question { prompt: s.to_string() })
    }
}

impl Answer {
    pub fn evaluate(&self, input: String) -> Correctness {
        if self.correct.contains(&input) {
            return Correct;
        }

        return Incorrect;
    }
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


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(Path::new("questions.krs"))?;

    let mut line_num = 1;
    let mut question: Option<Question> = None;
    let mut pairs: Vec<(Question, Answer)> = Vec::new();

    let mut comment = String::new();

    for result in BufReader::new(file).lines() {
        let line = result?;

        match Line::from_str(line.trim())? {
            Line::Comment(text) => {
                comment.push_str(&(text + "\n"));
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

                pairs.push((question.clone().unwrap(), answer));
                question = None;
            }
        }

        line_num += 1;
    }

    pairs.shuffle(&mut thread_rng());

    execute!(stdout(),
        Clear(All),
        MoveTo(0, 0),
        SetForegroundColor(Yellow),
        Print(comment + "\n"),
        SetForegroundColor(Reset)
    )?;

    await_key()?;

    let mut score = 0;
    loop {
        for pair in pairs.iter_mut() {
            execute!(
                stdout(),
                Clear(All),
                MoveTo(0, 0),

                SetForegroundColor(DarkGrey),
                Print("Press Ctrl + C to exit\n"),
                Print(format!("Score: {}", score)),
                SetForegroundColor(Reset),
                Print(" | "),
                SetForegroundColor(Yellow),
                Print(&pair.0.prompt),
                Print("\n> "),
                SetForegroundColor(Reset)
            )?;

            stdout().flush()?;

            let mut input = String::new();
            stdin().read_line(&mut input)?;

            let correct_fun = || -> Result<i64, io::Error> {
                execute!(
                    stdout(),
                    SetForegroundColor(Green),
                    Print("Correct!\n"),
                    SetForegroundColor(Reset)
                )?;
                Ok(1)
            };

            let incorrect_fun = || -> Result<i64, io::Error> {
                execute!(
                    stdout(),
                    SetForegroundColor(Red),
                    Print("Incorrect :(\n"),
                    SetForegroundColor(Reset)
                )?;
                Ok(-1)
            };

            match pair.1.evaluate(input.trim().to_string()) {
                Correct => {
                    score += (correct_fun)()?;
                }
                Incorrect => {
                    let delta = (incorrect_fun)()?;

                    execute!(
                        stdout(),
                        SetForegroundColor(Color::DarkYellow),
                        Print("Override, I am correct (y/N): "),
                    )?;

                    let mut correction = String::new();
                    stdin().read_line(&mut correction)?;

                    execute!(stdout(), SetForegroundColor(Reset))?;

                    if correction.to_ascii_lowercase().starts_with("y") {
                        score += (correct_fun)()?;
                        pair.1.correct.push(input.trim().to_string());
                    } else {
                        let correct = pair.1.correct.clone();

                        // If any answers contain ',', use ';' as the joiner
                        let semicolon = (&correct).into_iter().any(|x| x.contains(","));
                        let joiner = match semicolon {
                            true => "; ", false => ", "
                        };

                        execute!(
                            stdout(),
                            SetForegroundColor(DarkGrey),
                            Print(format!("Correct answer{} {}\n",
                            match correct.len() {
                                1 => " was",
                                _ => "s were"
                            }, correct.join(joiner))),
                            SetForegroundColor(Reset)
                        )?;

                        score += delta;
                    }
                }
            }
            await_key()?;
        }
    }
}

fn await_key() -> Result<(), io::Error> {
    execute!(stdout(),
        SetForegroundColor(DarkGrey),
        Print("Press any key to continue..."),
        SetForegroundColor(Reset)
    )?;
    event::read()?;
    Ok(())
}