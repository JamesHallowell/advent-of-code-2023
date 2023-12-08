use std::collections::HashMap;

fn parse(
    input: &'static str,
) -> (
    impl Iterator<Item = Direction> + Clone,
    HashMap<&'static str, Node>,
) {
    let (instructions, network) = input.split_once("\n\n").unwrap();

    let instructions = instructions.chars().map(|char| match char {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    });

    let network = network.split('\n').filter(|&node| node != "").fold(
        HashMap::<&'static str, Node>::default(),
        |mut map, node| {
            let (key, value) = node.split_once(" = ").unwrap();

            let (left, right) = value
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(", ")
                .unwrap();

            map.insert(key, Node { left, right });
            map
        },
    );

    (instructions, network)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    left: &'static str,
    right: &'static str,
}

fn part_1(input: &'static str) -> i32 {
    let (instructions, network) = parse(input);
    let mut instructions = instructions.cycle();

    let mut node = network.get("AAA").unwrap();
    let mut steps = 0;
    loop {
        steps += 1;
        let next = match instructions.next().unwrap() {
            Direction::Left => node.left,
            Direction::Right => node.right,
        };

        if next == "ZZZ" {
            break;
        }

        node = network.get(next).unwrap();
    }

    steps
}

fn part_2(input: &'static str) -> u64 {
    let (instructions, network) = parse(input);
    let mut instructions = instructions.cycle();

    let starts = network
        .keys()
        .filter(|&key| key.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();

    let steps = starts
        .into_iter()
        .map(|start| {
            let mut steps = 0;

            let mut node = network.get(start).unwrap();
            loop {
                steps += 1;

                let next = match instructions.next().unwrap() {
                    Direction::Left => node.left,
                    Direction::Right => node.right,
                };

                if next.ends_with('Z') {
                    break;
                }

                node = network.get(next).unwrap();
            }
            steps
        })
        .collect::<Vec<_>>();

    let gcd = |mut a, mut b| {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    };

    steps
        .iter()
        .fold(1, |lcm, &step| lcm * step / gcd(lcm, step))
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("input.txt")));
    println!("Part 2: {}", part_2(include_str!("input.txt")));
}
