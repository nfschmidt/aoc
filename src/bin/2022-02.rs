use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();


    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut total_score = 0;
    for line in input.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        match fields[0] {
            "A" => match fields[1] {
                "X" => total_score += 1 + 3,
                "Y" => total_score += 2 + 6,
                "Z" => total_score += 3 + 0,
                x => panic!("invalid input: {}", x),
            },
            "B" => match fields[1] {
                "X" => total_score += 1 + 0,
                "Y" => total_score += 2 + 3,
                "Z" => total_score += 3 + 6,
                x => panic!("invalid input: {}", x),
            },
            "C" => match fields[1] {
                "X" => total_score += 1 + 6,
                "Y" => total_score += 2 + 0,
                "Z" => total_score += 3 + 3,
                x => panic!("invalid input: {}", x),
            },
            x => panic!("invalid input: {}", x),
        }
    }

    total_score
}

fn part2(input: &str) -> u64 {
    let mut total_score = 0;
    for line in input.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        match fields[0] {
            "A" => match fields[1] {
                "X" => total_score += 3 + 0,
                "Y" => total_score += 1 + 3,
                "Z" => total_score += 2 + 6,
                x => panic!("invalid input: {}", x),
            },
            "B" => match fields[1] {
                "X" => total_score += 1 + 0,
                "Y" => total_score += 2 + 3,
                "Z" => total_score += 3 + 6,
                x => panic!("invalid input: {}", x),
            },
            "C" => match fields[1] {
                "X" => total_score += 2 + 0,
                "Y" => total_score += 3 + 3,
                "Z" => total_score += 1 + 6,
                x => panic!("invalid input: {}", x),
            },
            x => panic!("invalid input: {}", x),
        }
    }

    total_score
}
