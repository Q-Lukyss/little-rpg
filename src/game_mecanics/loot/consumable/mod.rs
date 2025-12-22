#[derive(Debug, Clone)]
pub enum Consumable {
    Potion(Potion),
    Elixir(ElixirType),
    Key,
    QuestItem,
}
#[derive(Debug, Clone)]
pub enum ElixirType {
    Damage,
    Health,
    Defense,
    Speed,
}

#[derive(Debug, Clone)]
pub struct Potion {
    pub name: String,
    pub description: String,
    pub heal_amount: u32,
}

impl Potion {
    pub fn new(name: String, description: String, heal_amount: u32) -> Self {
        Potion {
            name,
            description,
            heal_amount,
        }
    }
    pub fn get_basic_potion() -> Potion {
        Potion::new(
            "Potion basique".into(),
            "Une potion de base qui rend 25 points de vie".into(),
            25,
        )
    }
}
