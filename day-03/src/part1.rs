use std::str::FromStr;

use anyhow::Result;
use derive_more::{Add, From};

#[derive(Debug, From, Clone, Copy, Add)]
struct Point(i32, i32);

pub struct Schematic {
    data: Vec<Vec<char>>,
    bounds: Point,
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let data: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let bounds = (data.len() as i32, data[0].len() as i32).into();
        Ok(Schematic { data, bounds })
    }
}

impl Schematic {
    fn is_inbound(&self, point: Point) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < self.bounds.0 && point.1 < self.bounds.1
    }

    fn get(&self, point: Point) -> Option<char> {
        match self.is_inbound(point) {
            true => Some(self.data[point.0 as usize][point.1 as usize]),
            false => None,
        }
    }

    fn get_surrounding(&self, point: Point) -> Vec<char> {
        itertools::iproduct!((-1..=1), (-1..=1))
            .filter(|p| *p != (0, 0)) // skip (0,0)
            .filter_map(|delta| self.get(point + delta.into()))
            .collect()
    }
}

#[derive(Default)]
pub struct NumbersStack {
    numbers: Vec<u32>,
    stack: u32,
    is_adjacent: bool,
}

impl NumbersStack {
    /// Push a char into the temporary stack
    fn push(&mut self, c: char) {
        self.stack = self.stack * 10 + c.to_digit(10).expect("to be a digit")
    }

    fn set_adjacent(&mut self) {
        self.is_adjacent = true;
    }

    /// Commit the cache to the numbers
    fn commit(&mut self) {
        if self.is_adjacent && self.stack > 0 {
            self.numbers.push(self.stack);
        }
        self.clear();
    }

    /// Clear the stack & is_adjacent
    fn clear(&mut self) {
        self.stack = Default::default();
        self.is_adjacent = Default::default();
    }
}

pub fn process(input: &str) -> Result<String> {
    let schematic: Schematic = input.parse().expect("to be valid schematic");

    let mut stack = NumbersStack::default();
    for x in 0..schematic.bounds.0 {
        for y in 0..schematic.bounds.1 {
            let p = (x, y).into();
            let c = schematic.get(p).expect("point to exists");
            if c.is_ascii_digit() {
                stack.push(c);
                if schematic
                    .get_surrounding(p)
                    .into_iter()
                    .any(|s| !s.is_ascii_digit() && s != '.')
                {
                    stack.set_adjacent();
                }
            } else {
                stack.commit();
            }
        }
        stack.commit();
    }

    Ok(stack.numbers.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_inbound() {
        let schema = Schematic {
            data: vec![],
            bounds: Point(2, 2),
        };
        assert!(schema.is_inbound(Point(0, 0)));
        assert!(!schema.is_inbound(Point(-1, 0)));
        assert!(!schema.is_inbound(Point(0, -1)));
        assert!(schema.is_inbound(Point(0, 1)));
        assert!(schema.is_inbound(Point(1, 0)));
        assert!(schema.is_inbound(Point(1, 1)));
        assert!(!schema.is_inbound(Point(2, 0)));
        assert!(!schema.is_inbound(Point(0, 2)));
        assert!(!schema.is_inbound(Point(2, 2)));
    }

    #[test]
    fn test_get() {
        let schema: Schematic = "...*......\n..35..633.".parse().unwrap();
        assert_eq!(schema.get(Point(0, 0)), Some('.'));
        assert_eq!(schema.get(Point(3, 0)), None);
        assert_eq!(schema.get(Point(0, 3)), Some('*'));
        assert_eq!(schema.get(Point(1, 2)), Some('3'));
        assert_eq!(schema.get(Point(0, 10)), None);
    }

    #[test]
    fn test_get_surrounding() {
        let schema: Schematic = "467..114..\n...*......\n..35..633.".parse().unwrap();
        assert_eq!(schema.get_surrounding(Point(0, 0)), vec!['6', '.', '.']);
        assert_eq!(
            schema.get_surrounding(Point(1, 3)),
            vec!['7', '.', '.', '.', '.', '3', '5', '.']
        );
        assert_eq!(schema.get_surrounding(Point(2, 9)), vec!['.', '.', '3']);
        assert_eq!(schema.get_surrounding(Point(4, 0)), vec![]);
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> Result<()> {
        let input = r"123.
.123";
        assert_eq!("0", process(input)?);
        Ok(())
    }
}
