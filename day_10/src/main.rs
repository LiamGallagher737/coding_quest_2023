use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");
const START: &str = "TYC";
const END: &str = "EAR";
const STOP_TIME: u32 = 600;

fn main() {
    let mut data = HashMap::new();
    for line in INPUT.lines() {
        let from = &line[0..3];

        let mut next_points = vec![];
        for to in line[7..].split_whitespace() {
            let point = &to[..3];
            let distance: u32 = to[4..].parse().unwrap();
            next_points.push((point, distance));
        }

        data.insert(from, next_points);
    }

    let mut vertex_map = HashMap::with_capacity(data.len());
    for (key, _) in &data {
        vertex_map.insert(*key, u32::MAX);
    }
    *vertex_map.get_mut(&START).unwrap() = 0;

    let mut visited = HashSet::new();

    while !visited.contains(END) {
        let shortest = vertex_map
            .iter()
            .filter(|(k, _)| !visited.contains(&k.to_string()))
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap()
            .to_string();

        visited.insert(shortest.clone());

        for (point, distance) in &data[shortest.as_str()] {
            let distance_from_a = vertex_map[shortest.as_str()] + distance + STOP_TIME;
            if vertex_map[point] > distance_from_a {
                *vertex_map.get_mut(point).unwrap() = distance_from_a;
            }
        }
    }

    println!("{START} to {END} will take {} seconds", vertex_map[END] - STOP_TIME);

    // dbg!(vertex_map);
}
