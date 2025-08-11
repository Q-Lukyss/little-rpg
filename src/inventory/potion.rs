use crate::player::stats::Stat;

#[derive(Debug, Clone)]
pub enum Potion {
    Healing(u32), // soin de X hp
    StatBoost { stat: Stat, value: u32, duration: u8 }, // augmente une stat
}
