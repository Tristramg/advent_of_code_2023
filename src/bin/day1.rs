fn parse(line: &str) -> i32 {
    let mut words_to_int = std::collections::HashMap::new();
    words_to_int.insert("one", 1);
    words_to_int.insert("two", 2);
    words_to_int.insert("three", 3);
    words_to_int.insert("four", 4);
    words_to_int.insert("five", 5);
    words_to_int.insert("six", 6);
    words_to_int.insert("seven", 7);
    words_to_int.insert("eight", 8);
    words_to_int.insert("nine", 9);
    let mut digits = vec![];
    for i in 0..line.len() {
        if let Some(d) = line.get(i..i + 1).and_then(|d| d.parse::<i32>().ok()) {
            digits.push(d)
        } else if let Some(d) = line.get(i..i + 5).and_then(|d| words_to_int.get(d)) {
            digits.push(*d)
        } else if let Some(d) = line.get(i..i + 4).and_then(|d| words_to_int.get(d)) {
            digits.push(*d)
        } else if let Some(d) = line.get(i..i + 3).and_then(|d| words_to_int.get(d)) {
            digits.push(*d)
        }
    }

    let decimal = digits.first().expect("missing first digit");
    let unit = digits.last().expect("missing last digit");
    decimal * 10 + unit
}

fn run(input: String) -> i32 {
    input.lines().map(parse).sum()
}
fn main() {
    let contents = std::fs::read_to_string("inputs/day_1").expect("could not read input");
    println!("{}", run(contents));
}

#[test]
fn test_input() {
    let input = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#
        .to_owned();
    assert_eq!(281, run(input))
}
