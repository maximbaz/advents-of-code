use regex::Regex;
use std::fs;

pub fn run_part1() -> usize {
    part1(&input(&read_file()))
}

pub fn run_part2() -> usize {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("input/day2.txt").expect("Error reading the file")
}

struct PasswordWithPolicy<'a> {
    password: &'a str,
    letter: char,
    from: usize,
    to: usize,
}

fn input<'a>(string: &'a str) -> Vec<PasswordWithPolicy> {
    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();

    string
        .trim()
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            PasswordWithPolicy {
                from: caps.get(1).unwrap().as_str().parse().unwrap(),
                to: caps.get(2).unwrap().as_str().parse().unwrap(),
                letter: caps.get(3).unwrap().as_str().chars().next().unwrap(),
                password: caps.get(4).unwrap().as_str(),
            }
        })
        .collect()
}

fn part1(input: &Vec<PasswordWithPolicy>) -> usize {
    input
        .iter()
        .filter(|p| (p.from..(p.to + 1)).contains(&p.password.matches(p.letter).count()))
        .count()
}

fn part2(input: &Vec<PasswordWithPolicy>) -> usize {
    input
        .iter()
        .filter(|p| {
            let first = p.password.chars().nth(p.from - 1) == Some(p.letter);
            let second = p.password.chars().nth(p.to - 1) == Some(p.letter);
            (first || second) && !(first && second)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            part1(&input(
                &"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
            "
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            1,
            part2(&input(
                &"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
            "
            ))
        );
    }
}
