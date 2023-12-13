use anyhow::{Context, Result};

use day_08::part2::process;

fn main() -> Result<()> {
    let input = include_str!("../../input2.txt");
    let output = process(input).context("process part 2")?;
    println!("{output}");
    Ok(())
}
