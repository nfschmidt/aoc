use std::{
    error,
    fmt::{Debug, Display},
    io::{stdin, Read},
};

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let calories = string_to_total_calories(&input)?;

    println!("{}", part1(&calories)?);
    println!("{}", part2(&calories)?);

    Ok(())
}

#[derive(PartialEq, Debug)]
enum Error {
    InvalidInput,
    NotEnoughValues,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

fn string_to_total_calories(input: &str) -> Result<Vec<u64>> {
    let mut calories = Vec::new();

    let mut current = None;
    for line in input.lines() {
        let content = line.trim_end();

        if content == "" {
            if let Some(value) = current {
                calories.push(value);
                current = None;
            }
        } else {
            let num = content.parse::<u64>().map_err(|_| Error::InvalidInput)?;
            current = match current {
                Some(value) => Some(value + num),
                None => Some(num),
            }
        }
    }

    if let Some(value) = current {
        calories.push(value);
    }

    Ok(calories)
}

fn part1(calories: &[u64]) -> Result<u64> {
    Ok(*calories.iter().max().ok_or(Error::NotEnoughValues)?)
}

fn part2(calories: &[u64]) -> Result<u64> {
    if calories.len() < 3 {
        return Err(Error::NotEnoughValues);
    }

    let max = calories.iter().fold((0, 0, 0), |(m1, m2, m3), &e| {
        if e > m1 {
            (e, m1, m2)
        } else if e > m2 {
            (m1, e, m2)
        } else if e > m3 {
            (m1, m2, e)
        } else {
            (m1, m2, m3)
        }
    });

    Ok(max.0 + max.1 + max.2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_test_parser() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let expected = Ok(vec![6000, 4000, 11000, 24000, 10000]);
        assert_eq!(string_to_total_calories(input), expected);
    }

    #[test]
    fn input_parser_with_0_calories_returns_single_0_value() {
        let input = "0";
        let expected = Ok(vec![0]);
        assert_eq!(string_to_total_calories(input), expected);
    }

    #[test]
    fn input_parser_returns_error_if_u64_parsing_fails() {
        let input = "--invalid--";
        let expected = Err(Error::InvalidInput);
        assert_eq!(string_to_total_calories(input), expected);
    }

    fn input() -> Vec<u64> {
        vec![6000, 4000, 11000, 24000, 10000]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), Ok(24000));
    }

    #[test]
    fn part1_fails_with_empty_list_of_calories() {
        assert_eq!(part1(&[]), Err(Error::NotEnoughValues));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), Ok(45000));
    }

    #[test]
    fn part2_fails_with_empty_list_of_calories() {
        assert_eq!(part2(&[2, 2]), Err(Error::NotEnoughValues));
    }
}
