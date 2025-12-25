use rand::Rng;

#[derive(Debug, Clone)]
pub enum WeaponType {
    Sword,
    Axe,
    Greatsword,
    Dagger,
    Spear,
}

impl WeaponType {
    pub fn handling(&self) -> Handling {
        match self {
            WeaponType::Greatsword => Handling::TwoHanded,
            _ => Handling::OneHanded,
        }
    }
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

    pub fn handling(&self) -> Handling {
        self.weapon_type.handling()
    }

    pub fn get_first_weapon() -> Weapon {
        Weapon::new(
            "Épée rouillée".to_string(),
            "Une épée sans doute affutée jadis, mais hélas le temps ne l'as pas épargé".to_string(),
            5,
            WeaponType::Sword,
        )
    }
    pub fn get_random_rusted_weapon() -> Weapon {
        let roll = rand::rng().random_range(0..5);
        match roll {
            0 => Weapon::new(
                "Épée rouillée".to_string(),
                "Une épée sans doute affutée jadis, mais hélas le temps ne l'as pas épargé"
                    .to_string(),
                5,
                WeaponType::Sword,
            ),
            1 => Weapon::new(
                "Hache rouillée".to_string(),
                "Une hache sans doute affutée jadis, mais hélas le temps ne l'as pas épargé"
                    .to_string(),
                5,
                WeaponType::Axe,
            ),
            2 => Weapon::new(
                "Grande-Épée rouillée".to_string(),
                "Une grande épée sans doute affutée jadis, mais hélas le temps ne l'as pas épargé"
                    .to_string(),
                5,
                WeaponType::Greatsword,
            ),
            3 => Weapon::new(
                "Dague rouillée".to_string(),
                "Une dague sans doute affutée jadis, mais hélas le temps ne l'as pas épargé"
                    .to_string(),
                5,
                WeaponType::Dagger,
            ),
            4 => Weapon::new(
                "Lance rouillée".to_string(),
                "Une lance sans doute affutée jadis, mais hélas le temps ne l'as pas épargé"
                    .to_string(),
                5,
                WeaponType::Spear,
            ),
            _ => unreachable!(),
        }
    }
}
