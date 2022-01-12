use super::command::Command;

#[derive(Debug, Default, Eq, PartialEq)]
pub(super) struct Submarine2 {
    horizontal_position: usize,
    depth: usize,
    aim: usize,
}

impl Submarine2 {
    pub(super) fn get_result(&self) -> usize {
        self.horizontal_position * self.depth
    }

    pub(super) fn run_command(&mut self, command: Command) {
        match command {
            Command::Forward(value) => {
                self.horizontal_position += value as usize;
                self.depth += value as usize * self.aim;
            }
            Command::Down(value) => self.aim += value as usize,
            Command::Up(value) => self.aim -= value as usize,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Submarine2;
    use crate::day02::input::{
        EXAMPLE,
        INPUT,
    };

    mod run_command {
        #[test]
        fn example() {
            let mut submarine = super::Submarine2::default();

            super::EXAMPLE
                .lines()
                .map(|line| line.parse().unwrap())
                .for_each(|command| submarine.run_command(command));

            dbg!(&submarine);

            assert_eq!(submarine.horizontal_position, 15);
            assert_eq!(submarine.depth, 60);
        }

        #[test]
        fn input() {
            let mut submarine = super::Submarine2::default();

            super::INPUT
                .iter()
                .map(|line| line.parse().unwrap())
                .for_each(|command| submarine.run_command(command));

            dbg!(&submarine);

            assert_eq!(submarine.horizontal_position, 1957);
            assert_eq!(submarine.depth, 1004584);
            assert_eq!(submarine.aim, 955);
        }
    }
}
