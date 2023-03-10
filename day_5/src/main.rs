const INPUT: &str = include_str!("../INPUT.txt");

fn main() {
    let mut pixels = [[false; 10]; 50];
    for line in INPUT.lines() {
        let [x, y, width, height]: [usize; 4] = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for x in x..(x + width) {
            for y in y..(y + height) {
                pixels[x][y] = !pixels[x][y];
            }
        }
    }

    for n in 0..10 {
        for row in pixels {
            print!("{}", if row[n] { '#' } else { '.' })
        }
        println!();
    }
}
