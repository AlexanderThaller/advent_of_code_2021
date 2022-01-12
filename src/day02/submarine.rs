use super::command::Command;

#[derive(Debug, Default, Eq, PartialEq)]
pub(super) struct Submarine {
    horizontal_position: usize,
    depth: usize,
}

impl Submarine {
    pub(super) fn get_result(&self) -> usize {
        self.horizontal_position * self.depth
    }

    pub(super) fn run_command(&mut self, command: Command) {
        match command {
            Command::Forward(value) => self.horizontal_position += value as usize,
            Command::Down(value) => self.depth += value as usize,
            Command::Up(value) => self.depth -= value as usize,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Submarine;
    use crate::day02::input::EXAMPLE;

    mod run_command {
        #[test]
        fn example() {
            let mut submarine = super::Submarine::default();

            super::EXAMPLE
                .lines()
                .map(|line| line.parse().unwrap())
                .for_each(|command| submarine.run_command(command));

            dbg!(&submarine);

            assert_eq!(submarine.horizontal_position, 15);
            assert_eq!(submarine.depth, 10);
        }
    }
}
