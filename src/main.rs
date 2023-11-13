mod board;
mod dice;
mod intro;
mod player;

fn main() {
    intro::introduction();
    let player1 = player::Player::new("crestfallen", "red");
    let player2 = player::Player::new("daveydark", "blue");
    let player3 = player::Player::new("dostmalone", "yellow");
    let player4 = player::Player::new("nvme", "green");
    let players: [player::Player; 4] = [player1, player2, player3, player4];

    let ludo_board = board::GameBoard::new(&players);
    ludo_board.display_board();

    println!("{}", dice::roll_dice());
}
