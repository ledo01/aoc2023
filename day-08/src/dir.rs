#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
}

impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Dir::Left),
            'R' => Ok(Dir::Right),
            _ => Err(()),
        }
    }
}

pub fn parse_line(line: &str) -> (&str, (&str, &str)) {
    let (index, rest) = line.split_once(" = ").expect("to have an =");
    let rest = rest
        .strip_prefix('(')
        .expect("to start with (")
        .strip_suffix(')')
        .expect("to end with )")
        .split_once(", ")
        .expect("to have a ,");
    (index, rest)
}
