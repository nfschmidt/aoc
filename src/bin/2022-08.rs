use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let mut forest = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        forest.push(row);
    }

    println!("{}", part1(&forest));
    println!("{}", part2(&forest));
}

fn part1(forest: &Vec<Vec<u32>>) -> u32 {
    let rows = forest.len();
    let columns = forest.get(0).unwrap().len();

    let mut visible_count = 0;

    for tree_row in 0..rows {
        for tree_col in 0..columns {
            let height = forest.get(tree_row).unwrap().get(tree_col).unwrap();

            let mut visible = true;
            for r in 0..tree_row {
                if forest.get(r).unwrap().get(tree_col).unwrap() >= height {
                    visible = false;
                }
            }

            if visible {
                visible_count += 1;
                continue;
            }

            visible = true;
            for r in (tree_row + 1)..rows {
                if forest.get(r).unwrap().get(tree_col).unwrap() >= height {
                    visible = false;
                }
            }

            if visible {
                visible_count += 1;
                continue;
            }

            visible = true;
            for c in 0..tree_col {
                if forest.get(tree_row).unwrap().get(c).unwrap() >= height {
                    visible = false;
                }
            }

            if visible {
                visible_count += 1;
                continue;
            }

            visible = true;
            for c in (tree_col + 1)..columns {
                if forest.get(tree_row).unwrap().get(c).unwrap() >= height {
                    visible = false;
                }
            }

            if visible {
                visible_count += 1;
                continue;
            }
        }
    }

    visible_count
}

fn part2(forest: &Vec<Vec<u32>>) -> u64 {
    let rows = forest.len();
    let columns = forest.get(0).unwrap().len();

    let mut scores = Vec::new();

    for tree_row in 0..rows {
        for tree_col in 0..columns {
            let mut score = 1;

            let height = forest.get(tree_row).unwrap().get(tree_col).unwrap();

            let mut count = 0;
            for r in (0..tree_row).rev() {
                count += 1;
                if forest.get(r).unwrap().get(tree_col).unwrap() >= height {
                    break;
                }
            }
            score *= count;

            count = 0;
            for r in tree_row + 1..rows {
                count += 1;
                if forest.get(r).unwrap().get(tree_col).unwrap() >= height {
                    break;
                }
            }
            score *= count;

            count = 0;
            for c in (0..tree_col).rev() {
                count += 1;
                if forest.get(tree_row).unwrap().get(c).unwrap() >= height {
                    break;
                }
            }
            score *= count;

            count = 0;
            for c in tree_col + 1..columns {
                count += 1;
                if forest.get(tree_row).unwrap().get(c).unwrap() >= height {
                    break;
                }
            }
            score *= count;

            scores.push(score);
        }
    }

    scores.sort();
    *scores.last().unwrap()
}
