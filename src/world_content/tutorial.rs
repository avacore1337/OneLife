use crate::game::Game;

pub fn get_tutorial_texts() -> Vec<&'static str> {
    vec![
        "Welcome to the tutorial. You are born into a brutal life of slavery as an orphant.\
        You must now try to make the best of the sitution and try to live as long and happy life \
        as possible. But it's going to be tough, but try to make the best of it.\
        After all, you only have One Life",

        "You are breathing your last breaths. If you have any money left to spend, \
        you should probably spend them now on making sure you get a proper funeral.\
        otherwise your imortal soul might not make it accross the river Styx.\
        But you don't seem to have very much money...",

        "So, you died. Dirt poor and misserable that is. \
        You walk up to the ferry man, but without the money to pay him he waves you off.\
        As you walk along the banks of the river you feel as you start to lose yourself.\
        Your existence slowly dissapating into nothingness. After a long time you see an old man sitting in the corner.\
        You try to strike up a conversation. But he simply looks you deep in your eyes and asks \"First time?\".\
        \"First time what?\" you reply. \"Dying of course. Did you bring any coins?\" the old man replies.\
        \"No...\" you stammer out. \"Well, bring some next time.\" The man says before suddenly jumping up and hitting \
        you over the head with the blunt end of a stone sickle. \
        Everything fades to black and you hear him whisper \"One Life\"",

        "You are back to you 18:th birthday. But you remember having lived and died. \
        You feel that you can do better this time around. After all, you have already been through this once. \
        And this time, maybe you can even afford a funeral",

        "So, you died, but this time you brought coins! Not very mind you. \
            Turns out you were neither well liked nor bought a fancy grave. \
            Maybe try fixing that until next time you come here?",
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
    if info.tutorial_step == 1 && game.state.life_stats.is_dying {
        info.show_tutorial = true;
    }
    if info.tutorial_step == 2 && game.state.life_stats.dead {
        info.show_tutorial = true;
    }
    if info.tutorial_step == 3
        && !game.state.life_stats.dead
        && game.state.rebirth_stats.rebirth_count >= 1
    {
        info.show_tutorial = true;
    }
    if info.tutorial_step == 4
        && game.state.life_stats.dead
        && game.state.rebirth_stats.coins == 2.0
    {
        info.show_tutorial = true;
    }
}
