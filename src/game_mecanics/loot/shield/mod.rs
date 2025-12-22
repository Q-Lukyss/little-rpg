use crate::game_mecanics::Handling;

#[derive(Debug, Clone)]
pub struct Shield {
    pub name: String,
    pub description: String,
    pub defense: u32,
    pub handling: Handling,
}

impl Shield {
    pub fn new(name: String, description: String, defense: u32, handling: Handling) -> Shield {
        Shield {
            name,
            description,
            defense,
            handling,
        }
    }

    pub fn get_first_shield() -> Shield {
        Shield::new(
            "Bouclier rouillée".to_string(),
            "Un bouclier rouillée, on peut apercevoir des trous... mieux que rien j'imagine"
                .to_string(),
            5,
            Handling::OneHanded,
        )
    }
}
