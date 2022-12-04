use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut result = 0;

    for line in input.lines() {
        let ranges = line.split(',').collect::<Vec<_>>();
        let range1 = ranges.get(0).unwrap().split('-').collect::<Vec<_>>();
        let range1_start = range1.get(0).unwrap().parse::<u64>().unwrap();
        let range1_end = range1.get(1).unwrap().parse::<u64>().unwrap();

        let range2 = ranges.get(1).unwrap().split('-').collect::<Vec<_>>();
        let range2_start = range2.get(0).unwrap().parse::<u64>().unwrap();
        let range2_end = range2.get(1).unwrap().parse::<u64>().unwrap();

        if (range1_start >= range2_start && range1_end <= range2_end)
            || (range2_start >= range1_start && range2_end <= range1_end)
        {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> u64 {
    let mut result = 0;

    for line in input.lines() {
        let ranges = line.split(',').collect::<Vec<_>>();
        let range1 = ranges.get(0).unwrap().split('-').collect::<Vec<_>>();
        let range1_start = range1.get(0).unwrap().parse::<u64>().unwrap();
        let range1_end = range1.get(1).unwrap().parse::<u64>().unwrap();

        let range2 = ranges.get(1).unwrap().split('-').collect::<Vec<_>>();
        let range2_start = range2.get(0).unwrap().parse::<u64>().unwrap();
        let range2_end = range2.get(1).unwrap().parse::<u64>().unwrap();

        if (range1_start >= range2_start && range1_start <= range2_end)
            || (range1_end >= range2_start && range1_end <= range2_end)
            || (range2_start >= range1_start && range2_start <= range1_end)
            || (range2_end >= range1_start && range2_end <= range1_end)
        {
            result += 1;
        }
    }

    result
}
