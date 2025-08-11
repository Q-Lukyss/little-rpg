#[derive(Debug, Clone, PartialEq)]
pub enum Stat {
    Attack,
    Hp,
    MaxHp,
    Crit,
    Defense,
    Level
}

// a terme remplacera l enum je pense
pub struct Stats {
    attack : i32,
    hp : i32,
    maxHp : i32,
    crit : f32,
    defense : i32,
    xp : i32,
    xp_to_level_up : i32,
    level : u8, // max 255 suffiant
} 
