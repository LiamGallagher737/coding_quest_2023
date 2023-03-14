use std::time::Duration;

const INPUT: &str = include_str!("../input.txt");
const SIZE: i32 = 20;

fn main() {
    let mut fruits = INPUT
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|f| Vec2::parse(f));

    let mut points = 0;
    let mut current_fruit = fruits.next().unwrap();
    let mut snake_cells = vec![SnakeCell {
        position: Vec2::new(0, 0),
        last_move: '.',
    }];

    'main_loop: for mov in INPUT.lines().nth(3).unwrap().chars() {
        // print_map(&snake_cells, &current_fruit);
        // print!("{}[2J", 27 as char);
        // std::thread::sleep(Duration::from_millis(100));

        let mut next_head_position = snake_cells[0].position.clone();
        next_head_position.mov(mov);

        // Out of bounds logic
        if !(0..SIZE).contains(&next_head_position.x) || !(0..SIZE).contains(&next_head_position.y)
        {
            break;
        }

        // On a fruit logic
        if next_head_position == current_fruit {
            current_fruit = fruits.next().unwrap();
            points += 101;
            snake_cells.insert(
                0,
                SnakeCell {
                    position: next_head_position,
                    last_move: mov,
                },
            );
            continue;
        }

        // Crashed into snake logic
        for cell in &snake_cells {
            if cell.position == next_head_position {
                break 'main_loop;
            }
        }

        // Moving logic
        for n in (1..snake_cells.len()).rev() {
            let mov = snake_cells[n - 1].last_move;
            snake_cells[n].position.mov(mov);
            snake_cells[n].last_move = mov;
        }

        snake_cells[0].position.mov(mov);
        snake_cells[0].last_move = mov;
        points += 1;
    }

    print_map(&snake_cells, &current_fruit);
    println!("Score: {points}");
    // dbg!(snake_cells);
}

#[derive(Debug)]
struct SnakeCell {
    position: Vec2,
    last_move: char,
}

#[derive(PartialEq, Clone, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn mov(&mut self, mov: char) {
        match mov {
            'U' => self.y -= 1,
            'D' => self.y += 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _ => unreachable!(),
        }
    }

    fn parse(input: &str) -> Self {
        let nums: Vec<_> = input.split(',').collect();
        Self {
            x: nums[0].parse().unwrap(),
            y: nums[1].parse().unwrap(),
        }
    }
}

fn print_map(snake_cells: &Vec<SnakeCell>, fruit: &Vec2) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            if *fruit == Vec2::new(x, y) {
                print!("F");
            } else if let Some(_) = snake_cells.iter().find(|c| c.position == Vec2::new(x, y)) {
                print!("S");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
