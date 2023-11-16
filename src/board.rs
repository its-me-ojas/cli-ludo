use crate::player::Player;
use colored::*;

pub const BOARD_SIZE: usize = 15;
// static movable_indexes: [(usize,usize);52] = [
//     (6, 0),(6, 1),(6, 2),(6, 3),(6, 4),(6, 5),
//     (5, 6),(4, 6),(3, 6),(2, 6),(1, 6),(0, 6),
//     (0, 7),(0, 8),
//     (1, 8),(2, 8),(3,8),(4,8),(5,8),
//     (6,9),(6,10),(6,11),(6,12),(6,13),(6,14),
//     (7,14),(8,14),
//     (8,13),(8,12),(8,11),(8,10),(8,9),
//     (9,8),(10,8),(11,8),(12,8),(13,8),(14,8),
//     (14,7),(14,6),
//     (13,6),(12,6),(11,6),(10,6),(9,6),
//     (8,5),(8,4),(8,3),(8,2),(8,1),
//     (8,0),(7,0)
// ];

pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }
}
pub struct GameBoard {
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
}

impl GameBoard {
    // constructor
    pub fn new(players: [Player; 4]) -> GameBoard {
        let mut board = [[" "; BOARD_SIZE]; BOARD_SIZE];

        // Start zones
        board[6][1] = "R";
        board[1][8] = "G";
        board[8][13] = "Y";
        board[13][6] = "B";

        // Center Square
        for i in 6..=8 {
            for j in 6..=8 {
                board[i][j] = "X";
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
                    board[i][j] = "■";
                }
                if (j >= 0 && j <= 5 || j >= 9 && j <= 14)
                    && (i == 0 || i == 5 || i == 9 || i == 14)
                {
                    board[i][j] = "■";
                }
            }
        }

        // Safe Sqaures
        board[8][2] = "S";
        board[6][12] = "S";
        board[2][6] = "S";
        board[12][8] = "S";

        // Home Stretch
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if i == 7 && (j != 6 && j != 7 && j != 8) {
                    board[i][j] = "*";
                } else if j == 7 && (i != 6 && i != 7 && i != 8) {
                    board[i][j] = "*";
                }
            }
        }

        // Movement Sqaures
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if i == 6 && (j != 6 && j != 7 && j != 8 && j != 1 && j != 12) {
                    board[i][j] = "-";
                } else if i == 8 && (j != 6 && j != 7 && j != 8 && j != 2 && j != 13) {
                    board[i][j] = "-";
                }
            }
        }
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if j == 6 && (i != 6 && i != 7 && i != 8 && i != 2 && i != 13) {
                    board[i][j] = "|";
                } else if j == 8 && (i != 6 && i != 7 && i != 8 && i != 1 && i != 12) {
                    board[i][j] = "|";
                }
            }
        }
        GameBoard { board }
    }

    // move tokens
    pub fn move_token(
        &mut self,
        position: Position,
        steps: usize,
        players: &[Player; 4],
    ) -> Position {
        let mut current_position = position;

        for _ in 0..steps {
            match self.board[current_position.row][current_position.col] {
                "-" => current_position.col += 1,
                "|" => current_position.row += 1,
                "H" => {
                    println!("Found an H block");
                    // Implement special behavior for 'H' if needed
                }
                _ => {
                    println!("Invalid movement Square");
                    break;
                }
            }

            // Display the updated board after each step
            self.display_board(players);
            println!();
        }

        current_position
    }

    // fn to display the board
    pub fn display_board(&self, players: &[Player; 4]) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '■' {
                    if i <= 5 && j <= 5 {
                        print!("{} ", "■".red());
                    } else if i <= 5 && j >= 9 {
                        print!("{} ", "■".green());
                    } else if i >= 9 && j <= 5 {
                        print!("{} ", "■".blue());
                    } else {
                        print!("{} ", "■".yellow());
                    }
                } else {
                    print!("{} ", cell);
                }

            }
            println!();
        }
    }
}

pub fn set_tokens_of_player(board: &mut [[char; 15]; 15], players: &[Player; 4]) -> () {
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
