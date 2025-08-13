use crate::entity::types::StatsType;

#[derive(Debug, Clone)]
pub enum Potion {
    Healing(u32), // soin de X hp
    StatBoost {
        stat: StatsType, // type de stat à booster
        value: u32, 
        duration: u8 // nb de tours 
    }, 
}
