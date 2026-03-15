#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_card_applies_damage() {
        let mut app = App::new();
        let initial_player = app.player_hp();
        let initial_opp = app.opponent_hp();

        app.play_card(0);  // Pliny: self -4, opp -13

        assert_eq!(app.player_hp(), initial_player - 4);
        assert_eq!(app.opponent_hp(), initial_opp - 13);
    }

    #[test]
    fn test_game_over_detection() {
        let mut app = App::new();
        app.player_hp = 0;
        assert!(app.is_game_over());
    }
}