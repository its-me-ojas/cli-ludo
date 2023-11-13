// use colored::*;

use crate::player::Player;

const BOARD_SIZE: usize = 15;

pub struct GameBoard {
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
}

impl GameBoard {
    // constructor
    pub fn new(players: &[Player; 4]) -> GameBoard {
        let mut board = [[' '; BOARD_SIZE]; BOARD_SIZE];

        // Start zones
        board[6][1] = 'R';
        board[1][8] = 'G';
        board[8][13] = 'Y';
        board[13][6] = 'B';

        // Center Square
        for i in 6..=8 {
            for j in 6..=8 {
                board[i][j] = 'X';
            }
        }

        // Player tokens
        set_tokens_of_player(&mut board, players);

        // Player Boundaries
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if (i >= 0 && i <= 5 || i >= 9 && i <= 14)
                    && (j == 0 || j == 5 || j == 9 || j == 14)
                {
                    board[i][j] = '■';
                }
                if (j >= 0 && j <= 5 || j >= 9 && j <= 14)
                    && (i == 0 || i == 5 || i == 9 || i == 14)
                {
                    board[i][j] = '■';
                }
            }
        }

        // Safe Sqaures
        board[8][2] = 'S';
        board[6][12] = 'S';
        board[2][6] = 'S';
        board[11][8] = 'S';

        GameBoard { board }
    }

    // fn to display the board
    pub fn display_board(&self) {
        for row in &self.board {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}

fn set_tokens_of_player(board: &mut [[char; 15]; 15], players: &[Player; 4]) -> () {
    for (index, player) in players.iter().enumerate() {
        let positions = match index {
            0 => &[(2, 2), (2, 3), (3, 2), (3, 3)],
            1 => &[(11, 2), (11, 3), (12, 2), (12, 3)],
            2 => &[(11, 11), (11, 12), (12, 11), (12, 12)],
            3 => &[(2, 11), (2, 12), (3, 11), (3, 12)],
            _ => &[(0, 0); 4],
        };

        for (i, j) in positions {
            for token in &player.tokens {
                board[*i][*j] = if !token.movable {
                    player
                        .color
                        .chars()
                        .next()
                        .unwrap_or(' ')
                        .to_uppercase()
                        .next()
                        .unwrap_or(' ')
                } else {
                    ' '
                };
            }
        }
    }
}
