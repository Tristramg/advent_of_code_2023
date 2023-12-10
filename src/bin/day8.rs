use regex::Regex;
use std::collections::HashMap;

fn parse_node(line: &str) -> (String, (String, String)) {
    let re = Regex::new(r"(...) = \((...), (...)\)").expect("could not compile regex");
    let captures = re.captures(line).unwrap();
    (
        captures[1].to_string(),
        (captures[2].to_string(), captures[3].to_string()),
    )
}

fn parse(input: &str) -> (&[u8], HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().as_bytes();
    lines.next().unwrap();
    (instructions, lines.map(parse_node).collect())
}

fn run1(input: &str) -> usize {
    let (instructions, graph) = parse(input);
    let mut current = "AAA";
    let mut steps = 0;
    while current != "ZZZ" {
        let instruction = instructions[steps % instructions.len()];
        steps += 1;
        let (left, right) = graph.get(current).unwrap();
        current = match instruction {
            b'L' => left,
            b'R' => right,
            _ => unreachable!(),
        }
    }
    steps
}

fn run2(input: &str) -> usize {
    let (instructions, graph) = parse(input);

    let node_to_int: HashMap<String, usize> = graph
        .iter()
        .enumerate()
        .map(|(i, val)| (val.0.clone(), i))
        .collect();

    let is_end_node: Vec<_> = graph.iter().map(|(k, _)| k.ends_with("Z")).collect();
    let succ_right: Vec<_> = graph
        .values()
        .map(|(_l, r)| node_to_int.get(r).unwrap())
        .collect();
    let succ_left: Vec<_> = graph
        .values()
        .map(|(l, _r)| node_to_int.get(l).unwrap())
        .collect();

    for start in graph
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| node_to_int.get(k).unwrap())
    {
        let mut loop_detector = HashMap::new();
        let mut steps = 0;
        let mut current_ = (*start, steps);
        loop {
            let instruction_position = steps % instructions.len();
            let instruction = instructions[instruction_position];
            let next = match instruction {
                b'L' => succ_left[current_.0],
                b'R' => succ_right[current_.0],
                _ => unreachable!(),
            };
            steps += 1;
            current_ = (*next, instruction_position);
            if loop_detector.contains_key(&current_) {
                println!(
                    "Offset = {}, loop = {}",
                    loop_detector[&current_],
                    steps - loop_detector[&current_]
                );
                break;
            }
            if is_end_node[*next] {
                //println!("Current_: {:?} at step {}", current_, steps);
                loop_detector.insert(current_, steps);
            }
        }
    }

    /*while !current.iter().all(|k| is_end_node[**k]) {
        let instruction = instructions[steps % instructions.len()];
        for i in 0..current.len() {
            if is_end_node[i] {
                println!("End node for {i} at {}", steps % instructions.len());
            }
        }
        steps += 1;
        if steps % 10_000_000 == 0 {
            dbg!(steps / 1_000_000);
        };
        current = current
            .iter()
            .map(|k| match instruction {
                b'L' => succ_left[**k],
                b'R' => succ_right[**k],
                _ => unreachable!(),
            })
            .collect()
    }*/
    0
}

fn main() {
    let contents = std::fs::read_to_string("inputs/day_8").expect("could not read input");
    println!("part1: {}, part2: {}", run1(&contents), run2(&contents));
}

#[test]
fn test_star1() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(2, run1(input));

    let input = "LLR
    
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(6, run1(input));
}

#[test]
fn test_star2() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    assert_eq!(0, run2(input))
}
