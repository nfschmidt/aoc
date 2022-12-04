use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let middle = line.len() / 2;
        let first_rucksack = line.chars().take(middle).collect::<HashSet<_>>();

        for item in line.chars().skip(middle) {
            if first_rucksack.contains(&item) {
                result += if item <= 'Z' {
                    item as u64 - 'A' as u64 + 27
                } else {
                    item as u64 - 'a' as u64 + 1
                };
                break;
            }
        }
    }

    result
}

fn part2(input: &str) -> u64 {
    let first_elves = input.lines().step_by(3);
    let second_elves = input.lines().skip(1).step_by(3);
    let third_elves = input.lines().skip(2).step_by(3);

    let elf_groups = first_elves.zip(second_elves).zip(third_elves);

    let mut result = 0;
    for ((r1, r2), r3) in elf_groups {
        let first_rucksack = r1.chars().collect::<HashSet<_>>();
        let second_rucksack = r2.chars().collect::<HashSet<_>>();
        let third_rucksack = r3.chars().collect::<HashSet<_>>();

        for item in first_rucksack {
            if second_rucksack.contains(&item) && third_rucksack.contains(&item) {
                result += if item <= 'Z' {
                    item as u64 - 'A' as u64 + 27
                } else {
                    item as u64 - 'a' as u64 + 1
                };
                break;
            }
        }
    }

    result
}
