use super::traits::{AttackProfile, HasStats};

pub fn resolve_attack<A, D>(attacker: &A, defender: &mut D) -> i32
where
    A: AttackProfile,
    D: AttackProfile + HasStats,
{
    let raw = attacker.attack_value() - defender.defense_value();
    let dmg = raw.max(0);
    defender.stats_mut().take_damage(dmg);
    dmg
}
