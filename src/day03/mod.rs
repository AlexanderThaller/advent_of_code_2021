mod input;

pub(super) fn run() {
    part01();
    part02();
}

fn part01() {
    todo!()
}

fn part02() {
    todo!()
}

#[derive(Debug, Default, Eq, PartialEq)]
struct GammaRate(usize);

impl GammaRate {
    fn calculate(input: &[usize]) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::GammaRate;
    use crate::day03::input::EXAMPLE;

    mod calculate {
        #[test]
        fn example() {
            let got = super::GammaRate::calculate(super::EXAMPLE);

            dbg!(got);

            todo!()
        }
    }
}
