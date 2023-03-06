use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut total_quantities = HashMap::<&str, u64>::new();
    for line in INPUT.lines() {
        let [_uuid, quantity, category]: [&str; 3] =
            line.split(' ').collect::<Vec<&str>>().try_into().unwrap();

        total_quantities.entry(category).or_default();
        total_quantities
            .entry(category)
            .and_modify(|v| *v += quantity.parse::<u64>().unwrap());
    }
    let mods: Vec<_> = total_quantities.values().map(|v| v % 100).collect();
    let prod: u64 = mods.iter().product();
    println!("Product: {prod}");
}
