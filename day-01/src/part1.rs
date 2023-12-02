use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let output = input
        .lines()
        .map(|line| {
            let mut nums = line.chars().filter_map(|c| c.to_digit(10));
            let first = nums.next().expect("to start with a number");
            match nums.last() {
                Some(last) => first * 10 + last,
                None => first * 10 + first,
            }
        })
        .sum::<u32>()
        .to_string();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
