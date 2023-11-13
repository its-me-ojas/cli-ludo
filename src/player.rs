pub struct Player {
    name: String,
    color: String,
    tokens: Vec<Token>,
}

struct Token {
    id: usize,
    position: usize,
    movable: bool,
}

impl Player {
    // constructor
    pub fn new(name: &str, color: &str) -> Player {
        Player {
            name: String::from(name),
            color: String::from(color),
            tokens: vec![Token::new(1), Token::new(2), Token::new(3), Token::new(4)],
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
        }
    }
}
