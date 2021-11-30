use crate::game::Game;

pub fn get_tutorial_texts() -> Vec<&'static str> {
    vec![
        "Welcome to the tutorial",
        "So, you died",
        "So, you died, but this time you brought money!",
        "how did you get here?",
    ]
}

pub fn check_for_tutorial_step(game: &mut Game) {
    let info = &mut game.meta_data.info;
    if info.disable_tutorial {
        return;
    }
    if info.tutorial_step == 0 {
        info.show_tutorial = true;
    }
    if info.tutorial_step == 1 && game.state.life_stats.dead {
        info.show_tutorial = true;
    }
    if info.tutorial_step == 2
        && game.state.life_stats.dead
        && game.state.rebirth_stats.coins == 2.0
    {
        info.show_tutorial = true;
    }
}
