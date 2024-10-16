use crate::aoc::problem::{Problem, ProblemError};
use huparse::parse::Parse;
use huparse::parser;
use derive_more::FromStr;
use std::ops::{Index, IndexMut};

#[derive(FromStr)]
enum Color {
    Red,
    Green,
    Blue,
}

impl IndexMut<Color> for [usize; 3] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::Red => &mut self[0],
            Color::Green => &mut self[1],
            Color::Blue => &mut self[2],
        }
    }
}

impl Index<Color> for [usize; 3] {
    type Output = usize;
    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::Red => &self[0],
            Color::Green => &self[1],
            Color::Blue => &self[2],
        }
    }
}

#[derive(Debug)]
struct Pick([usize; 3]);

impl From<Vec<(usize, Color)>> for Pick {
    fn from(value: Vec<(usize, Color)>) -> Self {
        let mut balls = [0, 0, 0];
        value
            .into_iter()
            .for_each(|(number, color)| balls[color] = number);
        Pick(balls)
    }
}

impl Pick {
    fn is_possible_from(&self, bag: [usize; 3]) -> bool {
        self.0.iter().zip(bag.iter()).all(|(p, max)| p <= max)
    }
}

#[derive(Debug)]
struct CubeGame {
    id: usize,
    picks: Vec<Pick>,
}

fn zip<T: Copy + Default, const N: usize>(a: [T; N], b: [T; N]) -> [(T, T); N] {
    let mut ind = 0;
    a.map(|value| {
        let res = (value, b[ind]);
        ind += 1;
        res
    })
}

impl CubeGame {
    fn minimum_possible_bag(&self) -> [usize; 3] {
        self.picks.iter().fold([0, 0, 0], |acc, pick| {
            zip(acc, pick.0).map(|(curr_min_poss, pick)| usize::max(curr_min_poss, pick))
        })
    }
}

#[derive(Debug)]
pub struct CubeGameSession(Vec<CubeGame>);

impl Problem for CubeGameSession {
    fn parse(puzzle_input: String) -> Result<Self, ProblemError> {
        let pick_parser = parser!([("% %", usize, Color) | ", "]);
        let parser = parser!([ ("Game %: %", usize, [{ pick_parser } | "; "]) | "\n"!] );
        let games = parser
            .parse_top(&puzzle_input)
            .map_err(|e| ProblemError::ParsingFailed(e.to_string()))?;

        let games = games
            .into_iter()
            .map(|game| CubeGame {
                id: game.0,
                picks: game.1.into_iter().map(Pick::from).collect(),
            })
            .collect();
        Ok(CubeGameSession(games))
    }

    fn part_one(&self) -> Result<String, ProblemError> {
        let id_sum: usize = self
            .0
            .iter()
            .filter(|CubeGame { id: _id, picks }| {
                picks.iter().all(|p| p.is_possible_from([12, 13, 14]))
            })
            .map(|CubeGame { id, picks: _ }| id)
            .sum();
        Ok(format!("{id_sum}"))
    }

    fn part_two(&self) -> Result<String, ProblemError> {
        let power_sum: usize = self
            .0
            .iter()
            .map(|game| game.minimum_possible_bag().iter().product::<usize>())
            .sum();
        Ok(format!("{power_sum}"))
    }
}

#[cfg(test)]
mod tests {

    use crate::day02::CubeGame;
    use crate::day02::Pick;

    #[test]
    fn minimum_possible_bag_for_one_pick() {
        let game = CubeGame {
            id: 0,
            picks: vec![Pick([3, 6, 7])],
        };
        assert_eq!(game.minimum_possible_bag(), [3, 6, 7]);
    }

    #[test]
    fn minimum_possible_bag_for_pick_with_missing_color() {
        let game = CubeGame {
            id: 0,
            picks: vec![Pick([3, 0, 7]), Pick([1, 4, 0])],
        };
        assert_eq!(game.minimum_possible_bag(), [3, 4, 7]);
    }
}
