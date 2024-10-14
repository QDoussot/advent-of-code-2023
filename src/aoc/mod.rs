use derive_more::Display;
use std::io;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;
pub mod problem;

#[derive(Display, Debug)]
pub enum Error {
    #[display("Failed opening file \"{}\" : \"{}\"", _0, _1)]
    CantOpenInputFile(String, String),
    #[display("No solver implemented for day {}, part: {}", _0, _1)]
    NoCorrespondingSolver(usize,usize),
    #[allow(dead_code)]
    #[display("Solver/Parser failed: \"{}\"", _0)]
    CouldNotSolveProblem(problem::ProblemError),
}

impl From<problem::ProblemError> for Error {
    fn from(value: problem::ProblemError) -> Self {
        Self::CouldNotSolveProblem(value)
    }
}

pub trait Aoc {
    fn solve(&self, lines: Vec<String>, day: usize, part: usize) -> Result<String, Error>;
}

pub trait AocEx: Aoc {
    fn run(&self) -> Result<String, Error>;
}

impl<T> AocEx for T
where
    T: Aoc,
{
    fn run(&self) -> Result<String, Error> {
        let opt = Opt::from_args();

        let file_name = match opt.input {
            None => {
                let ext = match opt.example {
                    false => "",
                    true => ".example",
                };
                format!("inputs/{}{}", opt.day, ext)
            }
            Some(file_name) => file_name,
        };

        let file = std::fs::File::open(&file_name)
            .map_err(|e| Error::CantOpenInputFile(file_name, e.to_string()))?;

        let lines = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        let solution = self.solve(lines, opt.day, opt.part)?;

        Ok(solution)
    }
}

#[derive(StructOpt)]
pub struct Opt {
    day: usize,
    part: usize,
    #[structopt(long)]
    input: Option<String>,
    #[structopt(long, conflicts_with = "input")]
    example: bool,
}
