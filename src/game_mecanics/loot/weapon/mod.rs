#[derive(Debug, Clone)]
pub enum WeaponType {
    Sword,
    Axe,
    Greatsword,
    Dagger,
    Spear,
}
#[derive(Debug, Clone)]
pub enum Handling {
    OneHanded,
    TwoHanded,
}
#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub description: String,
    pub damage: u32,
    pub weapon_type: WeaponType,
}

impl Weapon {
    pub fn new(name: String, description: String, damage: u32, weapon_type: WeaponType) -> Self {
        Weapon {
            name,
            description,
            damage,
            weapon_type,
        }
    }

    pub fn get_first_weapon() -> Weapon {
        Weapon::new(
            "Épée rouillée".to_string(),
            "Une épée sans doute affutée jadis, mais hélas le temps ne l'as pas épargé".to_string(),
            5,
            WeaponType::Sword,
        )
    }
}
