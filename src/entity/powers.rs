#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum InnatePower {
    FlatAttack(i32),
    PercentAttack(f32),
    FlatDefense(i32),
    PercentDefense(f32),
    Lifesteal(f32),
    Bleed { damage: i32, turns: u8, chance: f32 },
    ActiveBite { power: i32, lifesteal: f32, cooldown: u8 },
}
