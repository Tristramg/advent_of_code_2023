use std::collections::HashSet;

fn to_hash_set(numbers: &str) -> HashSet<i32> {
    numbers
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

fn parse_line(line: &str) -> usize {
    let (_, b) = line.split_once(": ").unwrap();
    let (winning, i_have) = b.split_once(" | ").unwrap();
    to_hash_set(winning)
        .intersection(&to_hash_set(i_have))
        .count()
}
fn run1(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(|len| {
            if len < 2 {
                len
            } else {
                2_usize.pow(len as u32 - 1)
            }
        })
        .sum()
}

fn run2(input: &str) -> i32 {
    let num_lines = input.lines().count();
    let mut counts = vec![1; num_lines];
    input
        .lines()
        .map(parse_line)
        .enumerate()
        .for_each(|(position, count)| {
            for i in (position + 1)..(position + count + 1).min(num_lines) {
                counts[i] += counts[position];
            }
        });
    counts.iter().sum()
}

fn main() {
    let contents = std::fs::read_to_string("inputs/day_4").expect("could not read input");
    println!("part1: {}, part2: {}", run1(&contents), run2(&contents));
}

#[test]
fn test_star1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(13, run1(input));
}

#[test]
fn test_start2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(30, run2(input))
}
