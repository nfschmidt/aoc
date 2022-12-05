use std::{
    fmt::{Debug, Display},
    io::{stdin, Read},
    process::exit,
};

fn main() {
    let mut input = String::new();
    if let Err(e) = stdin().read_to_string(&mut input) {
        println!("could not read input: {}", e);
        exit(1);
    }

    let calories = get_ok_result(parse_calories_by_elf(&input));

    let part1_result = get_ok_result(part1(&calories));
    let part2_result = get_ok_result(part2(&calories));

    println!("{}\n{}", part1_result, part2_result);
}

fn get_ok_result<T>(result: Result<T>) -> T {
    match result {
        Ok(r) => return r,
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        }
    };
}

#[derive(PartialEq, Debug)]
enum Error {
    InvalidInput { line: usize },
    NotEnoughElves { needed: usize, got: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInput { line } => {
                write!(f, "could not parse input data on line {}", line)
            }
            Error::NotEnoughElves { needed, got } => {
                let pluralize = |&e| if e == 1 as usize { "elf" } else { "elves" };

                write!(
                    f,
                    "data for {} {} is needed, but got data for {} {}",
                    needed,
                    pluralize(needed),
                    got,
                    pluralize(got),
                )
            }
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

type Calorie = u64;

fn parse_calories_by_elf(input: &str) -> Result<Vec<Calorie>> {
    let mut calories = Vec::new();

    let mut current = None;
    for (i, line) in input.lines().enumerate() {
        let content = line.trim_end();

        if content == "" {
            if let Some(value) = current {
                calories.push(value);
                current = None;
            }
        } else {
            let num = content
                .parse::<u64>()
                .map_err(|_| Error::InvalidInput { line: i + 1 })?;
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

fn part1(calories: &[Calorie]) -> Result<Calorie> {
    Ok(*calories
        .iter()
        .max()
        .ok_or(Error::NotEnoughElves { needed: 1, got: 0 })?)
}

fn part2(calories: &[Calorie]) -> Result<Calorie> {
    if calories.len() < 3 {
        return Err(Error::NotEnoughElves {
            needed: 3,
            got: calories.len(),
        });
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
        assert_eq!(parse_calories_by_elf(input), expected);
    }

    #[test]
    fn input_parser_with_0_calories_returns_single_0_value() {
        let input = "0";
        let expected = Ok(vec![0]);
        assert_eq!(parse_calories_by_elf(input), expected);
    }

    #[test]
    fn input_parser_returns_error_if_u64_parsing_fails() {
        let input = "--invalid--";
        let expected = Err(Error::InvalidInput { line: 1 });
        assert_eq!(parse_calories_by_elf(input), expected);
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
        assert_eq!(part1(&[]), Err(Error::NotEnoughElves { needed: 1, got: 0 }));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), Ok(45000));
    }

    #[test]
    fn part2_fails_with_empty_list_of_calories() {
        assert_eq!(
            part2(&[2, 2]),
            Err(Error::NotEnoughElves { needed: 3, got: 2 })
        );
    }
}
