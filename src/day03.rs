use crate::aoc::problem::{Problem, ProblemError};
use huparse::parse::Parse;
use huparse::parser;

#[derive(Debug)]
pub struct EngineSchematic(Vec<String>);

#[allow(dead_code)]
fn number_positions(line: &str) -> Vec<(usize, &str)> {
    let mut res = line.char_indices().fold(
        (Vec::<(usize, &str)>::new(), None),
        |(numbers, curr_start), (ind, char)| match (curr_start, char.is_digit(10)) {
            //New number found
            (None, true) => (numbers, Some(ind)),
            //Going on with dots
            (None, false) => (numbers, None),
            //Going on with current number
            (Some(curr_start), true) => (numbers, Some(curr_start)),
            //Ended current number
            (Some(curr_start), false) => (
                [numbers, vec![(curr_start, &line[curr_start..ind])]].concat(),
                None,
            ),
        },
    );
    if let Some(curr_start) = res.1 {
        res.0.push((curr_start, &line[curr_start..]));
    }
    res.0
}

impl EngineSchematic {
    fn has_adjacent_symbol(&self, line: usize, start: usize, end: usize) -> bool {
        if end <= start { return false };

        let is_symbol = |c: char| !c.is_digit(10) && c != '.';
        let adj_start = if start > 0 { start - 1 } else { 0 };
        let adj_end = if end + 1 <= self.0[line].len() {
            end + 1
        } else {
            self.0[line].len()
        };

        let mut has_adjacent_symbol = false;
        if line > 0 {
            has_adjacent_symbol |= self.0[line - 1][adj_start..adj_end].contains(is_symbol)
        } 
        if adj_start < start {
            has_adjacent_symbol |= self.0[line][adj_start..adj_start + 1].contains(is_symbol)
        } 
        if adj_end > end {
            has_adjacent_symbol |= self.0[line][adj_end-1..adj_end].contains(is_symbol)
        } 
        if line + 1 < self.0.len() {
            has_adjacent_symbol |= self.0[line + 1][adj_start..adj_end].contains(is_symbol)
        }
        has_adjacent_symbol
    }
}

impl Problem for EngineSchematic {
    fn parse(puzzle_input: String) -> Result<Self, ProblemError> {
        Ok(Self(
            puzzle_input
                .split_terminator("\n")
                .map(String::from)
                .collect(),
        ))
    }

    fn part_one(&self) -> Result<String, ProblemError> {
        let part_numbers: Result<Vec<usize>, _> = self
            .0
            .iter()
            .enumerate()
            .map(|(line_nbr, line)| {
                number_positions(line)
                    .iter()
                    .filter_map(move |(pos, number)| {
                        if self.has_adjacent_symbol(line_nbr, *pos, *pos + number.len()) {
                            Some(number.parse::<usize>())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Result<usize, _>>>()
            })
            .flatten()
            .inspect(|number| println!("{number:?}"))
            .collect();

        let sum: usize = part_numbers
            .map_err(|e| ProblemError::ParsingFailed(e.to_string()))?
            .iter()
            .sum();
        Ok(format!("{sum}"))
    }

    fn part_two(&self) -> Result<String, ProblemError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_returns_numbers_with_indices() {
        use crate::day03::number_positions;

        let shematic_line = "117..*98.23...45";
        //                   0123456789abcdef
        let expected = vec![(0, "117"), (6, "98"), (9, "23"), (14, "45")];
        assert_eq!(number_positions(&shematic_line), expected);
    }
}
