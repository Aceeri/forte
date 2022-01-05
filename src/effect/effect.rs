use bevy::prelude::*;
use bevy::ecs::component::Component;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use fxhash::FxHashMap;

#[derive(Clone, Debug)]
pub struct Relations<T>
where 
    T: 'static + Send + Sync + Component + Relation,
{
    relations: FxHashMap<Entity, Entity>,
    phantom: PhantomData<T>,
}

impl<T> Default for Relations<T>
where 
    T: 'static + Send + Sync + Component + Relation,
{
    fn default() -> Self {
        Self {
            relations: FxHashMap::default(),
            phantom: PhantomData::<T>::default(),
        }
    }
}

pub trait Relation {
    fn entity(&self) -> Entity;
} 

impl<T> Deref for Relations<T>
where 
    T: 'static + Send + Sync + Component + Relation,
{
    type Target = FxHashMap<Entity, Entity>;
    fn deref(&self) -> &Self::Target {
        &self.relations
    }
}

impl<T> DerefMut for Relations<T>
where 
    T: 'static + Send + Sync + Component + Relation,
    Self: Deref<Target = FxHashMap<Entity, Entity>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.relations
    }
}

impl<T> Relations<T>
where 
    T: 'static + Send + Sync + Component + Relation,
{
    pub fn cache(mut relations: ResMut<Relations<T>>, query: Query<(Entity, &T), Changed<T>>) {
        for (effect, target) in query.iter() {
            relations.insert(effect, target.entity());
        }
    }

    pub fn cleanup(mut relations: ResMut<Relations<T>>, query: RemovedComponents<&T>) {
        for entity in query.iter() {
            relations.remove(&entity);
        }
    }
}

// Effect's target (player, mob, etc.)
pub struct EffectTarget(pub Entity);

impl Relation for EffectTarget {
    fn entity(&self) -> Entity {
        self.0
    }
}

impl EffectTarget {
    pub fn entity(&self) -> Entity {
        self.0
    }
}

pub struct FromAbility(Entity);
