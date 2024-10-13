use derive_more::Display;
use std::fmt::Debug;

#[derive(Display, Debug)]
pub struct ProblemError {}

#[allow(dead_code)]
pub trait Problem: Sized {
    fn parse(lines: Vec<String>) -> Result<Self, ProblemError>;
    fn part_one(&self) -> Result<String, ProblemError>;
    fn part_two(&self) -> Result<String, ProblemError>;
}

#[allow(dead_code)]
pub fn solve<T: Problem + Debug>(lines: Vec<String>, part: usize) -> Result<String, ProblemError> {
    let problem = T::parse(lines).map_err(|_| ProblemError{})?;
    if part == 0 {
        Ok(format!("{:#?}", problem))
    } else if part == 1 {
        problem.part_one().map_err(|_| ProblemError{})
    } else {
        problem.part_two().map_err(|_| ProblemError{})
    }
}
