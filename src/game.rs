use bracket_lib::prelude::*;

const STARTING_HP: i32 = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum GamePhase {
    PlayerTurn,
    AiTurn,
    MixerDecision,
    GameOver,
}

#[derive(PartialEq, Clone)]
pub enum StatusEffect {
    BoostOpponentDamage(f32),   // multiplier for next card(s) damage to opponent
    ExtraSelfDamage(i32),       // flat added self-damage next play
    SkipNextTurn,               // player skips one full turn
    DoubleAiPlays,              // AI plays twice next round (bad!)
}

#[derive(Clone)]
pub struct Card {
    pub name: String,
    pub description: String,
    pub damage_to_self: i32,
    pub damage_to_opponent: i32,
}

pub struct App {
    player_hp: i32,
    opponent_hp: i32,
    message: String,
    hand: Vec<Card>,
    rng: RandomNumberGenerator,
    phase: GamePhase,
    status_effects: Vec<(StatusEffect, u32)>,  // (effect, turns remaining)
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            player_hp: STARTING_HP,
            opponent_hp: STARTING_HP,
            message: "Pick your poison, legend.".to_string(),
            hand: Vec::new(),
            rng: RandomNumberGenerator::new(),
            phase: GamePhase::PlayerTurn,
            status_effects: Vec::new(),
        };
        app.new_hand();
        app
    }

    pub fn reset(&mut self) {
        *self = App::new();
    }

    fn new_hand(&mut self) {
        self.hand = vec![
            Card { name: "Pliny the Elder".into(), description: "Legendary DIPA".into(), damage_to_self: 4, damage_to_opponent: 13 },
            Card { name: "Heady Topper".into(),    description: "Hazy Vermont bomb".into(), damage_to_self: 7, damage_to_opponent: 15 },
            Card { name: "Stone IPA".into(),       description: "West Coast classic".into(), damage_to_self: 2, damage_to_opponent: 10 },
            Card { name: "Guinness".into(),        description: "Thick & healing".into(), damage_to_self: -5, damage_to_opponent: 9 },
            Card { name: "Bud Light".into(),       description: "Light & sneaky".into(), damage_to_self: -3, damage_to_opponent: 6 },
        ];
    }

    // ── Public getters / queries ───────────────────────────────────────

    pub fn player_hp(&self) -> i32 { self.player_hp }
    pub fn opponent_hp(&self) -> i32 { self.opponent_hp }
    pub fn message(&self) -> &str { &self.message }

    pub fn is_game_over(&self) -> bool {
        self.player_hp <= 0 || self.opponent_hp <= 0
    }

    pub fn is_player_turn(&self) -> bool {
        self.phase == GamePhase::PlayerTurn
    }

    pub fn is_ai_turn(&self) -> bool {
        self.phase == GamePhase::AiTurn
    }

    pub fn is_mixer_phase(&self) -> bool {
        self.phase == GamePhase::MixerDecision
    }

    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    // ── Game actions ───────────────────────────────────────────────────

    pub fn play_card(&mut self, index: usize) {
        if !self.is_player_turn() || self.is_game_over() {
            self.message = "You can't play right now!".to_string();
            return;
        }

        if index >= self.hand.len() {
            self.message = "Invalid card choice!".to_string();
            return;
        }

        let card = &self.hand[index];

        let mut self_dmg = card.damage_to_self + self.extra_self_damage_this_turn();
        let opp_dmg = self.modify_damage_to_opponent(card.damage_to_opponent);

        self.player_hp -= self_dmg;
        self.opponent_hp -= opp_dmg;

        self.message = format!(
            "You crushed a {}! ({} to you, {} to them)",
            card.name, self_dmg, opp_dmg
        );

        self.phase = GamePhase::AiTurn;
    }

    pub fn advance_turn(&mut self) {
        match self.phase {
            GamePhase::AiTurn => {
                if self.is_game_over() {
                    self.phase = GamePhase::GameOver;
                    return;
                }

                let choice = self.rng.range(0, self.hand.len() as i32) as usize;
                let card = &self.hand[choice];

                self.player_hp -= card.damage_to_opponent;
                self.opponent_hp -= card.damage_to_self / 2;

                self.message.push_str(&format!("  AI chugged {}!", card.name));

                self.new_hand();

                // After full round → trigger mixer
                self.phase = GamePhase::MixerDecision;
                self.message = "Mixer Event! Choose your chaser (1-3):".to_string();
            }

            _ => {
                // Only call this when in AiTurn
            }
        }

        self.tick_status_effects();
    }

    pub fn choose_mixer(&mut self, choice: usize) {
        if self.phase != GamePhase::MixerDecision {
            return;
        }

        match choice {
            0 => { // 1: Red Bull chaser
                self.apply_status(StatusEffect::BoostOpponentDamage(1.5), 1);
                self.apply_status(StatusEffect::ExtraSelfDamage(2), 1);
                self.message = "Red Bull chaser! Next card hits harder... but it'll sting.".to_string();
            }
            1 => { // 2: Fireball shot
                self.opponent_hp -= 10;
                self.apply_status(StatusEffect::SkipNextTurn, 1);
                self.message = "Fireball shot! Opponent takes 10 extra... but you're sitting this one out.".to_string();
            }
            2 => { // 3: Greasy pizza
                self.player_hp += 15;
                self.opponent_hp += 5;
                self.message = "Greasy pizza! You recover 15 HP, opponent steals 5 back.".to_string();
            }
            _ => {
                self.message = "Invalid mixer choice... you hesitate and feel worse.".to_string();
            }
        }

        self.phase = GamePhase::PlayerTurn;
    }

    // ── Status effect helpers ──────────────────────────────────────────

    fn apply_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.push((effect, duration));
    }

    fn tick_status_effects(&mut self) {
        self.status_effects.retain_mut(|(_, turns)| {
            *turns = turns.saturating_sub(1);
            *turns > 0
        });
    }

    fn modify_damage_to_opponent(&self, base: i32) -> i32 {
        let mut dmg = base as f32;
        for (eff, _) in &self.status_effects {
            if let StatusEffect::BoostOpponentDamage(multi) = eff {
                dmg *= *multi;
            }
        }
        dmg as i32
    }

    fn extra_self_damage_this_turn(&self) -> i32 {
        let mut extra = 0;
        for (eff, _) in &self.status_effects {
            if let StatusEffect::ExtraSelfDamage(add) = eff {
                extra += *add;
            }
        }
        extra
    }
}