fn handle_line(line: &str) -> i32 {
    let mut data: Vec<_> = line.split(" ").map(|i| i.parse::<i32>().unwrap()).collect();
    let mut result = 0;
    while data.iter().any(|i| *i != 0) {
        result += data.last().unwrap();
        data = data.windows(2).map(|w| w[1] - w[0]).collect();
    }
    result
}
fn handle_line2(line: &str) -> i32 {
    let mut data: Vec<_> = line.split(" ").map(|i| i.parse::<i32>().unwrap()).collect();
    let mut result = *data.first().unwrap();
    let mut count = 0;
    while data.iter().any(|i| *i != 0) {
        data = data.windows(2).map(|w| w[1] - w[0]).collect();
        result = data.first().unwrap() - result;
        count += 1;
    }
    if count % 2 == 0 {
        result
    } else {
        -result
    }
}
fn run1(input: &str) -> i32 {
    input.lines().map(handle_line).sum()
}

fn run2(input: &str) -> i32 {
    input.lines().map(handle_line2).sum()
}

fn main() {
    let contents = std::fs::read_to_string("inputs/day_9").expect("could not read input");
    println!("part1: {}, part2: {}", run1(&contents), run2(&contents));
}

#[test]
fn test_line() {
    assert_eq!(18, handle_line("0 3 6 9 12 15"));
}

#[test]
fn test_star1() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(114, run1(input))
}

#[test]
fn test_line2() {
    assert_eq!(-3, handle_line2("0 3 6 9 12 15"));
    assert_eq!(0, handle_line2("1 3 6 10 15 21"));
    assert_eq!(5, handle_line2("10 13 16 21 30 45"));
}

#[test]
fn test_star2() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(2, run2(input))
}
