mod input;

pub(super) fn run() {
    part01();
    part02();
}

fn part01() {
    let rate = Rate::calculate(input::INPUT);

    println!("DAY03 PART1: {}", rate.result());
}

fn part02() {
    todo!()
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Rate {
    gamma: usize,
    epsilon: usize,
}

impl Rate {
    fn result(&self) -> usize {
        self.gamma * self.epsilon
    }

    fn calculate(input: &[&str]) -> Self {
        let len = input[0].len();

        let chars = input
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .fold(
                Vec::with_capacity(len),
                |mut acc: Vec<(usize, usize)>, line| {
                    for (index, c) in line.into_iter().enumerate() {
                        if c == '0' {
                            if let Some(value) = acc.get_mut(index) {
                                value.0 += 1;
                                acc[index] = *value;
                            } else {
                                acc.push((1, 0));
                            }
                        } else if let Some(value) = acc.get_mut(index) {
                            value.1 += 1;
                            acc[index] = *value;
                        } else {
                            acc.push((0, 1));
                        }
                    }

                    acc
                },
            );

        let mut gamma = Vec::new();
        let mut epsilon = Vec::new();

        for c in chars {
            if c.0 > c.1 {
                gamma.push(0);
                epsilon.push(1);
            } else {
                gamma.push(1);
                epsilon.push(0);
            }
        }

        let gamma = gamma.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&x.to_string());
            acc
        });
        let epsilon = epsilon.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&x.to_string());
            acc
        });

        let gamma = usize::from_str_radix(&gamma, 2).unwrap();
        let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

        Self { gamma, epsilon }
    }
}

#[cfg(test)]
mod test {
    use super::Rate;
    use crate::day03::input::EXAMPLE;

    mod calculate {
        #[test]
        fn example() {
            let got = super::Rate::calculate(super::EXAMPLE);

            let expected = super::Rate {
                gamma: 22,
                epsilon: 9,
            };

            dbg!(&got);

            assert_eq!(expected, got);
        }
    }
}
