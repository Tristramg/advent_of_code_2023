use std::collections::HashSet;

use nom::InputIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    fn delta(&self, dx: i32, dy: i32) -> Option<Self> {
        if (self.x == 0 && dx < 0) || (self.y == 0 && dy < 0) {
            None
        } else {
            Some(Self {
                x: (self.x as i32 + dx) as usize,
                y: (self.y as i32 + dy) as usize,
            })
        }
    }
    fn succ(&self, c: char) -> Vec<Node> {
        match c {
            '|' => vec![self.delta(0, -1), self.delta(0, 1)],
            '-' => vec![self.delta(-1, 0), self.delta(1, 0)],
            'L' => vec![self.delta(0, -1), self.delta(1, 0)],
            'J' => vec![self.delta(0, -1), self.delta(-1, 0)],
            '7' => vec![self.delta(-1, 0), self.delta(0, 1)],
            'F' => vec![self.delta(0, 1), self.delta(1, 0)],
            '.' => vec![],
            'S' => vec![
                self.delta(-1, 0),
                self.delta(1, 0),
                self.delta(0, -1),
                self.delta(0, 1),
            ],
            _ => unreachable!(),
        }
        .iter()
        .filter_map(|c| *c)
        .collect()
    }
}

struct Graph {
    source: Node,
    succ: Vec<Vec<Vec<Node>>>,
    input: Vec<Vec<char>>,
}

impl Graph {
    fn from_line((y, line): (usize, &str)) -> Vec<Vec<Node>> {
        line.iter_elements()
            .enumerate()
            .map(|(x, c)| Node { x, y }.succ(c))
            .collect()
    }
    fn from(input: &str) -> Self {
        let (x, y, _c) = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter_elements()
                    .enumerate()
                    .map(move |(x, c)| (x, y, c))
            })
            .find(|(_x, _y, c)| *c == 'S')
            .unwrap();
        let source = Node { x, y };
        let succ = input.lines().enumerate().map(Graph::from_line).collect();
        let input = input.lines().map(|l| l.iter_elements().collect()).collect();
        Self {
            source,
            succ,
            input,
        }
    }

    fn valid_succ(&self, n: Node) -> Vec<Node> {
        let ymax = self.succ.len();
        let xmax = self.succ[0].len();
        self.succ[n.y][n.x]
            .iter()
            .filter(|n| n.x < xmax && n.y < ymax)
            .map(|c| *c)
            .collect()
    }

    fn search(&self) -> i32 {
        dbg!(self.source);
        // First find a node that actually leaves from the start
        let mut current = self
            .valid_succ(self.source)
            .iter()
            .filter(|s| self.valid_succ(**s).iter().any(|s| *s == self.source))
            .next()
            .expect("could not start")
            .clone();

        let mut distance = 0;
        let mut visited = HashSet::<Node>::new();

        while current != self.source {
            visited.insert(current);
            current = self
                .valid_succ(current)
                .iter()
                .filter(|s| !visited.contains(*s))
                .filter(|s| **s != self.source || distance > 0)
                .next()
                .unwrap()
                .clone();
            distance += 1;
        }

        // Print a cleaned path with only relevant pipes. Stored with a shell pipe in day_10_cleaned
        for y in 0..self.input.len() {
            for x in 0..self.input[y].len() {
                if (Node { x, y }) == self.source {
                    print!("S");
                } else if visited.contains(&Node { x, y }) {
                    print!("{}", self.input[y][x])
                } else {
                    print!(".")
                }
            }
            println!();
        }

        (distance + 1) / 2
    }
}

fn insides(line: &str) -> i32 {
    let mut outside = true;
    let mut count = 0;
    let mut entered_through = '.';
    for c in line.iter_elements() {
        match (c, entered_through) {
            ('-', _) => (),
            ('|', _) | ('J', 'F') | ('7', 'L') => {
                outside = !outside;
            }
            ('.', _) => {
                if !outside {
                    count = count + 1;
                }
            }
            ('L', _) => {
                entered_through = 'L';
            }
            ('F', _) => {
                entered_through = 'F';
            }
            ('7', 'F') | ('J', 'L') => (),
            a => {
                dbg!(a, line);
                unreachable!();
            }
        }
    }

    count
}

fn run2(input: &str) -> i32 {
    input.lines().map(insides).sum()
}

fn main() {
    let contents = std::fs::read_to_string("inputs/day_10").expect("could not read input");
    let g = Graph::from(&contents);
    let one = g.search();

    let contents =
        std::fs::read_to_string("inputs/day_10_cleaned.txt").expect("could not read input");
    let two = run2(&contents);

    println!("Day 10: first star: {one}; second star: {two}");
}

#[test]
fn test_simple() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";

    let g = Graph::from(input);
    assert_eq!(4, g.search());
}

#[test]
fn run_complex() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let g = Graph::from(input);
    assert_eq!(8, g.search());
}

#[test]
fn simple2() {
    let input = "...........
.F-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    assert_eq!(4, run2(input));
}
