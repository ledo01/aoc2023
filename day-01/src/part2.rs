use anyhow::Result;

pub fn parse_line(line: &str) -> u32 {
    let mut numbers = (0..line.len()).filter_map(|index| {
        let slice = &line[index..];
        match slice {
            _ if slice.starts_with("one") => Some(1),
            _ if slice.starts_with("two") => Some(2),
            _ if slice.starts_with("three") => Some(3),
            _ if slice.starts_with("four") => Some(4),
            _ if slice.starts_with("five") => Some(5),
            _ if slice.starts_with("six") => Some(6),
            _ if slice.starts_with("seven") => Some(7),
            _ if slice.starts_with("eight") => Some(8),
            _ if slice.starts_with("nine") => Some(9),
            _ => slice.chars().next().unwrap().to_digit(10),
        }
    });
    let first = numbers.next().expect("to start with a number");
    match numbers.last() {
        Some(last) => first * 10 + last,
        None => first * 10 + first,
    }
}

pub fn process(input: &str) -> Result<String> {
    let output = input.lines().map(parse_line).sum::<u32>().to_string();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> Result<()> {
        assert_eq!(83, parse_line("eightwothree"));
        assert_eq!(24, parse_line("xtwone3four"));
        assert_eq!(76, parse_line("7pqrstsixteen"));
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
