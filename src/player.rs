use std::process;

pub struct Player {
    name: String,
    pub color: String,
    pub tokens: Vec<Token>,
    pub starting_position: (usize, usize),
}

pub struct Token {
    id: usize,
    position: usize,
    pub movable: bool,
    pub full_circle: bool,
}

impl Player {
    // constructor
    pub fn new(name: &str, color: &str) -> Player {
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
            name: String::from(name),
            color: String::from(color),
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
            full_circle: false,
        }
    }
}
