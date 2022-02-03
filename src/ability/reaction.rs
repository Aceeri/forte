
use bevy::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub struct FromAbility(Entity);

#[derive(Bundle, Clone)]
pub struct AbilityDefinition {
    stun: Option<Stun>,
    projectile_speed: Option<ProjectileSpeed>,
    projectile_direction: Option<ProjectileDirection>,
}

#[derive(Component)]
pub struct OnHit(AbilityDefinition);

fn on_hit(
    mut commands: Commands,
    on_hit: Query<(&OnHit, Entity)>,
) {
    for (hit, entity) in on_hit.iter() {
        // Probably need to integrate rapier/physics here to check for the collision somehow.
        commands.spawn_bundle(hit.0.clone())
            .insert(FromAbility(entity));
    }
}