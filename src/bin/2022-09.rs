use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u16 {
    let mut head = (0i16, 0i16);
    let mut tail = (0i16, 0i16);

    let mut tail_visited = HashSet::new();
    tail_visited.insert(tail);

    for line in input.lines() {
        let mut fields = line.split_whitespace();
        let movement = fields.next().unwrap();
        let amount = fields.next().unwrap().parse::<u8>().unwrap();

        for _step in 0..amount {
            let new_head = match movement {
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                _ => panic!("unexpected line: {}", line),
            };

            if (new_head.0 - tail.0).abs() > 1 || (new_head.1 - tail.1).abs() > 1 {
                tail = head;
                tail_visited.insert(tail);
            }

            head = new_head;
        }
    }

    tail_visited.len() as u16
}

fn part2(input: &str) -> u16 {
    let mut head = (0i32, 0i32);
    let mut string = Vec::from([head; 10]);

    let mut tail_visited = HashSet::new();
    tail_visited.insert((0, 0));

    for line in input.lines() {
        let mut fields = line.split_whitespace();
        let movement = fields.next().unwrap();
        let amount = fields.next().unwrap().parse::<u8>().unwrap();

        for _step in 0..amount {
            head = match movement {
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                _ => panic!("unexpected line: {}", line),
            };

            string[0] = head;
            let mut prev_knot = head;

            for knot in string.iter_mut().skip(1) {
                let h_diff = prev_knot.0 - knot.0;
                let v_diff = prev_knot.1 - knot.1;

                match (h_diff, v_diff) {
                    (2, 0) => knot.0 += 1,
                    (-2, 0) => knot.0 -= 1,
                    (0, 2) => knot.1 += 1,
                    (0, -2) => knot.1 -= 1,

                    (-1, 2) => *knot = (knot.0 - 1, knot.1 + 1),
                    (-1, -2) => *knot = (knot.0 - 1, knot.1 - 1),
                    (1, 2) => *knot = (knot.0 + 1, knot.1 + 1),
                    (1, -2) => *knot = (knot.0 + 1, knot.1 - 1),

                    (-2, 1) => *knot = (knot.0 - 1, knot.1 + 1),
                    (-2, -1) => *knot = (knot.0 - 1, knot.1 - 1),
                    (2, 1) => *knot = (knot.0 + 1, knot.1 + 1),
                    (2, -1) => *knot = (knot.0 + 1, knot.1 - 1),

                    (-2, 2) => *knot = (knot.0 - 1, knot.1 + 1),
                    (-2, -2) => *knot = (knot.0 - 1, knot.1 - 1),
                    (2, 2) => *knot = (knot.0 + 1, knot.1 + 1),
                    (2, -2) => *knot = (knot.0 + 1, knot.1 - 1),

                    _ => {}
                }

                prev_knot = *knot;
            }

            tail_visited.insert(*string.last().unwrap());
        }
    }

    tail_visited.len() as u16
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_case1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2_case1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_case2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(part2(&input), 36);
    }
}
