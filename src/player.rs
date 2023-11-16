use std::process;

pub struct Player<'a> {
    name: &'a str,
    pub color: &'a str,
    pub tokens: Vec<Token>,
    pub starting_position: (usize, usize),
}

pub struct Token {
    id: usize,
    position: usize,
    pub movable: bool,
    count: usize,
}

impl<'a> Player<'a> {
    // constructor
    pub fn new<'b>(name: &'b str, color: &'b str) -> Player<'b> {
        let starting_position = match color {
            "red" => (6, 1),
            "green" => (6, 1),
            "blue" => (1, 8),
            "yellow" => (8, 13),
            _ => {
                eprintln!("wrong color");
                process::exit(1);
            }
        };
        Player {
            name,
            color,
            tokens: vec![Token::new(1), Token::new(2), Token::new(3), Token::new(4)],
            starting_position,
        }
    }

    pub fn display_info(&self) {
        println!("Player: {}", self.name);
        println!("Color: {}", self.color);
        println!("Tokens: ");
        for token in &self.tokens {
            println!("Token {} : Position {}", token.id, token.position);
        }
    }
}

impl Token {
    // constructor
    fn new(id: usize) -> Token {
        Token {
            id,
            position: 0,
            movable: false,
            count: 0,
        }
    }
}
