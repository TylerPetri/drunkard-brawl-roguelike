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
            self.app.advance_turn(); // AI acts immediately
        }

        self.draw_ui(ctx);

        if let Some(key) = ctx.key {
            self.handle_input(key);
        }
    }
}

impl State {
    fn draw_ui(&self, ctx: &mut BTerm) {
        // Title - centered, yellow fg, black bg
        ctx.print_color_centered(
            1,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            "=== DRUNKARD BRAWL 🍺🥴 ===",
        );

        // HP lines - note: format! returns String, but print_color accepts &str via &
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

        // Message
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
                "GAME OVER — you blacked out or they did!",
            );
            return;
        }

        // Box around hand choices
        ctx.draw_box(3, 16, 74, 13, RGB::named(WHITE), RGB::named(BLACK));

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

        ctx.print_color(
            6,
            29,
            RGB::named(GRAY),
            RGB::named(BLACK),
            "Q = Quit  |  R = Restart",
        );
    }

    fn handle_input(&mut self, key: VirtualKeyCode) {
        if self.app.is_game_over() {
            match key {
                VirtualKeyCode::R => self.app = App::new(),
                VirtualKeyCode::Q => std::process::exit(0),
                _ => {}
            }
            return;
        }

        if !self.app.is_player_turn() {
            // Ignore input during AI turn (or future phases)
            return;
        }

        let index = match key {
            VirtualKeyCode::Key1 => Some(0),
            VirtualKeyCode::Key2 => Some(1),
            VirtualKeyCode::Key3 => Some(2),
            VirtualKeyCode::Key4 => Some(3),
            VirtualKeyCode::Key5 => Some(4),
            _ => None,
        };

        if let Some(idx) = index {
            self.app.play_card(idx);
            // No need for self.app.advance_turn() here anymore —
            // advance_turn is now called in tick when phase changes
        } else {
            match key {
                VirtualKeyCode::R => self.app = App::new(),
                VirtualKeyCode::Q => std::process::exit(0),
                _ => {}
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Drunkard Brawl 🍺🥴")
        .build()?;

    main_loop(context, State { app: App::new() })
}
