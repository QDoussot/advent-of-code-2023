mod aoc;
use crate::aoc::AocEx;
use aoc::{Aoc, Error};

struct TwentyThree {}

impl Aoc for TwentyThree {
    fn solve(&self, lines: Vec<String>, day: usize, part: usize) -> Result<String, Error> {
        let _ = lines;
        Err(Error::NoCorrespondingSolver(day,part))
    }
}

fn main() -> Result<(), Error> {
    let tt = TwentyThree {};
    let solution = tt.run()?;
    println!("{solution}");
    Ok(())
}
