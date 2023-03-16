use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");
const SHOPS: usize = 12;

fn main() {
    let start = std::time::Instant::now();

    let shops: Vec<usize> = (1..SHOPS).collect();
    let mut matrix = [[0_u32; SHOPS]; SHOPS];

    // Parse input into `matrix`
    for (y, row) in INPUT.lines().enumerate() {
        for (x, cell) in row.split_whitespace().enumerate() {
            matrix[x][y] = cell.parse().unwrap();
        }
    }

    let mut combinations: Vec<_> = shops.iter().permutations(shops.len()).collect();

    let min = combinations
        .par_iter_mut()
        .map(|c| {
            c.insert(0, &0);
            c.push(&0);
            let mut distance = 0;
            for window in c.windows(2) {
                let last = *window[0];
                let n = *window[1];
                distance += matrix[last][n];
            }
            distance
        })
        .min()
        .unwrap();

    let duration = start.elapsed();

    println!("Min Distance: {}", min);
    println!("Found In: {:.2?}", duration);
}
