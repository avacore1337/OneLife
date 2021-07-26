use super::tier::Tier;

struct world {
    tiers: Vec<Tier>,
}

fn init_tiers() -> Vec<Tier> {
    // T1 Orphant slave
    // T2 Slave
    // T3 Farmless Peasant
    // T4 Estate peasant
    // T5 Landowning peasant
    // T6 Farm owner
    // T7 Estate owner
    // T8 Grand estate owner
    // T9 low Noble
    // T10 middle Noble
    // T11 high Noble
    // T12 Regent
    // T13 King
    // T14 King of Kings
    // T15 The August one.

    return vec![Tier {
        title: "Orphant slave".to_string(),
        desrcription: "The unluckies souls in this harsh world".to_string(),
        unlocks: vec!["Only the basics of life itself is available".to_string()],
    }];
}
