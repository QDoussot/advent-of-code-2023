use derive_more::Display;
use std::fmt::Debug;

#[derive(Display, Debug)]
pub enum ProblemError {
    UnverifiedConstraint(String),
    ParsingFailed(String)
}

#[allow(dead_code)]
pub trait Problem: Sized {
    fn parse(puzzle_input: String) -> Result<Self, ProblemError>;
    fn part_one(&self) -> Result<String, ProblemError>;
    fn part_two(&self) -> Result<String, ProblemError>;
}

#[allow(dead_code)]
pub fn solve<T: Problem + Debug>(puzzle_input: String, part: usize) -> Result<String, ProblemError> {
    let problem = T::parse(puzzle_input)?;
    if part == 0 {
        Ok(format!("{:#?}", problem))
    } else if part == 1 {
        problem.part_one()
    } else {
        problem.part_two()
    }
}
