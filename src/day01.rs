use crate::aoc::problem::{Problem, ProblemError};
use derive_more::Display;
use itertools::Itertools;

#[derive(Debug, Display)]
struct Calibration(String);

trait Reader {
    fn find_first_digit(cal: &Calibration) -> Option<u32>;
    fn find_last_digit(cal: &Calibration) -> Option<u32>;
}

struct CharDigit {}
impl Reader for CharDigit {
    fn find_first_digit(cal: &Calibration) -> Option<u32> {
        cal.0
            .chars()
            .find(|c| c.is_digit(10))
            .and_then(|c| c.to_digit(10))
    }

    fn find_last_digit(cal: &Calibration) -> Option<u32> {
        cal.0
            .chars()
            .rfind(|c| c.is_digit(10))
            .and_then(|c| c.to_digit(10))
    }
}

struct SpelledOut {}
impl SpelledOut {
    fn digits() -> Vec<(&'static str, u32)> {
        vec![
            ("0", 0u32),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
    }
}

impl Reader for SpelledOut {
    fn find_first_digit(cal: &Calibration) -> Option<u32> {
        Self::digits()
            .iter()
            .map(|(code, value)| cal.0.find(code).zip(Some(value)))
            .flatten()
            .min_by_key(|(pos, _)| *pos)
            .map(|(_, value)| *value)
    }

    fn find_last_digit(cal: &Calibration) -> Option<u32> {
        Self::digits()
            .iter()
            .map(|(code, value)| cal.0.rfind(code).zip(Some(value)))
            .flatten()
            .max_by_key(|(pos, _)| *pos)
            .map(|(_, value)| *value)
    }
}

#[derive(Debug)]
pub struct Trebuchet {
    calibration: Vec<Calibration>,
}

impl Trebuchet {
    fn sum_calibration_values<D: Reader>(&self) -> Result<u32, ProblemError> {
        self.calibration
            .iter()
            .enumerate()
            .map(|(i, cal)| {
                D::find_first_digit(cal)
                    .zip(D::find_last_digit(cal))
                    .ok_or_else(|| {
                        ProblemError::UnverifiedConstraint(format!(
                            "No digit for calibration line {}",
                            i + 1
                        ))
                    })
            })
            .fold_ok(0, |sum, (tenth, unit)| {
                sum + tenth * 10 + unit
            })
    }
}

impl Problem for Trebuchet {
    fn parse(lines: Vec<String>) -> Result<Self, crate::aoc::problem::ProblemError> {
        let calibration = lines.into_iter().map(Calibration).collect();
        Ok(Trebuchet { calibration })
    }

    fn part_one(&self) -> Result<String, crate::aoc::problem::ProblemError> {
        let result = self.sum_calibration_values::<CharDigit>()?;
        Ok(format!("{result}"))
    }

    fn part_two(&self) -> Result<String, crate::aoc::problem::ProblemError> {
        let result = self.sum_calibration_values::<SpelledOut>()?;
        Ok(format!("{result}"))
    }
}
