use std::collections::VecDeque;
use std::ops::Range;

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

#[derive(Debug)]
struct MapEntry {
    dst: u64,
    src: u64,
    len: u64,
}

fn parse_seeds(input: &'static str) -> impl Iterator<Item = u64> {
    let (seeds, _) = input.split_once('\n').unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    seeds.split_whitespace().map(|s| s.parse().unwrap())
}

fn parse_map(input: &'static str) -> Map {
    let (_, entries) = input.split_once(" map:\n").unwrap();
    let entries = entries
        .trim()
        .split('\n')
        .map(|entry| {
            let mut values = entry.split_whitespace().map(|s| s.parse::<u64>().unwrap());
            MapEntry {
                dst: values.next().unwrap(),
                src: values.next().unwrap(),
                len: values.next().unwrap(),
            }
        })
        .collect();

    Map { entries }
}

fn parse(input: &'static str) -> Input {
    let seeds = parse_seeds(input).collect();

    let maps = input
        .split("\n\n")
        .skip(1)
        .map(parse_map)
        .collect::<Vec<_>>();

    Input { seeds, maps }
}

impl Input {
    fn seeds_as_ranges(&self) -> impl Iterator<Item = Range<u64>> + '_ {
        self.seeds
            .chunks(2)
            .map(|chunks| chunks[0]..chunks[0] + chunks[1])
    }
}

impl MapEntry {
    fn src_range(&self) -> Range<u64> {
        self.src..self.src + self.len
    }

    fn map_value(&self, value: u64) -> u64 {
        assert!(self.src_range().contains(&value));

        let offset = value - self.src;
        self.dst + offset
    }

    fn map_range(&self, range: &Range<u64>) -> Range<u64> {
        assert!(self.src_range().start <= range.start);
        assert!(self.src_range().end >= range.end);

        let start = self.map_value(range.start);
        let len = range.end - range.start;

        start..start + len
    }
}

fn map_value(map: &Map, value: u64) -> u64 {
    map.entries
        .iter()
        .find(|entry| entry.src_range().contains(&value))
        .map(|entry| entry.map_value(value))
        .unwrap_or(value)
}

fn overlap(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);

    if start < end {
        Some(start..end)
    } else {
        None
    }
}

fn split_range(a: &Range<u64>, b: &Range<u64>) -> (Option<Range<u64>>, Option<Range<u64>>) {
    let overlap = overlap(a, b).expect("no overlap");

    let full_range = a.start.min(b.start)..a.end.max(b.end);
    let left = (full_range.start < overlap.start).then_some(full_range.start..overlap.start);
    let right = (full_range.end > overlap.end).then_some(overlap.end..full_range.end);
    (left, right)
}

fn map_values(map: &Map, values: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut inputs = VecDeque::from(values);
    let mut outputs = Vec::new();

    while let Some(next) = inputs.pop_front() {
        let mapped = map.entries.iter().find_map(|entry| {
            let src = entry.src_range();
            if let Some(overlap) = overlap(&next, &src) {
                if overlap != next {
                    let (left, right) = split_range(&next, &overlap);
                    if let Some(left) = left {
                        inputs.push_front(left);
                    }
                    if let Some(right) = right {
                        inputs.push_front(right);
                    }
                }

                return Some(entry.map_range(&overlap));
            };
            None
        });

        outputs.push(mapped.unwrap_or(next));
    }

    outputs
}

fn part_1(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .map(|seed| {
            input
                .maps
                .iter()
                .fold(*seed, |value, map| map_value(map, value))
        })
        .min()
        .expect("no minimum value found")
}

fn part_2(input: &Input) -> u64 {
    input
        .seeds_as_ranges()
        .map(|seeds| {
            input
                .maps
                .iter()
                .fold(vec![seeds], |values, map| map_values(map, values))
                .into_iter()
                .map(|range| range.start)
                .min()
                .unwrap()
        })
        .min()
        .expect("no minimum value found")
}

fn main() {
    let inputs = parse(include_str!("../input.txt"));

    println!("Part 1: {}", part_1(&inputs));
    println!("Part 2: {}", part_2(&inputs));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn overlaps() {
        let a = 0..5;
        let b = 3..8;
        let c = 0..3;
        let d = 5..8;
        let e = 7..10;

        assert_eq!(overlap(&a, &b), Some(3..5));
        assert_eq!(overlap(&a, &c), Some(0..3));
        assert_eq!(overlap(&a, &d), None);
        assert_eq!(overlap(&a, &e), None);

        assert_eq!(overlap(&b, &a), Some(3..5));
        assert_eq!(overlap(&b, &c), None);
        assert_eq!(overlap(&b, &d), Some(5..8));
        assert_eq!(overlap(&b, &e), Some(7..8));

        assert_eq!(overlap(&c, &a), Some(0..3));
        assert_eq!(overlap(&c, &b), None);
        assert_eq!(overlap(&c, &d), None);
        assert_eq!(overlap(&c, &e), None);

        assert_eq!(overlap(&d, &a), None);
        assert_eq!(overlap(&d, &b), Some(5..8));
        assert_eq!(overlap(&d, &c), None);
        assert_eq!(overlap(&d, &e), Some(7..8));
    }
}
