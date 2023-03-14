const INPUT: &str = include_str!("../input.txt");
const TIME: i32 = 3600 + 60;
const SIZE: usize = 100;

fn main() {
    let mut astriods = vec![];
    for astriod in INPUT.lines() {
        astriods.push(Astroid::parse(astriod));
    }

    let mut map = [[false; SIZE]; SIZE];

    for t in 0..TIME {
        for astroid in &mut astriods {
            if (0.0..SIZE as f32).contains(&astroid.x) && (0.0..SIZE as f32).contains(&astroid.y) && t > 3600 {
                map[astroid.x.floor() as usize][astroid.y.floor() as usize] = true;
            }
            astroid.x += astroid.x_vel;
            astroid.y += astroid.y_vel;
        }
    }

    for n in 0..map.len() {
        for row in map {
            print!("{}", if row[n] { '#' } else { '.' })
        }
        println!();
    }

    for (x, v) in map.iter().enumerate() {
        for (y, cell) in v.iter().enumerate() {
            if !*cell {
                println!("{x}:{y} is safe!");
            }
        }
    }

    if map.iter().flatten().find(|v| !**v).is_some() {
        println!("Valid psoition exists");
    } else {
        println!("No valid psoition exists");
    }
}

struct Astroid {
    x: f32,
    y: f32,
    x_vel: f32,
    y_vel: f32,
}

impl Astroid {
    fn parse(input: &str) -> Self {
        let input: Vec<_> = input.split_whitespace().collect();
        Self {
            x: input[0].parse().unwrap(),
            y: input[1].parse().unwrap(),
            x_vel: input[2].parse().unwrap(),
            y_vel: input[3].parse().unwrap(),
        }
    }
}

// let [x, y, x_speed, y_speed]: [i32; 4] = astriod
//     .split_whitespace()
//     .map(|n| n.parse::<i32>().unwrap())
//     .collect::<Vec<_>>()
//     .try_into()
//     .unwrap();

// for t in 0..TIME {
//     let x = x + (x_speed * t);
//     let y = y + (y_speed * t);

//     let Ok(x) = usize::try_from(x) else {
//         continue;
//     };
//     let Ok(y) = usize::try_from(y) else {
//         continue;
//     };

//     if let Some(Some(cell)) = map.get_mut(x).map(|v| v.get_mut(y)) {
//         *cell = true;
//     }
// }
