#[derive(Debug, Clone)]
pub struct Map {
    pub name: String,
    pub description: String,
    pub location_type: Location,
    pub price: Option<u32>,
}

impl Map {
    pub fn new(
        name: String,
        description: String,
        location_type: Location,
        price: Option<u32>,
    ) -> Self {
        Map {
            name,
            description,
            location_type,
            price,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Location {
    City(String),
    Dungeon(String),
}
