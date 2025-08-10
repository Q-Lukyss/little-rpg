use super::base::Player;

impl Player {
    fn set_xp_to_level_up(&mut self) {
        self.xp.1 = self.level * 100;
    }

    fn display_xp(&self) {
        println!("Xp : {} / {}", self.xp.0, self.xp.1);
    }

    pub fn gain_xp(&mut self, amount: u32) {
        println!("Vous avez gagné {} points d'xp", amount);
        self.xp.0 += amount;
        self.display_xp();

        let xp_to_level_up = self.xp.1;
        while self.xp.0 >= xp_to_level_up {
            self.xp.0 -= xp_to_level_up;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.set_xp_to_level_up();
        self.hp.1 += 1;
        self.hp.0 = self.hp.1;
        self.attack += 1;
        println!("{} a gagné un niveau !", self.name);
        println!("Toutes les stats ont pris +1");
        println!("PV soignés au max {}", self.hp.1);
        self.display_xp();
    }
}
