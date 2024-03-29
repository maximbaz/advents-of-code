use itertools::Itertools;
use lazy_static::lazy_static;
use std::env;
use std::fmt::Display;
use std::fs;
use std::time::Instant;

mod year2020;
mod year2021;

pub trait Solution {
    type Input;
    type Output: Display;

    fn parse_input(&self, input: String) -> Self::Input;
    fn part1(&self, input: Self::Input) -> Self::Output;
    fn part2(&self, input: Self::Input) -> Self::Output;

    fn read_input(&self, year: u16, day: u8) -> String {
        fs::read_to_string(format!("input/{}/{:02}", year, day)).expect("Error reading the file")
    }

    fn solve(&self, year: u16, day: u8, part: u8) -> String {
        let input = self.parse_input(self.read_input(year, day));
        if part == 1 {
            self.part1(input).to_string()
        } else {
            self.part2(input).to_string()
        }
    }
}

fn main() {
    let args = env::args().collect_vec();
    let year = args.get(1).map(|x| x.parse().expect("invalid number"));
    let day = args.get(2).map(|x| x.parse().expect("invalid number"));
    let part = args.get(3).map(|x| x.parse().expect("invalid number"));

    (2015..=2021)
        .cartesian_product((1..=25).cartesian_product(1..=2))
        .filter(|(y, (d, p))| {
            *y == year.unwrap_or(*y) && *d == day.unwrap_or(*d) && *p == part.unwrap_or(*p)
        })
        .filter_map(|(y, (d, p))| solve(y, d, p).map(|(a, ms)| (y, d, p, a, ms)))
        .for_each(|(y, d, p, a, ms)| {
            println!("{} day {:02} part {} => {:15} (took {}ms)", y, d, p, a, ms)
        });
}

fn solve(year: u16, day: u8, part: u8) -> Option<(String, u128)> {
    match year {
        2021 => match day {
            1 => Some(measure(|| year2021::day01::Task.solve(year, day, part))),
            2 => Some(measure(|| year2021::day02::Task.solve(year, day, part))),
            3 => Some(measure(|| year2021::day03::Task.solve(year, day, part))),
            _ => None,
        },
        2020 => match day {
            1 => Some(measure(|| year2020::day01::Task.solve(year, day, part))),
            2 => Some(measure(|| year2020::day02::Task.solve(year, day, part))),
            3 => Some(measure(|| year2020::day03::Task.solve(year, day, part))),
            4 => Some(measure(|| year2020::day04::Task.solve(year, day, part))),
            5 => Some(measure(|| year2020::day05::Task.solve(year, day, part))),
            6 => Some(measure(|| year2020::day06::Task.solve(year, day, part))),
            7 => Some(measure(|| year2020::day07::Task.solve(year, day, part))),
            8 => Some(measure(|| year2020::day08::Task.solve(year, day, part))),
            9 => Some(measure(|| year2020::day09::Task.solve(year, day, part))),
            10 => Some(measure(|| year2020::day10::Task.solve(year, day, part))),
            11 => Some(measure(|| year2020::day11::Task.solve(year, day, part))),
            12 => Some(measure(|| year2020::day12::Task.solve(year, day, part))),
            13 => Some(measure(|| year2020::day13::Task.solve(year, day, part))),
            14 => Some(measure(|| year2020::day14::Task.solve(year, day, part))),
            15 => Some(measure(|| year2020::day15::Task.solve(year, day, part))),
            16 => Some(measure(|| year2020::day16::Task.solve(year, day, part))),
            17 => Some(measure(|| year2020::day17::Task.solve(year, day, part))),
            18 => Some(measure(|| year2020::day18::Task.solve(year, day, part))),
            19 => Some(measure(|| year2020::day19::Task.solve(year, day, part))),
            _ => None,
        },
        _ => None,
    }
}

fn measure(f: impl Fn() -> String) -> (String, u128) {
    let now = Instant::now();
    (f(), now.elapsed().as_millis())
}
