use bracket_lib::prelude::*;

pub struct App {
    // We'll put everything here soon
    player_hp: i32,
    opponent_hp: i32,
    turn: u32,               // 0 = player's turn, 1 = opponent's turn
    message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            player_hp: 50,
            opponent_hp: 50,
            turn: 0,
            message: "Welcome to Drunkard Brawl!".to_string(),
        }
    }
}