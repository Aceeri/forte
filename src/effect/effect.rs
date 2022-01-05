use bevy::prelude::*;

use std::collections::HashMap;


#[derive(Clone, Debug, Default)]
pub struct EffectRelations(pub HashMap<Entity, Entity>);

pub fn cache_relations(mut relations: ResMut<EffectRelations>, query: Query<(Entity, &EffectTarget), Changed<EffectTarget>>) {
    for (effect, target) in query.iter() {
        relations.0.insert(effect, target.entity());
    }
}

pub fn cleanup_relations(mut relations: ResMut<EffectRelations>, query: RemovedComponents<&EffectTarget>) {
    for entity in query.iter() {
        relations.0.remove(&entity);
    }
}

// Effect's target (player, mob, etc.)
pub struct EffectTarget(pub Entity);

impl EffectTarget {
    pub fn entity(&self) -> Entity {
        self.0
    }
}

pub struct FromAbility(Entity);
