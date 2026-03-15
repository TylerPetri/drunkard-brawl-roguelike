use bracket_lib::prelude::*;

mod game;
use game::App;

struct State {
    app: App,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        // Auto-advance AI turn when it's their phase
        if self.app.is_ai_turn() && !self.app.is_game_over() {
            self.app.advance_turn();
        }

        self.draw_ui(ctx);

        if let Some(key) = ctx.key {
            self.handle_input(key);
        }
    }
}

impl State {
    fn draw_ui(&self, ctx: &mut BTerm) {
        // Title
        ctx.print_color_centered(
            1,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            "=== DRUNKARD BRAWL 🍺🥴 ===",
        );

        // HP display
        ctx.print_color(
            5,
            4,
            RGB::named(WHITE),
            RGB::named(BLACK),
            &format!("YOU       : {}", self.app.player_hp()),
        );
        ctx.print_color(
            5,
            6,
            RGB::named(CYAN),
            RGB::named(BLACK),
            &format!("OPPONENT  : {}", self.app.opponent_hp()),
        );

        // Phase / state indicator
        let phase_text = if self.app.is_game_over() {
            "GAME OVER"
        } else if self.app.is_player_turn() {
            "YOUR TURN – choose a beer!"
        } else if self.app.is_ai_turn() {
            "Opponent is chugging..."
        } else if self.app.is_mixer_phase() {
            "MIXER EVENT! Pick your chaser (1-3)"
        } else {
            "???"
        };

        ctx.print_color_centered(
            8,
            RGB::named(LIGHT_GREEN),
            RGB::named(BLACK),
            phase_text,
        );

        // Main message
        ctx.print_color_centered(
            10,
            RGB::named(LIGHT_BLUE),
            RGB::named(BLACK),
            self.app.message(),
        );

        if self.app.is_game_over() {
            ctx.print_color_centered(
                14,
                RGB::named(RED),
                RGB::named(BLACK),
                "YOU BLACKED OUT – or they did!  (R = restart)",
            );
        }

        // Action area (hand or mixer choices)
        ctx.draw_box(3, 16, 74, 13, RGB::named(WHITE), RGB::named(BLACK));

        if self.app.is_game_over() {
            // nothing extra
        } else if self.app.is_mixer_phase() {
            ctx.print_color_centered(
                18,
                RGB::named(PINK),
                RGB::named(BLACK),
                "Choose your mixer / chaser...",
            );

            let mixer_options = [
                "1) Red Bull chaser — next card +50% dmg to them, but +2 self dmg",
                "2) Fireball shot — instant +10 to them, but skip your next turn",
                "3) Greasy pizza — +15 HP to you, +5 HP to them",
            ];

            for (i, line) in mixer_options.iter().enumerate() {
                let y = 21 + i as i32;
                ctx.print(6, y, format!("{}) {}", i + 1, line));
            }
        } else if self.app.is_player_turn() {
            ctx.print_color_centered(
                18,
                RGB::named(ORANGE),
                RGB::named(BLACK),
                "CHOOSE YOUR BEER (press 1-5)",
            );

            let hand = self.app.get_hand();
            for (i, card) in hand.iter().enumerate() {
                let y = 21 + i as i32;
                ctx.print(
                    6,
                    y,
                    format!("{}) {} — {}", i + 1, card.name, card.description),
                );
            }
        }

        // Footer controls
        ctx.print_color(
            6,
            29,
            RGB::named(GRAY),
            RGB::named(BLACK),
            "Q = Quit    R = Restart",
        );
    }

    fn handle_input(&mut self, key: VirtualKeyCode) {
        if self.app.is_game_over() {
            match key {
                VirtualKeyCode::R => self.app.reset(),
                VirtualKeyCode::Q => std::process::exit(0),
                _ => {}
            }
            return;
        }

        // Mixer phase has priority
        if self.app.is_mixer_phase() {
            let choice = match key {
                VirtualKeyCode::Key1 => Some(0usize),
                VirtualKeyCode::Key2 => Some(1),
                VirtualKeyCode::Key3 => Some(2),
                _ => None,
            };

            if let Some(idx) = choice {
                self.app.choose_mixer(idx);
            }
            return;
        }

        // Normal player turn – card selection
        if self.app.is_player_turn() {
            let card_idx = match key {
                VirtualKeyCode::Key1 => Some(0usize),
                VirtualKeyCode::Key2 => Some(1),
                VirtualKeyCode::Key3 => Some(2),
                VirtualKeyCode::Key4 => Some(3),
                VirtualKeyCode::Key5 => Some(4),
                _ => None,
            };

            if let Some(idx) = card_idx {
                self.app.play_card(idx);
            }
        }

        // Global shortcuts
        match key {
            VirtualKeyCode::R => self.app.reset(),
            VirtualKeyCode::Q => std::process::exit(0),
            _ => {}
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Drunkard Brawl 🍺🥴")
        .build()?;

    main_loop(context, State { app: App::new() })
}