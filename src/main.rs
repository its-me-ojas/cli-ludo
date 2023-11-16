use board::Position;

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

    let mut ludo_board = board::GameBoard::new(&players);
    ludo_board.display_board(&players);

    let dice_number = dice::roll_dice();
    ludo_board.move_token(Position::new(6, 1), dice_number, &players);
}
