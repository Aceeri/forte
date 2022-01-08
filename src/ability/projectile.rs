use bevy::prelude::*;

#[derive(Component, PartialEq, Debug, Clone)]
pub struct ProjectileSpeed(u32);

#[derive(Component, PartialEq, Debug, Clone)]
pub struct ProjectileDirection(Vec3);

fn move_projectile(
    mut projectiles: Query<(&mut Transform, &ProjectileSpeed, &ProjectileDirection)>,
) {
    for (mut transform, speed, direction) in projectiles.iter_mut() {
        transform.translation += direction.0 * speed.0 as f32;
    }
}
