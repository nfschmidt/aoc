use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let input_parts = input.split_terminator("\n\n").collect::<Vec<_>>();

    // parse stacks
    let stacks_input = input_parts.get(0).unwrap();
    let mut stacks = parse_stacks(&stacks_input);
    let mut stacks2 = parse_stacks(&stacks_input);

    // parse moves
    let move_inputs = input_parts.get(1).unwrap();

    println!("{}", part1(&mut stacks, &move_inputs));
    println!("{}", part2(&mut stacks2, &move_inputs));
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let stack_lines = input.lines().collect::<Vec<_>>();
    let mut stack_lines = stack_lines.iter().rev();
    let stacks_count = stack_lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u8>()
        .unwrap();

    let mut stacks = Vec::new();
    for _ in 0..stacks_count {
        stacks.push(Vec::new());
    }

    for line in stack_lines {
        for (i, c) in line.chars().enumerate() {
            if c != ' ' && i == (i / 4) * 4 + 1 {
                stacks.get_mut(i / 4).unwrap().push(c);
            }
        }
    }

    stacks
}

fn part1(stacks: &mut Vec<Vec<char>>, input: &str) -> String {
    for line in input.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let amount = fields.get(1).unwrap().parse::<usize>().unwrap();
        let from = fields.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = fields.get(5).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let crt = stacks.get_mut(from).unwrap().pop().unwrap();
            stacks.get_mut(to).unwrap().push(crt);
        }
    }

    stacks
        .iter()
        .fold(String::new(), |acc, stack| match stack.last() {
            Some(c) => format!("{}{}", acc, c),
            None => acc,
        })
}

fn part2(stacks: &mut Vec<Vec<char>>, input: &str) -> String {
    for line in input.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let amount = fields.get(1).unwrap().parse::<usize>().unwrap();
        let from = fields.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = fields.get(5).unwrap().parse::<usize>().unwrap() - 1;

        let mut temp_stack = Vec::new();
        for _ in 0..amount {
            let crt = stacks.get_mut(from).unwrap().pop().unwrap();
            temp_stack.push(crt);
        }

        temp_stack.reverse();
        stacks.get_mut(to).unwrap().append(&mut temp_stack);
    }

    stacks
        .iter()
        .fold(String::new(), |acc, stack| match stack.last() {
            Some(c) => format!("{}{}", acc, c),
            None => acc,
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_stacks() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

        let got = parse_stacks(&input);
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        assert_eq!(got, expected);
    }

    #[test]
    fn test_part1() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let input = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let got = part1(&mut stacks, &input);
        let expected = "CMZ";

        assert_eq!(got, expected);
    }

    #[test]
    fn test_part2() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let input = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let got = part2(&mut stacks, &input);
        let expected = "MCD";

        assert_eq!(got, expected);
    }
}
