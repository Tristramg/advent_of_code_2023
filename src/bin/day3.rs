use regex::Regex;
#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
    value: String,
}

impl Symbol {
    fn parse_line((y, line): (usize, &str)) -> Vec<Symbol> {
        Regex::new(r"([^\d\.])")
            .expect("could not compile regex")
            .find_iter(line)
            .map(|m| Symbol {
                x: m.start(),
                y,
                value: m.as_str().to_owned(),
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
struct Number {
    x_min: usize, // position of leftest digit
    x_max: usize, // position of the rightest digit
    y: usize,
    value: i32,
}

impl Number {
    fn parse_line((y, line): (usize, &str)) -> Vec<Number> {
        let re = Regex::new(r"(\d+)").expect("could not compile regex");
        re.find_iter(line)
            .map(|m| Number {
                x_min: m.start(),
                x_max: m.end() - 1, // end is the byte after the last one
                y,
                value: m.as_str().parse().expect("a number"),
            })
            .collect()
    }

    fn touches(self, symbol: &Symbol) -> bool {
        self.y <= symbol.y + 1
            && symbol.y <= self.y + 1
            && self.x_min <= symbol.x + 1
            && symbol.x <= self.x_max + 1
    }
}

fn numbers(input: &str) -> Vec<Number> {
    input
        .lines()
        .enumerate()
        .map(Number::parse_line)
        .flatten()
        .collect()
}

fn symbols(input: &str) -> Vec<Symbol> {
    input
        .lines()
        .enumerate()
        .map(Symbol::parse_line)
        .flatten()
        .collect()
}

fn run1(input: &str) -> i32 {
    let symbols = symbols(input);

    numbers(input)
        .iter()
        .filter(|n| symbols.iter().any(|symbol| n.touches(symbol)))
        .map(|n| n.value)
        .sum()
}

fn run2(input: &str) -> i32 {
    let numbers = numbers(input);
    symbols(input)
        .iter()
        .filter(|s| s.value == "*")
        .map(|s| numbers.iter().filter(|n| n.touches(s)).collect::<Vec<_>>())
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0].value * numbers[1].value)
        .sum()
}

fn main() {
    let contents = std::fs::read_to_string("inputs/day_3").expect("could not read input");
    println!("part1: {}, part2: {}", run1(&contents), run2(&contents));
}

#[test]
fn test_star1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    assert_eq!(4361, run1(input))
}

#[test]
fn test_star2() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(467835, run2(input))
}
