use anyhow::Result;

use crate::map::Map;

pub fn process(input: &str) -> Result<String> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<_> = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let maps: Vec<Map> = maps.split("\n\n").map(|map| map.parse().unwrap()).collect();
    Ok(seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |source, map| map.get(source)))
        .min()
        .expect("to have a min")
        .to_string())
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

    #[test]
    fn test_process() -> Result<()> {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process(input)?, "35");
        Ok(())
    }
}
