use std::{collections::HashSet, str::FromStr};

use anyhow::Result;

type Point = (i32, i32);

#[derive(Debug)]
pub struct Part {
    pub value: u32,
    pub points: HashSet<Point>,
}

impl Part {
    fn new(origin: Point, c: char) -> Self {
        let value = c.to_digit(10).expect("to be a digit");
        let points = itertools::iproduct!((-1..=1), (-1..=1))
            .map(|(x, y)| (origin.0 + x, origin.1 + y))
            .collect();

        Part { value, points }
    }

    /// Push a new digit along with its origin
    fn push(&mut self, origin: Point, c: char) {
        let value = c.to_digit(10).expect("to be a digit");
        self.value = self.value * 10 + value;
        self.points.extend(
            itertools::iproduct!((-1..=1), (-1..=1)).map(|(x, y)| (origin.0 + x, origin.1 + y)),
        )
    }
}

pub struct Schematic {
    pub parts: Vec<Part>,
    pub symbols: HashSet<Point>,
    pub gears: HashSet<Point>,
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut parts = vec![];
        let mut symbols = HashSet::new();
        let mut gears = HashSet::new();
        for (x, line) in s.lines().enumerate() {
            let mut cur_part: Option<Part> = None;
            for (y, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    match cur_part {
                        Some(ref mut part) => part.push((x as i32, y as i32), c),
                        None => cur_part = Some(Part::new((x as i32, y as i32), c)),
                    }
                } else {
                    if let Some(part) = cur_part {
                        parts.push(part);
                        cur_part = None
                    }
                    if c != '.' {
                        symbols.insert((x as i32, y as i32));
                        if c == '*' {
                            gears.insert((x as i32, y as i32));
                        }
                    }
                }
            }
            if let Some(part) = cur_part {
                parts.push(part);
            }
        }
        Ok(Schematic {
            parts,
            symbols,
            gears,
        })
    }
}
