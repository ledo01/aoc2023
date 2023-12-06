use anyhow::{Context, Result};

use day_05::part1::process;

fn main() -> Result<()> {
    let input = include_str!("../../input1.txt");
    let output = process(input).context("process part 1")?;
    println!("{output}");
    Ok(())
}
