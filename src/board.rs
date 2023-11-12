const BOARD_SIZE: usize = 15;

pub struct GameBoard {
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
}

impl GameBoard {
    // constructor
    pub fn new() -> GameBoard {
        let mut board = [[' '; BOARD_SIZE]; BOARD_SIZE];

        // Start zones
        board[0][0] = 'A';
        board[0][BOARD_SIZE - 1] = 'B';
        board[BOARD_SIZE - 1][0] = 'C';
        board[BOARD_SIZE - 1][BOARD_SIZE - 1] = 'D';

        // Main path
        for i in 1..BOARD_SIZE - 1 {
            board[0][i] = '-';
            board[BOARD_SIZE - 1][i] = '-';
            board[i][0] = '|';
            board[i][BOARD_SIZE - 1] = '|';
        }

        // Safe zones
        for i in 1..4 {
            for j in 1..4 {
                board[i][j] = 'S';
                board[i][BOARD_SIZE - 1 - j] = 'S';
                board[BOARD_SIZE - 1 - i][j] = 'S';
                board[BOARD_SIZE - 1 - i][BOARD_SIZE - 1 - j] = 'S';
            }
        }

        // Home stretch
        for i in 4..BOARD_SIZE - 4 {
            board[i][4] = 'H';
            board[i][BOARD_SIZE - 5] = 'H';
            board[4][i] = 'H';
            board[BOARD_SIZE - 5][i] = 'H';
        }

        // intialising the center sqaure
        board[7][7] = 'X';

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
