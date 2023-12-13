use std::collections::HashMap;

use anyhow::Result;

use crate::dir::{parse_line, Dir};

pub fn process(input: &str) -> Result<String> {
    let mut lines = input.lines();
    let mut directions = lines
        .next()
        .expect("to have the directions")
        .chars()
        .filter_map(|c| Dir::try_from(c).ok())
        .cycle();

    let map: HashMap<&str, (&str, &str)> = lines.skip(1).map(parse_line).collect();

    let mut cursor = "AAA";
    let target = "ZZZ";
    let mut steps = 0;
    while cursor != target {
        let values = map.get(cursor).expect("to have an entry");
        cursor = match directions.next() {
            Some(Dir::Left) => values.0,
            Some(Dir::Right) => values.1,
            None => unreachable!(),
        };
        steps += 1;
    }

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        "2"
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        "6"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> Result<()> {
        assert_eq!(process(input)?, expected);
        Ok(())
    }
}
