#[derive(Debug, Eq, PartialEq)]
pub(super) enum Command {
    Forward(u8),
    Down(u8),
    Up(u8),
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_ascii_whitespace().collect::<Vec<_>>();

        match split.as_slice() {
            [] | [_] => Err("too short".to_string()),
            [_, _, _, ..] => Err("too long".to_string()),

            [command, amount] => {
                let parsed_amount = amount.parse().unwrap();

                match *command {
                    "forward" => Ok(Self::Forward(parsed_amount)),
                    "down" => Ok(Self::Down(parsed_amount)),
                    "up" => Ok(Self::Up(parsed_amount)),

                    _ => Err(format!("unkown command {}", command)),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Command;
    use crate::day02::input::EXAMPLE;

    mod from_str {
        #[test]
        fn example() {
            use super::Command::*;

            let expected = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];

            let got = super::EXAMPLE
                .lines()
                .map(|line| line.parse().unwrap())
                .collect::<Vec<super::Command>>();

            dbg!(&got);

            assert_eq!(expected, got);
        }
    }
}
