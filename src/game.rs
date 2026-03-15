use bracket_lib::prelude::*;

const STARTING_HP: i32 = 50;
const HAND_SIZE: usize = 5;

#[derive(PartialEq, Copy, Clone)]
pub enum GamePhase {
    PlayerTurn,
    AiTurn,
    MixerDecision,
    GameOver,
}

#[derive(PartialEq, Clone)]
pub enum StatusEffect {
    BoostOpponentDamage(f32),
    ExtraSelfDamage(i32),
    SkipNextTurn,
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
    deck: Vec<Card>,
    hand: Vec<Card>,
    discard: Vec<Card>,
    rng: RandomNumberGenerator,
    phase: GamePhase,
    status_effects: Vec<(StatusEffect, u32)>,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            player_hp: STARTING_HP,
            opponent_hp: STARTING_HP,
            message: "Your deck is ready... time to drink.".to_string(),
            deck: Vec::new(),
            hand: Vec::new(),
            discard: Vec::new(),
            rng: RandomNumberGenerator::new(),
            phase: GamePhase::PlayerTurn,
            status_effects: Vec::new(),
        };
        app.build_starter_deck();
        app.draw_hand();
        app
    }

    pub fn reset(&mut self) {
        *self = App::new();
    }

    fn build_starter_deck(&mut self) {
        self.deck = vec![
            Card {
                name: "Pliny the Elder".into(),
                description: "Legendary DIPA".into(),
                damage_to_self: 5,
                damage_to_opponent: 14,
            },
            Card {
                name: "Heady Topper".into(),
                description: "Hazy Vermont bomb".into(),
                damage_to_self: 8,
                damage_to_opponent: 16,
            },
            Card {
                name: "Stone IPA".into(),
                description: "West Coast classic".into(),
                damage_to_self: 3,
                damage_to_opponent: 11,
            },
            Card {
                name: "Guinness".into(),
                description: "Thick & healing".into(),
                damage_to_self: -6,
                damage_to_opponent: 10,
            },
            Card {
                name: "Bud Light".into(),
                description: "Light & sneaky".into(),
                damage_to_self: -4,
                damage_to_opponent: 7,
            },
            Card {
                name: "Sierra Nevada Pale".into(),
                description: "Reliable classic".into(),
                damage_to_self: 4,
                damage_to_opponent: 12,
            },
            Card {
                name: "Lagunitas IPA".into(),
                description: "Citrus punch".into(),
                damage_to_self: 6,
                damage_to_opponent: 13,
            },
            Card {
                name: "Two Hearted Ale".into(),
                description: "Michigan legend".into(),
                damage_to_self: 5,
                damage_to_opponent: 14,
            },
            Card {
                name: "Zombie Dust".into(),
                description: "Pale Ale powerhouse".into(),
                damage_to_self: 7,
                damage_to_opponent: 15,
            },
            Card {
                name: "Corona Extra".into(),
                description: "Beach session beer".into(),
                damage_to_self: -3,
                damage_to_opponent: 8,
            },
            Card {
                name: "Miller High Life".into(),
                description: "The Champagne of Beers".into(),
                damage_to_self: -2,
                damage_to_opponent: 9,
            },
            Card {
                name: "PBR".into(),
                description: "Working class hero".into(),
                damage_to_self: -5,
                damage_to_opponent: 10,
            },
        ];
        App::shuffle_vec(&mut self.rng, &mut self.deck);
    }

    fn shuffle_vec<T>(rng: &mut RandomNumberGenerator, vec: &mut Vec<T>) {
        for i in (1..vec.len()).rev() {
            let j = rng.range(0, (i + 1) as i32) as usize;
            vec.swap(i, j);
        }
    }

    fn draw_hand(&mut self) {
        self.reshuffle_if_needed();

        let mut drawn = Vec::new();

        while drawn.len() < HAND_SIZE && !self.deck.is_empty() {
            if let Some(card) = self.deck.pop() {
                drawn.push(card);
            }
        }

        self.hand.extend(drawn);
    }

    fn reshuffle_if_needed(&mut self) {
        if self.deck.is_empty() && !self.discard.is_empty() {
            self.deck = self.discard.drain(..).collect();
            App::shuffle_vec(&mut self.rng, &mut self.deck);
            self.message = "Deck reshuffled!".to_string();
        }
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
    pub fn deck_size(&self) -> usize {
        self.deck.len()
    }
    pub fn discard_size(&self) -> usize {
        self.discard.len()
    }

    pub fn play_card(&mut self, index: usize) {
        if !self.is_player_turn() || self.is_game_over() || index >= self.hand.len() {
            self.message = "You can't play right now!".to_string();
            return;
        }

        let card = self.hand.remove(index);
        let self_dmg = card.damage_to_self + self.extra_self_damage_this_turn();
        let opp_dmg = self.modify_damage_to_opponent(card.damage_to_opponent);

        self.player_hp -= self_dmg;
        self.opponent_hp -= opp_dmg;

        self.discard.push(card.clone());

        self.message = format!(
            "You crushed a {}! ({} to you, {} to them)",
            card.name, self_dmg, opp_dmg
        );

        self.phase = GamePhase::AiTurn;
    }

    pub fn advance_turn(&mut self) {
        if self.phase == GamePhase::AiTurn {
            if self.is_game_over() {
                self.phase = GamePhase::GameOver;
                return;
            }

            if !self.hand.is_empty() {
                let choice = self.rng.range(0, self.hand.len() as i32) as usize;
                let card = &self.hand[choice];
                self.player_hp -= card.damage_to_opponent;
                self.opponent_hp -= card.damage_to_self / 2;
                self.message
                    .push_str(&format!("  AI chugged {}!", card.name));
            }

            self.draw_hand();
            self.phase = GamePhase::MixerDecision;
            self.message = "Mixer Event! Choose your chaser (1-3):".to_string();
        }
        self.tick_status_effects();
    }

    pub fn choose_mixer(&mut self, choice: usize) {
        if self.phase != GamePhase::MixerDecision {
            return;
        }

        match choice {
            0 => {
                self.apply_status(StatusEffect::BoostOpponentDamage(1.5), 1);
                self.apply_status(StatusEffect::ExtraSelfDamage(2), 1);
                self.message =
                    "Red Bull chaser! Next card hits harder... but it'll sting.".to_string();
            }
            1 => {
                self.opponent_hp -= 12;
                self.apply_status(StatusEffect::SkipNextTurn, 1);
                self.message =
                    "Fireball shot! Opponent takes 12 extra... but you skip next turn.".to_string();
            }
            2 => {
                let new_card = Card {
                    name: "Dragon's Milk Stout".into(),
                    description: "Barrel-aged monster (permanent!)".into(),
                    damage_to_self: 6,
                    damage_to_opponent: 19,
                };
                self.deck.push(new_card);
                self.message =
                    "You discovered Dragon's Milk Stout! Added to your deck forever.".to_string();
            }
            _ => {}
        }

        self.phase = GamePhase::PlayerTurn;
        self.draw_hand();
    }

    fn apply_status(&mut self, effect: StatusEffect, duration: u32) {
        self.status_effects.push((effect, duration));
    }

    fn tick_status_effects(&mut self) {
        self.status_effects.retain_mut(|(_, t)| {
            *t = t.saturating_sub(1);
            *t > 0
        });
    }

    fn modify_damage_to_opponent(&self, base: i32) -> i32 {
        let mut dmg = base as f32;
        for (eff, _) in &self.status_effects {
            if let StatusEffect::BoostOpponentDamage(m) = eff {
                dmg *= *m;
            }
        }
        dmg as i32
    }

    fn extra_self_damage_this_turn(&self) -> i32 {
        self.status_effects
            .iter()
            .filter_map(|(e, _)| {
                if let StatusEffect::ExtraSelfDamage(x) = e {
                    Some(*x)
                } else {
                    None
                }
            })
            .sum()
    }
}
