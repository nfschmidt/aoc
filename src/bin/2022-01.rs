use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let part1_result = part1(&input);
    let part2_result = part2(&input);

    println!("{}", part1_result);
    println!("{}", part2_result);
}

fn part1(input: &str) -> u64 {
    let mut max = 0;
    let mut current = 0;

    for line in input.lines() {
        let content = line.trim_end();

        if content == "" {
            if current > max {
                max = current;
            }

            current = 0;
        } else {
            current += content.parse::<u64>().unwrap();
        }
    }

    if current > max {
        max = current;
    }

    return max;
}

fn part2(input: &str) -> u64 {
    let (mut max, mut max2, mut max3) = (0, 0, 0);
    let mut current = 0;

    for line in input.lines() {
        let content = line.trim_end();

        if content == "" {
            if current > max {
                (max, max2, max3) = (current, max, max2);
            } else if current > max2 {
                (max2, max3) = (current, max2);
            } else if current > max3 {
                max3 = current;
            }

            current = 0;
        } else {
            current += content.parse::<u64>().unwrap();
        }
    }

    if current > max {
        (max, max2, max3) = (current, max, max2);
    } else if current > max2 {
        (max2, max3) = (current, max2);
    } else if current > max3 {
        max3 = current;
    }

    return max + max2 + max3;
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 45000);
    }
}
