use bevy::core::{FixedTimestep, FixedTimesteps};
use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Health(u32);

#[derive(Debug, Clone, Default)]
pub struct MaxHealth(u32);

#[derive(Debug, Clone, Default)]
pub struct HealthRegen(u32);

/*
pub struct Armor(u32);
pub struct MagicResist(u32);

pub struct PhysicalPenetration(u32);
pub struct MagicPenetration(u32);

pub struct MovementSpeed(u32);
pub struct AttackSpeed(u32);
*/

#[derive(Bundle)]
pub struct PlayerStatBundle {
    health: Health,
    max_health: MaxHealth,
    health_regen: HealthRegen,
    /*
    armor: Armor,
    magic_resist: MagicResist,

    physical_penetration: PhysicalPenetration,
    magic_penetration: MagicPenetration,

    movement_speed: MovementSpeed,
    attack_speed: AttackSpeed,
    */
}

fn regen_health(mut query: Query<(&mut Health, &MaxHealth, &HealthRegen)>) {
    for (mut health, max_health, regen) in query.iter_mut() {
        if health.0 > 0 && health.0 < max_health.0 {
            health.0 = max_health.0.min(health.0 + regen.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn health_regen() {
        let mut app_builder = App::build();
        let mut app = std::mem::take(
            &mut app_builder
                .add_plugins(MinimalPlugins)
                //.add_plugins(DefaultPlugins)
                .add_stage(
                    CoreStage::PostUpdate,
                    SystemStage::parallel()
                        .with_run_criteria(FixedTimestep::step(1.0).with_label("every_second"))
                        .with_system(regen_health.system()),
                )
                .app,
        );

        let player = app
            .world
            .spawn()
            .insert(Health(1))
            .insert(MaxHealth(10))
            .insert(HealthRegen(5))
            .id();

        let dead_player = app
            .world
            .spawn()
            .insert(Health(0))
            .insert(MaxHealth(10))
            .insert(HealthRegen(5))
            .id();

        assert_eq!(app.world.get::<Health>(player).unwrap().0, 1);
        assert_eq!(app.world.get::<Health>(dead_player).unwrap().0, 0);

        app.update();
        assert_eq!(app.world.get::<Health>(player).unwrap().0, 1);

        std::thread::sleep(Duration::from_secs(1));
        app.update();
        assert_eq!(app.world.get::<Health>(player).unwrap().0, 6);

        std::thread::sleep(Duration::from_secs(1));
        app.update();

        assert_eq!(app.world.get::<Health>(player).unwrap().0, 10);
        assert_eq!(app.world.get::<Health>(dead_player).unwrap().0, 0);
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
