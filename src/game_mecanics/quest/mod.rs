use crate::game_mecanics::{Location, Loot};

pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: Option<Loot>,
}

impl Quest {
    pub fn new(name: &str, description: &str, reward: Option<Loot>) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            reward: reward,
        }
    }
}

pub fn first_quest() -> Quest {
    Quest::new(
        "Vaincre mon premier ennemi",
        "Participer un votre premier combat et sortez en victorieux!",
        Some(Loot::Map(Location::City("Village du héros".to_string()))),
    )
}
