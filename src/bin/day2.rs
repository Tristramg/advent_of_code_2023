use regex::Regex;

// Building a regex everytime should be expensive. But input is tiny.
fn get_color(part: &str, color: &str) -> i32 {
    let re = Regex::new(&format!(" (\\d+) {color}")).expect("invalid regex");
    re.captures(part)
        .and_then(|capture| {
            capture
                .get(1)
                .map(|t| t.as_str().parse().expect("invalid number"))
        })
        .unwrap_or_default()
}

fn max_dices(game_str: &str) -> (i32, (i32, i32, i32)) {
    let parts = game_str.split(':').collect::<Vec<_>>();
    assert!(parts.len() == 2);
    assert!(parts[0].starts_with("Game "));
    let game: i32 = parts[0]
        .replace("Game ", "")
        .parse()
        .expect("Could not parse game number");

    (
        game,
        parts[1].split(';').fold((0, 0, 0), |(r, g, b), draft| {
            (
                r.max(get_color(draft, "red")),
                g.max(get_color(draft, "green")),
                b.max(get_color(draft, "blue")),
            )
        }),
    )
}

fn run1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (game, (r, g, b)) = max_dices(line);
            if r <= 12 && g <= 13 && b <= 14 {
                game
            } else {
                0
            }
        })
        .sum()
}

fn run2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (_game, (r, g, b)) = max_dices(line);
            r * g * b
        })
        .sum()
}

fn main() {
    let contents = std::fs::read_to_string("inputs/day_2").expect("could not read input");
    println!("part1: {}, part2: {}", run1(&contents), run2(&contents));
}

#[test]
fn test_star1() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(8, run1(input))
}

#[test]
fn test_star2() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(2286, run2(input))
}
