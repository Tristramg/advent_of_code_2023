use nom::bytes::complete::tag;
use nom::character::complete::{newline, one_of};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::many1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

fn int(input: &str) -> IResult<&str, usize> {
    map_res(
        preceded(opt(tag(" ")), recognize(many1(one_of("0123456789")))),
        |s: &str| s.parse(),
    )(input)
}

fn int_list(input: &str) -> IResult<&str, Vec<usize>> {
    many1(int)(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(tag("seeds:"), terminated(int_list, many1(newline)))(input)
}

#[derive(Debug, PartialEq)]
struct RangedMapping {
    source: usize,
    destination: usize,
    length: usize,
}

impl RangedMapping {
    fn location(&self, pos: usize) -> Option<usize> {
        (pos >= self.source && pos < self.source + self.length)
            .then(|| self.destination + (pos - self.source))
    }
}

fn find(ranges: &Vec<RangedMapping>, pos: usize) -> usize {
    ranges
        .iter()
        .filter_map(|r| r.location(pos))
        .next()
        .unwrap_or(pos)
}

fn range(input: &str) -> IResult<&str, RangedMapping> {
    map(tuple((int, int, int)), |(destination, source, length)| {
        RangedMapping {
            source,
            destination,
            length,
        }
    })(input)
}

fn mapping<'a>(key: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<RangedMapping>> {
    let header = preceded(terminated(tag(key), tag(" map:")), newline);
    preceded(
        header,
        terminated(many1(terminated(range, newline)), opt(newline)),
    )
}

struct Data {
    seeds: Vec<usize>,
    to_soil: Vec<RangedMapping>,
    to_fertilizer: Vec<RangedMapping>,
    to_water: Vec<RangedMapping>,
    to_light: Vec<RangedMapping>,
    to_temperature: Vec<RangedMapping>,
    to_humidity: Vec<RangedMapping>,
    to_location: Vec<RangedMapping>,
}

impl Data {
    fn from(input: &str) -> Self {
        map(
            tuple((
                seeds,
                mapping("seed-to-soil"),
                mapping("soil-to-fertilizer"),
                mapping("fertilizer-to-water"),
                mapping("water-to-light"),
                mapping("light-to-temperature"),
                mapping("temperature-to-humidity"),
                mapping("humidity-to-location"),
            )),
            |(seeds, a, b, c, d, e, f, g)| Data {
                seeds,
                to_soil: a,
                to_fertilizer: b,
                to_water: c,
                to_light: d,
                to_temperature: e,
                to_humidity: f,
                to_location: g,
            },
        )(input)
        .unwrap()
        .1
    }

    fn location(&self, seed: usize) -> usize {
        let soil = find(&self.to_soil, seed);
        let fertilizer = find(&self.to_fertilizer, soil);
        let water = find(&self.to_water, fertilizer);
        let light = find(&self.to_light, water);
        let temperature = find(&self.to_temperature, light);
        let humidity = find(&self.to_humidity, temperature);
        let location = find(&self.to_location, humidity);
        location
    }
}

fn run1(input: &str) -> usize {
    let data = Data::from(input);
    let destinations = data.seeds.iter().map(|seed| (&data).location(seed.clone()));
    destinations.min().unwrap()
}

fn main() {
    let contents = std::fs::read_to_string("inputs/day_5").expect("could not read input");
    println!("part1: {}, part2: {}", run1(&contents), "meh");
}

#[test]
fn test_star1() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    assert_eq!(35, run1(input));
    //assert_eq!(46, run2(input));
}

#[test]
fn test_parse_int() {
    assert_eq!(int(" 23"), Ok(("", 23)));
}

#[test]
fn test_parse_int_list() {
    assert_eq!(int_list("23 12 14"), Ok(("", vec![23, 12, 14])));
}

#[test]
fn test_parse_seed_list() {
    assert_eq!(
        seeds("seeds: 79 14 55 13\n"),
        Ok(("", vec![79, 14, 55, 13]))
    );
}

#[test]
fn test_parse_mapping() {
    let input = "seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:";

    let expected = vec![
        RangedMapping {
            source: 98,
            destination: 50,
            length: 2,
        },
        RangedMapping {
            source: 50,
            destination: 52,
            length: 48,
        },
    ];
    assert_eq!(mapping("seed-to-soil")(input).unwrap().1, expected);
}

#[test]
fn test_range_location() {
    let r = RangedMapping {
        source: 10,
        destination: 15,
        length: 10,
    };
    assert_eq!(r.location(10), Some(15));
    assert_eq!(r.location(11), Some(16));
    assert_eq!(r.location(9), None);
    assert_eq!(r.location(19), Some(24));
    assert_eq!(r.location(20), None);
}
