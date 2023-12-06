use std::ops::Range;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Map(pub Vec<(Range<u64>, Range<u64>)>);

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .skip(1)
            .map(|line| {
                let mut iter = line.split(char::is_whitespace);
                let dest: u64 = iter
                    .next()
                    .expect("to have a dest")
                    .parse()
                    .expect("dest to be a number");
                let source: u64 = iter
                    .next()
                    .expect("to have a source")
                    .parse()
                    .expect("source to be a number");
                let range: u64 = iter
                    .next()
                    .expect("to have a range")
                    .parse()
                    .expect("range to be a number");
                (source..source + range, dest..dest + range)
            })
            .collect()))
    }
}

impl Map {
    /// Return the mapped destination
    pub fn get(&self, key: u64) -> u64 {
        match self.0.iter().find(|range| range.0.contains(&key)) {
            Some((source, dest)) => dest.start + (key - source.start),
            None => key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seed_to_soil() {
        let input = r"seed-to-soil map:
50 98 2
52 50 48";
        assert_eq!(
            input.parse::<Map>().unwrap(),
            Map(vec![(98..100, 50..52), (50..98, 52..100)])
        );
    }
}
