use bevy::ecs::component::Component;
use bevy::prelude::*;

use std::marker::PhantomData;

#[derive(Component, PartialEq, Debug, Clone)]
pub struct Remove<T: Component>(PhantomData<T>);

impl<T> Default for Remove<T>
where
    T: 'static + Send + Sync + Component,
{
    fn default() -> Self {
        Remove(PhantomData::<T>::default())
    }
}

#[derive(Component, PartialEq, Debug, Clone)]
pub struct Despawn;

pub fn cleanup_despawning(mut commands: Commands, despawning: Query<Entity, With<Despawn>>) {
    for entity in despawning.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_removing<T: 'static + Send + Sync + Component>(
    mut commands: Commands,
    removing: Query<Entity, With<Remove<T>>>,
) {
    for entity in removing.iter() {
        commands.entity(entity).remove::<T>();
    }
}

// Effect's target (player, mob, etc.)
#[derive(Component, PartialEq, Debug, Clone)]
pub struct EffectTarget(pub Entity);

impl EffectTarget {
    pub fn entity(&self) -> Entity {
        self.0
    }
}

#[derive(Component, PartialEq, Debug, Clone)]
pub struct FromAbility(Entity);
