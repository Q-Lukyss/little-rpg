#[derive(Debug, Clone)]
pub struct Stat {
    pub hp: u32,
    pub max_hp: u32,
    pub base_attack: u32,
    pub base_defense: u32,
    // pub powers: Vec<Power>,
}

impl Stat {
    pub fn new(hp: u32, max_hp: u32, base_attack: u32, base_defense: u32) -> Self {
        Stat {
            hp: hp,
            max_hp: max_hp,
            base_attack: base_attack,
            base_defense: base_defense,
            // powers: Vec::new(),
        }
    }
}
