use bevy::prelude::*;
use bevy::ecs::component::Component;

use std::marker::PhantomData;

pub struct Remove<T: Component>(PhantomData<T>);

impl<T> Default for Remove<T>
where
    T: 'static + Send + Sync + Component,
{
    fn default() -> Self {
        Remove(PhantomData::<T>::default())
    }
}

pub struct EffectDespawn;


// Effect's target (player, mob, etc.)
pub struct EffectTarget(pub Entity);

impl EffectTarget {
    pub fn entity(&self) -> Entity {
        self.0
    }
}

pub struct FromAbility(Entity);
