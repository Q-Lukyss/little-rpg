use crate::game_mecanics::{Location, Loot, Map};

pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: Vec<Loot>,
}

impl Quest {
    pub fn new(name: &str, description: &str, reward: Vec<Loot>) -> Self {
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
        vec![
            Loot::Map(Map::new(
                "Village du héros".to_string(),
                "Un village paisible, le premier que vous visitez.".to_string(),
                Location::City("Village du héros".to_string()),
                None,
            )),
            Loot::Gold(100),
        ],
    )
}

pub fn kill_10_gobelin() -> Quest {
    // Quest::new(
    //     "Tuer 10 Gobelins",
    //     "Tuer 10 Gobelins pour obtenir le Parchemin des Goblins.",
    //     vec![Loot::Key("Clé du coffre".to_string()), Loot::Gold(50)],
    // )
    // Il faudrait ajouter un traqueur de progression
    // Il faudra aussi ajouter un reward en Xp
    todo!("Quest tuer 10 gobelins")
}
