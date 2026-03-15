use bracket_lib::prelude::*;

const STARTING_HP: i32 = 50;
const CARDS_PER_HAND: usize = 5;

pub struct Card {
    pub name: String,
    pub description: String,
    pub damage_to_self: i32,
    pub damage_to_opponent: i32,
}

#[derive(PartialEq)]
pub enum GamePhase {
    PlayerTurn,
    AiTurn,
    // MixerEvent,       // placeholder for later
    GameOver,
}

// enum MixerChoice {
//     RedBull,      // +damage next, -hp after
//     FireballShot, // instant dmg, skip turn
//     Pizza,        // mutual heal
// }

pub struct App {
    player_hp: i32,
    opponent_hp: i32,
    message: String,
    hand: Vec<Card>,
    rng: RandomNumberGenerator,
    phase: GamePhase,
    // pending_mixer: Option<MixerChoice>,
    // status_effects: Vec<(StatusEffect, u32)>, // effect + turns left
}

// enum StatusEffect {
//     BoostDamage(f32), // multiplier
//     ExtraSelfDamage(i32),
//     SkipTurn,
//     // etc.
// }

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            player_hp: STARTING_HP,
            opponent_hp: STARTING_HP,
            message: "Pick your poison, legend.".to_string(),
            hand: Vec::new(),
            rng: RandomNumberGenerator::new(),
            phase: GamePhase::PlayerTurn,
        };
        app.new_hand();
        app
    }

    fn new_hand(&mut self) {
        self.hand = vec![
            Card {
                name: "Pliny the Elder".into(),
                description: "Legendary DIPA".into(),
                damage_to_self: 4,
                damage_to_opponent: 13,
            },
            Card {
                name: "Heady Topper".into(),
                description: "Hazy Vermont bomb".into(),
                damage_to_self: 7,
                damage_to_opponent: 15,
            },
            Card {
                name: "Stone IPA".into(),
                description: "West Coast classic".into(),
                damage_to_self: 2,
                damage_to_opponent: 10,
            },
            Card {
                name: "Guinness".into(),
                description: "Thick & healing".into(),
                damage_to_self: -5,
                damage_to_opponent: 9,
            },
            Card {
                name: "Bud Light".into(),
                description: "Light & sneaky".into(),
                damage_to_self: -3,
                damage_to_opponent: 6,
            },
        ];
    }

    pub fn player_hp(&self) -> i32 {
        self.player_hp
    }
    pub fn opponent_hp(&self) -> i32 {
        self.opponent_hp
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn is_player_turn(&self) -> bool {
        self.phase == GamePhase::PlayerTurn
    }
    pub fn is_ai_turn(&self) -> bool {
        self.phase == GamePhase::AiTurn
    }
    pub fn is_game_over(&self) -> bool {
        self.player_hp <= 0 || self.opponent_hp <= 0
    }
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    pub fn play_card(&mut self, index: usize) {
        if self.phase != GamePhase::PlayerTurn || self.is_game_over() || index >= self.hand.len() {
            self.message = if self.phase != GamePhase::PlayerTurn {
                "Not your turn!".to_string()
            } else {
                "Invalid card choice!".to_string()
            };
            return;
        }

        let card = &self.hand[index]; // borrow, no clone

        self.player_hp -= card.damage_to_self;
        self.opponent_hp -= card.damage_to_opponent;

        self.message = format!(
            "You crushed a {}! ({} to you, {} to them)",
            card.name, card.damage_to_self, card.damage_to_opponent
        );

        self.phase = GamePhase::AiTurn;
    }

    pub fn advance_turn(&mut self) {
        match self.phase {
            GamePhase::PlayerTurn => {
                // Player just played → go to AI
                self.phase = GamePhase::AiTurn;
            }
            GamePhase::AiTurn => {
                // AI plays
                if !self.is_game_over() {
                    let choice = self.rng.range(0, self.hand.len() as i32) as usize;
                    let card = &self.hand[choice];

                    self.player_hp -= card.damage_to_opponent;
                    self.opponent_hp -= card.damage_to_self / 2;

                    self.message
                        .push_str(&format!("  AI chugged {}!", card.name));
                }
                self.new_hand();
                self.phase = if self.is_game_over() {
                    GamePhase::GameOver
                } else {
                    GamePhase::PlayerTurn
                };
            }
            _ => {}
        }
    }
}
