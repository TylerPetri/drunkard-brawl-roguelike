use bracket_lib::prelude::*;

mod game;
use game::App;

struct State {
    app: App,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

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
        ctx.print_color_centered(1, RGB::named(YELLOW), RGB::named(BLACK), "=== DRUNKARD BRAWL 🍺🥴 ===");

        ctx.print_color(5, 4, RGB::named(WHITE), RGB::named(BLACK), &format!("YOU       : {}", self.app.player_hp()));
        ctx.print_color(5, 6, RGB::named(CYAN),  RGB::named(BLACK), &format!("OPPONENT  : {}", self.app.opponent_hp()));

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
        ctx.print_color_centered(8, RGB::named(LIGHT_GREEN), RGB::named(BLACK), phase_text);

        ctx.print_color_centered(10, RGB::named(LIGHT_BLUE), RGB::named(BLACK), self.app.message());

        if self.app.is_game_over() {
            ctx.print_color_centered(14, RGB::named(RED), RGB::named(BLACK), "YOU BLACKED OUT! (R = restart)");
        }

        ctx.draw_box(3, 16, 74, 13, RGB::named(WHITE), RGB::named(BLACK));

        if self.app.is_mixer_phase() {
            ctx.print_color_centered(18, RGB::named(PINK), RGB::named(BLACK), "Choose your mixer / chaser...");
            let options = [
                "1) Red Bull chaser — next card +50% dmg to them, +2 self dmg",
                "2) Fireball shot — instant +12 to them, skip your next turn",
                "3) Discover Dragon's Milk Stout — add permanent beast to deck!",
            ];
            for (i, line) in options.iter().enumerate() {
                ctx.print(6, 21 + i as i32, format!("{}) {}", i + 1, line));
            }
        } else if self.app.is_player_turn() {
            ctx.print_color_centered(18, RGB::named(ORANGE), RGB::named(BLACK), "CHOOSE YOUR BEER (press 1-5)");
            let hand = self.app.get_hand();
            for (i, card) in hand.iter().enumerate() {
                ctx.print(6, 21 + i as i32, format!("{}) {} — {}", i + 1, card.name, card.description));
            }
        }

        ctx.print_color(
            6,
            29,
            RGB::named(GRAY),
            RGB::named(BLACK),
            &format!("Deck: {} left | Discard: {} | Q=Quit  R=Restart", self.app.deck_size(), self.app.discard_size()),
        );
    }

    fn handle_input(&mut self, key: VirtualKeyCode) {
        if self.app.is_game_over() {
            if key == VirtualKeyCode::R { self.app.reset(); }
            if key == VirtualKeyCode::Q { std::process::exit(0); }
            return;
        }

        if self.app.is_mixer_phase() {
            let choice = match key {
                VirtualKeyCode::Key1 => Some(0usize),
                VirtualKeyCode::Key2 => Some(1),
                VirtualKeyCode::Key3 => Some(2),
                _ => None,
            };
            if let Some(idx) = choice { self.app.choose_mixer(idx); }
            return;
        }

        if self.app.is_player_turn() {
            let idx = match key {
                VirtualKeyCode::Key1 => Some(0),
                VirtualKeyCode::Key2 => Some(1),
                VirtualKeyCode::Key3 => Some(2),
                VirtualKeyCode::Key4 => Some(3),
                VirtualKeyCode::Key5 => Some(4),
                _ => None,
            };
            if let Some(i) = idx { self.app.play_card(i); }
        }

        if key == VirtualKeyCode::R { self.app.reset(); }
        if key == VirtualKeyCode::Q { std::process::exit(0); }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Drunkard Brawl 🍺🥴")
        .build()?;

    main_loop(context, State { app: App::new() })
}