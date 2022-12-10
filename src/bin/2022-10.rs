use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let values = register_values(input);
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];

    interesting_cycles
        .iter()
        .map(|c| values[c - 1] * *c as i32)
        .sum()
}

fn part2(input: &str) -> String {
    let values = register_values(input);
    let mut screen = String::new();

    for (cycle, &register) in values.iter().take(240).enumerate() {
        if cycle > 0 && cycle % 40 == 0 {
            screen.push('\n');
        }

        let position = cycle as i32 % 40;

        let is_lit = register - 1 <= position && position <= register + 1;
        screen.push(if is_lit { '#' } else { '.' })
    }

    screen
}

fn register_values(instructions: &str) -> Vec<i32> {
    let mut result = Vec::new();

    let mut register = 1;
    result.push(register);

    for instruction in instructions.lines() {
        let mut fields = instruction.split_whitespace();
        let op = fields.next().unwrap();

        match op {
            "noop" => {}
            "addx" => {
                let op_value = fields.next().unwrap().parse::<i32>().unwrap();
                result.push(register);

                register += op_value;
            }
            _ => panic!("unexpected instruction: {}", instruction),
        }

        result.push(register);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> String {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            .to_string()
    }

    #[test]
    fn part1_case1() {
        assert_eq!(part1(&input()), 13140);
    }

    #[test]
    fn part2_case1() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_string();
        assert_eq!(part2(&input()), expected);
    }
}
