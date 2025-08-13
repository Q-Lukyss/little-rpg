use super::types::{Entity, Stats, Equipment, Kind, PlayerData, ActiveBuff, StatsType};
use crate::inventory::{Item, Potion, Weapon, WeaponKind, shield::Shield, weapon::{OffHand, Wield}};
use super::traits::{AttackProfile};
use super::combat::resolve_attack;

pub fn make_player(name: String) -> Entity {
    Entity {
        name: Some(name),
        stats: Stats {
            hp: 20, 
            max_hp: 20,
            attack: 0,
            speed: 1, 
            defense: 0,
            crit: 0.05, 
            level: 1,
            xp: Some(0), xp_to_level: Some(100),
        },
        equipment: Equipment {
            main: Weapon { kind: WeaponKind::Sword, base_damage: 5, dmg_reduction_parry: 1 },
            off: Some(OffHand::Shield(Shield { dmg_reduction: 5 })),
        },
        buffs: vec![],
        kind: Kind::Player(PlayerData {
            inventory: vec![Item::Potion(Potion::Healing(10))],
        }),
    }
}

// ---------- Vue "Player" pour exposer les méthodes spécifiques joueur ----------
pub struct PlayerView<'a> {
    pub e: &'a mut Entity,
}

impl Entity {
    /// Retourne une vue mutable si (et seulement si) l'entité est un joueur.
    pub fn as_player_mut(&mut self) -> Option<PlayerView<'_>> {
        match self.kind {
            Kind::Player(_) => Some(PlayerView { e: self }),
            _ => None,
        }
    }
}

impl<'a> PlayerView<'a> {
    // ---- Potions & Buffs ----

    /// Utiliser une potion (depuis n’importe où).
    pub fn use_potion(&mut self, potion: Potion) {
        match potion {
            Potion::Healing(amount) => {
                let before = self.e.stats.hp;
                self.e.stats.hp = (self.e.stats.hp + amount as u32).min(self.e.stats.max_hp);
                println!("Tu récupères {} PV ! ({}/{})", self.e.stats.hp - before, self.e.stats.hp, self.e.stats.max_hp);
            }
            Potion::StatBoost { stat, value, duration } => {
                println!("Ton/ta {:?} augmente de {} pour {} tours !", stat, value, duration);
                self.e.buffs.push(ActiveBuff { stat: stat.clone(), value, remaining_turns: duration });

                // Comportement legacy : bonus immédiat sur la stat brute
                if stat == StatsType::Attack {
                    self.e.stats.attack += value as i32;
                }
            }
        }
    }

    /// Consommer une potion depuis l’inventaire du joueur si elle respecte `predicate`.
    /// Retourne true si une potion a été consommée.
    pub fn use_potion_from_inventory(&mut self, predicate: impl Fn(&Potion) -> bool) -> bool {
        let inv = match &mut self.e.kind {
            Kind::Player(PlayerData { inventory }) => inventory,
            _ => return false,
        };
        if let Some(idx) = inv.iter().position(|it| matches!(it, Item::Potion(p) if predicate(p))) {
            if let Item::Potion(p) = inv.remove(idx) {
                self.use_potion(p);
                return true;
            }
        }
        false
    }

    /// Décrémente la durée des buffs et retire les expirés (revert l’effet immédiat).
    pub fn update_buffs(&mut self) {
        let mut expired = vec![];
        for (i, buff) in self.e.buffs.iter_mut().enumerate() {
            if buff.remaining_turns > 0 { buff.remaining_turns -= 1; }
            if buff.remaining_turns == 0 {
                println!("L'effet du boost sur {:?} a expiré.", buff.stat);
                if buff.stat == StatsType::Attack {
                    self.e.stats.attack = self.e.stats.attack.saturating_sub(buff.value as i32);
                }
                expired.push(i);
            }
        }
        for &i in expired.iter().rev() {
            self.e.buffs.remove(i);
        }
    }

    // ---- Combat haut-niveau (compat Player::attack/defense) ----

    /// Attaquer une autre entité (renvoie les dégâts infligés).
    pub fn attack_entity(&self, defender: &mut Entity) -> i32 {
        resolve_attack(self.e, defender)
    }

    /// Se défendre contre un attaquant (applique les dégâts subis et les affiche).
    pub fn defend_against(&mut self, attacker: &Entity) -> i32 {
        let raw = attacker.attack_value() - self.e.defense_value();
        let dmg = raw.max(0);
        match dmg {
            0 => println!("Tu as bloqué toute l'attaque de l'ennemi !"),
            n => {
                println!("Tu as bloqué une partie de l'attaque de l'ennemi, tu perds {} hp", n);
            }
        }
        self.e.stats.take_damage(dmg);
        println!("Il te reste {} hp", self.e.stats.hp);
        dmg
    }

    // ---- Équipement ----

    /// Équiper une nouvelle arme principale.
    pub fn equip_weapon(&mut self, weapon: Weapon) {
        self.e.equipment.main = weapon;
        self.enforce_two_hands();
    }

    /// Retire l’off-hand si l’arme principale est à deux mains.
    pub fn enforce_two_hands(&mut self) {
        if let Wield::TwoHands = self.e.equipment.main.kind.get_weapon_wielding() {
            self.e.equipment.off = None;
        }
    }

    // ---- Cheats / pouvoirs spéciaux ----

    /// Pouvoir secret : tue instantanément la cible.
    pub fn demonic_eye(&self, target: &mut Entity) {
        println!("Une douleur soudaine s'empare de vous!");
        println!("Votre Oeil ! une lueur rouge s'en échappe !");
        println!("Vos forces vous abandonnent soudainement...");
        println!("...");
        println!("Les ennemis sont consumés par un mystérieux pouvoir");
        target.stats.hp = 0;
    }

    // ---- XP & Level ----

    fn set_xp_to_level_up(&mut self) {
        self.e.stats.xp_to_level = Some((self.e.stats.level as u32) * 100);
    }

    fn display_xp(&self) {
        if let (Some(xp), Some(to)) = (self.e.stats.xp, self.e.stats.xp_to_level) {
            println!("Xp : {} / {}", xp, to);
        }
    }

    pub fn gain_xp(&mut self, amount: u32) {
        let (Some(mut xp), Some(mut to)) = (self.e.stats.xp, self.e.stats.xp_to_level) else {
            // Non-joueur (ou XP désactivée) : noop
            return;
        };

        println!("Vous avez gagné {} points d'xp", amount);
        xp += amount;
        self.e.stats.xp = Some(xp);
        self.display_xp();

        while xp >= to {
            xp -= to;
            self.level_up();
            to = (self.e.stats.level as u32) * 100;
        }
        self.e.stats.xp = Some(xp);
        self.e.stats.xp_to_level = Some(to);
    }

    pub fn level_up(&mut self) {
        self.e.stats.level = self.e.stats.level.saturating_add(1);
        self.set_xp_to_level_up();
        self.e.stats.max_hp = self.e.stats.max_hp.saturating_add(1);
        self.e.stats.hp = self.e.stats.max_hp;
        self.e.stats.attack = self.e.stats.attack.saturating_add(1);

        println!("{} a gagné un niveau !", self.e.name.as_deref().unwrap_or("Le héros"));
        println!("Toutes les stats ont pris +1");
        println!("PV soignés au max {}", self.e.stats.max_hp);
        self.display_xp();
    }
}
