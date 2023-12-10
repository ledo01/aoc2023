use anyhow::Result;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

/// n: number of ms held
/// d: distance in mm
/// s: speed of the boat in mm/ms
/// t: total time in ms
/// r: record in mm
///
/// So the boat doesn't move if n(t-n) == 0 => n = 0 or n = t
/// Also, if n(t-n) > r then the record if broken, so
/// nt - n^2 - r > 0
fn solve(race: &Race) -> u32 {
    let t = race.time as f32;
    let r = race.distance as f32;
    let disc = (t.powf(2.0) - (4.0 * r)).sqrt();
    let min = (-t + disc) / -2.0;
    let max = (-t - disc) / -2.0;
    max.ceil() as u32 - min.floor() as u32 - 1
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("to have the times")
        .strip_prefix("Time:")
        .expect("to start with `Time:`")
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap());
    let distances = lines
        .next()
        .expect("to have the times")
        .strip_prefix("Distance:")
        .expect("to start with `Distance:`")
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap());
    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

pub fn process(input: &str) -> Result<String> {
    let races = parse_input(input);
    Ok(races.iter().map(solve).product::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input).unwrap(), "288");
        Ok(())
    }
}
