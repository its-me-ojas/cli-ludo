mod board;
mod player;

fn main() {
    println!("Welcome to Command Line Ludo");
    let ludo_board = board::GameBoard::new();
    ludo_board.display_board();
}
