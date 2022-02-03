use bevy::prelude::*;
use std::collections::VecDeque;
use std::marker::PhantomData;
use vec_collections::VecMap;

#[derive(Component, Debug, Clone)]
pub struct DamageHistory {
    history: VecDeque<Damage>,
}

impl DamageHistory {
    fn push(&mut self, damage: Damage) {
        self.history.push_back(damage);
    }
}

#[derive(Debug, Clone)]
pub struct Damage {
    pub from: Entity,
    pub amount: u32,
    //pub ability: Entity,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Health {
    amount: u32,
}

impl Health {
    pub fn new(health: u32) -> Self {
        Self { amount: health }
    }

    pub fn damage(&mut self, time: &Time, history: &mut DamageHistory, damage: Damage) {
        self.amount = self.amount.saturating_sub(damage.amount);
        history.push(damage);
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: u32) {
        self.amount = amount;
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct MaxHealth(u32);

#[derive(Component, Debug, Clone, Default)]
pub struct HealthRegen(u32);

pub fn regen(mut query: Query<(&mut Health, &MaxHealth, &HealthRegen)>) {
    for (mut health, max_health, regen) in query.iter_mut() {
        let current = health.amount();
        let max = max_health.0;
        let regen = regen.0;
        if current > 0 && current < max {
            health.set_amount(max.min(current + regen));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::core::FixedTimestep;
    use std::time::Duration;

    #[test]
    fn health_regen() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_system_to_stage(
            CoreStage::Update,
            regen
                .system()
                .with_run_criteria(FixedTimestep::step(0.6).with_label("100bpm")),
        );

        let player = app
            .world
            .spawn()
            .insert(Health::new(1))
            .insert(MaxHealth(10))
            .insert(HealthRegen(5))
            .id();

        let dead_player = app
            .world
            .spawn()
            .insert(Health::new(0))
            .insert(MaxHealth(10))
            .insert(HealthRegen(5))
            .id();

        assert_eq!(app.world.get::<Health>(player).unwrap().amount(), 1);
        assert_eq!(app.world.get::<Health>(dead_player).unwrap().amount(), 0);

        app.update();
        assert_eq!(app.world.get::<Health>(player).unwrap().amount(), 1);

        let bpm = Duration::from_millis(600); // 0.6 seconds is 100bpm
        std::thread::sleep(bpm);
        app.update();
        assert_eq!(app.world.get::<Health>(player).unwrap().amount(), 6);

        std::thread::sleep(bpm);
        app.update();

        assert_eq!(app.world.get::<Health>(player).unwrap().amount(), 10);
        assert_eq!(app.world.get::<Health>(dead_player).unwrap().amount(), 0);
    }
}

/*
attack_damage : Attack Damage,
ability_power : Ability Power,
armor : Armor,
magic_resist : Magic Resist,
health_max : Health,
resource_max : Resource,
health_regen : Health Regen,
resource_regen : Resource Regen,
movement_speed : Movement Speed,
magic_pen : Magic Penatration,
phys_pen : Physical Penatration,

cdr : Cooldown Reduction,
crit : Critical Strike Chance,
crit_damage : Critical Strike Damage,
attack_speed : Attack Speed,
lifesteal : Lifesteal,
spell_vamp : Spell Vamp,

heal_power : Healing Power,
shield_power : Shield Power,
tenacity : Tenacity,

damage_reduction : Damage Reduction,
damage_dealt : Damage Dealt,
damage_reflect : Damaged Reflect,

xp : XP,
xpgain : XP Gain,
gold : Gold,
goldgain : Gold Gain,
level : Level,
*/
