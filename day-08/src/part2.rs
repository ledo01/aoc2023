use std::{collections::HashMap, usize};

use anyhow::Result;
use num::Integer;

use crate::dir::{parse_line, Dir};

pub fn process(input: &str) -> Result<String> {
    let mut lines = input.lines();
    let directions: Vec<Dir> = lines
        .next()
        .expect("to have the directions")
        .chars()
        .filter_map(|c| Dir::try_from(c).ok())
        .collect();

    let nodes = lines.skip(1).map(parse_line);
    let map: HashMap<&str, (&str, &str)> = nodes.clone().collect();
    let nodes: Vec<&str> = nodes
        .filter_map(|(index, _)| index.ends_with('A').then_some(index))
        .collect();

    let results: Vec<_> = nodes
        .into_iter()
        .map(|node| {
            let mut dir = directions.iter().cycle();
            let mut cursor = node;
            let mut steps: usize = 0;
            while !cursor.ends_with('Z') {
                let values = map.get(cursor).expect("to have an entry");
                steps += 1;
                cursor = match dir.next().unwrap() {
                    Dir::Left => values.0,
                    Dir::Right => values.1,
                };
            }
            steps
        })
        .collect();

    Ok(results.iter().fold(1, |res, x| res.lcm(x)).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        "6"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> Result<()> {
        assert_eq!(process(input)?, expected);
        Ok(())
    }
}
