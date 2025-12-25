#[derive(Debug, Clone)]
pub struct Trinket {
    pub name: String,
    pub description: String,
    pub power: Option<Power>,
}

#[derive(Debug, Clone)]
pub enum Power {}

impl Trinket {
    pub fn new(name: String, description: String, power: Option<Power>) -> Trinket {
        Trinket {
            name,
            description,
            power,
        }
    }

    pub fn get_first_shield() -> Trinket {
        Trinket::new(
            "Collier rouillée".to_string(),
            "un collier rouillée".to_string(),
            None,
        )
    }
}
