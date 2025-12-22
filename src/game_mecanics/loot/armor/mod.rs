#[derive(Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub description: String,
    pub defense: u32,
}

impl Armor {
    pub fn new(name: String, description: String, defense: u32) -> Armor {
        Armor {
            name,
            description,
            defense,
        }
    }

    pub fn get_first_armor() -> Armor {
        Armor::new(
            "Armure rouillée".to_string(),
            "Une armure rouillées, elle n'est pas très agréable mais elle me protegera toujours plus que ma peau nue"
                .to_string(),
            10,
        )
    }
}
