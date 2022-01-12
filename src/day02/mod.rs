mod command;
mod input;
mod submarine;
mod submarine2;

use input::INPUT;
use submarine::Submarine;
use submarine2::Submarine2;

pub(super) fn run() {
    part01();
    part02();
}

fn part01() {
    let mut submarine = Submarine::default();

    INPUT
        .iter()
        .map(|line| line.parse().unwrap())
        .for_each(|command| submarine.run_command(command));

    println!("DAY02 PART1: {}", submarine.get_result());
}

fn part02() {
    let mut submarine = Submarine2::default();

    INPUT
        .iter()
        .map(|line| line.parse().unwrap())
        .for_each(|command| submarine.run_command(command));

    println!("DAY02 PART2: {}", submarine.get_result());
}
