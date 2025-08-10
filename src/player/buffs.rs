use super::base::{Player, ActiveBuff};
use crate::inventory::{Potion, Stat};

impl Player {
    pub fn use_potion(&mut self, potion: Potion) {
        match potion {
            Potion::Healing(amount) => {
                self.hp.0 = (self.hp.0 + amount as i32).min(self.hp.1);
                println!("Tu récupères {} PV ! ({}/{})", amount, self.hp.0, self.hp.1);
            }
            Potion::StatBoost { stat, value, duration } => {
                println!("Ton/ta {:?} augmente de {} pour {} tours !", stat, value, duration);
                self.buffs.push(ActiveBuff { stat: stat.clone(), value, remaining_turns: duration });

                if stat == Stat::Attack {
                    self.attack += value as i32;
                }
            }
        }
    }

    pub fn update_buffs(&mut self) {
        let mut expired = vec![];

        for (i, buff) in self.buffs.iter_mut().enumerate() {
            if buff.remaining_turns > 0 { buff.remaining_turns -= 1; }

            if buff.remaining_turns == 0 {
                println!("L'effet du boost sur {:?} a expiré.", buff.stat);
                if buff.stat == Stat::Attack {
                    self.attack = self.attack.saturating_sub(buff.value as i32);
                }
                expired.push(i);
            }
        }

        for &index in expired.iter().rev() {
            self.buffs.remove(index);
        }
    }
}
