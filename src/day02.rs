pub(super) fn run() {
    todo!()
}

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Forward(u16),
    Down(u16),
    Up(u16),
}

impl std::str::FromStr for Move {
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
    use super::Move;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    mod from_str {
        #[test]
        fn example() {
            use super::Move::*;

            let expected = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];

            let got = super::EXAMPLE
                .lines()
                .map(|line| line.parse().unwrap())
                .collect::<Vec<super::Move>>();

            dbg!(&got);

            assert_eq!(expected, got);
        }
    }
}
