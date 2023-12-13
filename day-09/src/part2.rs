use anyhow::Result;

fn predict_prev(values: &[i64]) -> i64 {
    if values.iter().all(|v| *v == 0) {
        return 0;
    }
    let deltas: Vec<_> = values
        .windows(2)
        .rev()
        .map(|window| window[1] - window[0])
        .rev()
        .collect();
    let first = values[0];
    first - predict_prev(&deltas)
}

pub fn process(input: &str) -> Result<String> {
    let results = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .map(|values: Vec<_>| predict_prev(&values))
        .sum::<i64>();
    Ok(results.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_next() {
        assert_eq!(predict_prev(&[0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(predict_prev(&[1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(predict_prev(&[10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process(input)?, "2");
        Ok(())
    }
}
