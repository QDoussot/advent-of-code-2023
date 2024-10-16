mod aoc;
mod day02;
mod day03;
use crate::aoc::problem::solve;
use crate::aoc::AocEx;
use aoc::{Aoc, Error};

mod day01;

struct TwentyThree {}

impl Aoc for TwentyThree {
    fn solve(&self, puzzle_input: String, day: usize, part: usize) -> Result<String, Error> {
        match day {
            1 => Ok(solve::<day01::Trebuchet>(puzzle_input, part)?),
            2 => Ok(solve::<day02::CubeGameSession>(puzzle_input, part)?),
            3 => Ok(solve::<day03::EngineSchematic>(puzzle_input, part)?),
        _ => Err(Error::NoCorrespondingSolver(day,part))
        }
    }
}

fn main() -> Result<(), Error> {
    let tt = TwentyThree {};
    let solution = tt.run()?;
    println!("{solution}");
    Ok(())
}
