use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

const WINNING_FORMATIONS: [[bool; 9]; 8] = [
    // Rows
    [true, true, true, false, false, false, false, false, false],
    [false, false, false, true, true, true, false, false, false],
    [false, false, false, false, false, false, true, true, true],
    // Columns
    [true, false, false, true, false, false, true, false, false],
    [false, true, false, false, true, false, false, true, false],
    [false, false, true, false, false, true, false, false, true],
    // Diagonals
    [true, false, false, false, true, false, false, false, true],
    [false, false, true, false, true, false, true, false, false],
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl From<u8> for Player {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::X,
            1 => Self::O,
            _ => panic!(),
        }
    }
}

fn main() {
    let mut wins = HashMap::new();
    wins.insert(Player::X, 0);
    wins.insert(Player::O, 0);
    let mut draws = 0;

    'game_loop: for game in INPUT.lines() {
        let mut board: [Option<Player>; 9] = [None; 9];

        for (i, square) in game.split_whitespace().enumerate() {
            let square: usize = square.parse().unwrap();
            let player = Player::from(i as u8 % 2);
            board[square - 1] = Some(player);

            if i < 3 {
                continue;
            }

            if let Some(player) = check_for_winners(board) {
                wins.entry(player).and_modify(|w| *w += 1);
                continue 'game_loop;
            }
        }

        draws += 1;
    }

    let player_x_wins = wins[&Player::X];
    let player_o_wins = wins[&Player::O];

    println!("X won: {player_x_wins}");
    println!("O won: {player_o_wins}");
    println!("Result: {}", player_x_wins * player_o_wins * draws);
}

fn check_for_winners(board: [Option<Player>; 9]) -> Option<Player> {
    for formation in WINNING_FORMATIONS {
        'player_loop: for player in [Player::X, Player::O] {
            for (square, _) in formation.iter().enumerate().filter(|s| *s.1) {
                if board[square] != Some(player) {
                    continue 'player_loop;
                }
            }

            return Some(player);
        }
    }

    None
}
