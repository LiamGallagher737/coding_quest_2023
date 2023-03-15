use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");
const SHOPS: usize = 12;

fn main() {
    let shops: Vec<usize> = (1..SHOPS).collect();
    let combinations: Vec<_> = shops.iter().permutations(shops.len()).collect();
    let mut matrix = [[0; SHOPS]; SHOPS];

    for (y, row) in INPUT.lines().enumerate() {
        for (x, cell) in row.split_whitespace().enumerate() {
            matrix[x][y] = cell.parse().unwrap();
        }
    }

    let mut distances = Vec::with_capacity(combinations.len());

    for combination in combinations.iter() {
        let mut distance = 0;
        let mut combination = combination.clone();
        combination.insert(0, &0);
        combination.push(&0);
        for window in combination.windows(2) {
            let last = *window[0];
            let n = *window[1];
            distance += matrix[last][n];
        }
        distances.push(distance);
    }

    dbg!(distances.iter().min());
}
