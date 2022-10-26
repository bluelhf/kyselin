#![feature(box_syntax)]
#![feature(if_let_guard)]

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{self, stdin, stdout};

use crossterm::terminal;
use crossterm::{
    event,
    execute, 
    style::*,
    terminal::*,
    cursor::MoveTo, 
    style::Color::*,
    terminal::ClearType::All
};

use parsing::QuestionFile;
use evaluation::Correctness::*;

mod error;
mod parsing;
mod evaluation;


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(Path::new("questions.krs"))?;
    let mut question_file = QuestionFile::parse(&file)?;

    execute!(stdout(),
        Clear(All),
        MoveTo(0, 0),
        SetForegroundColor(Yellow),
        Print(question_file.comment_string() + "\n"),
        SetForegroundColor(Reset)
    )?;
    
    if !question_file.comment_string().is_empty() {
        await_key()?;
    }

    let mut score = 0;
    loop {
        question_file.shuffle_entries();
        for entry in question_file.entries.iter_mut() {

            clear_terminal()?;
            execute!(
                stdout(),
                SetForegroundColor(DarkGrey),
                Print("Press Ctrl + C to exit\n"),
                Print(format!("Score: {}", score)),
                ResetColor,
                Print(" | "),
                SetForegroundColor(Yellow),
                Print(&entry.0.prompt),
                Print("\n> "),
                ResetColor
            )?;

            match entry.1.evaluate(input()?) {
                Correct => {
                    score += on_correct()?;
                }
                Incorrect => {
                    // say it was incorrect, but don't actually change the score yet
                    // (they might override it)
                    let delta = on_incorrect()?;
                    let correct = &entry.1.correct;

                    execute!(
                        stdout(),
                        SetForegroundColor(DarkGrey),
                        Print(format!("Correct answer{} {}\n",
                        match correct.len() {
                            1 => " was",
                            _ => "s were"
                        }, correct.join(readable_joiner(correct)))),
                        ResetColor
                    )?;

                    execute!(
                        stdout(),
                        SetForegroundColor(DarkYellow),
                        Print("Override, I am correct (y/N): "),
                        ResetColor
                    )?;

                    if input()?.to_ascii_lowercase().starts_with("y") {
                        score += on_correct()?;
                    } else {
                        score += delta;
                    }
                }
            }
            await_key()?;
        }
    }
}

fn on_correct() -> Result<i64, io::Error> {
    execute!(
        stdout(),
        SetForegroundColor(Green),
        Print("Correct!\n"),
        ResetColor
    )?;
    Ok(1)
}

fn on_incorrect() -> Result<i64, io::Error> {
    execute!(
        stdout(),
        SetForegroundColor(Red),
        Print("Incorrect :(\n"),
        ResetColor
    )?;
    Ok(-1)
}

fn await_key() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    execute!(stdout(),
        SetForegroundColor(DarkGrey),
        Print("Press any key to continue..."),
        SetForegroundColor(Reset)
    )?;
    event::read()?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn readable_joiner(data: &Vec<String>) -> &'static str {
    return if data.into_iter().any(|datum| datum.contains(",")) { "; " } else { ", "}
}

fn input() -> Result<String, io::Error> {
    let mut data = String::new();
    stdin().read_line(&mut data)?;
    return Ok(data.trim().to_string());
}

fn clear_terminal() -> Result<(), io::Error> {
    execute!(stdout(), ResetColor, Clear(ClearType::All), MoveTo(0, 0))
}