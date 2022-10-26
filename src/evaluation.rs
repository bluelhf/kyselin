use crate::evaluation::Correctness::*;

pub enum Correctness {
    Correct,
    Incorrect
}

impl crate::parsing::Answer {
    pub fn evaluate(&self, input: String) -> Correctness {
        if self.correct.contains(&input) {
            return Correct;
        }

        return Incorrect;
    }
}