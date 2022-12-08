use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u16 {
    pos_distinct_chars(input, 4)
}

fn part2(input: &str) -> u16 {
    pos_distinct_chars(input, 14)
}

fn pos_distinct_chars(input: &str, len: usize) -> u16 {
    let mut last_chars: VecDeque<char> = VecDeque::from_iter(input.chars().take(len - 1));

    for (i, c) in input.chars().skip(len - 1).enumerate() {
        last_chars.push_back(c);
        let set: HashSet<&char> = last_chars.iter().collect();

        if set.len() == len {
            return i as u16 + len as u16;
        }

        last_chars.pop_front();
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_case1() {
        let got = part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let expected = 5;

        assert_eq!(got, expected);
    }

    #[test]
    fn part1_case2() {
        let got = part1("nppdvjthqldpwncqszvftbrmjlhg");
        let expected = 6;

        assert_eq!(got, expected);
    }

    #[test]
    fn part1_case3() {
        let got = part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let expected = 10;

        assert_eq!(got, expected);
    }

    #[test]
    fn part1_case4() {
        let got = part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let expected = 11;

        assert_eq!(got, expected);
    }

    #[test]
    fn part2_case1() {
        let got = part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let expected = 19;

        assert_eq!(got, expected);
    }

    #[test]
    fn part2_case2() {
        let got = part2("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let expected = 23;

        assert_eq!(got, expected);
    }

    #[test]
    fn part2_case3() {
        let got = part2("nppdvjthqldpwncqszvftbrmjlhg");
        let expected = 23;

        assert_eq!(got, expected);
    }

    #[test]
    fn part2_case4() {
        let got = part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let expected = 29;

        assert_eq!(got, expected);
    }

    #[test]
    fn part2_case5() {
        let got = part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let expected = 26;

        assert_eq!(got, expected);
    }
}
