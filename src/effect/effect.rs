
use bevy::prelude::*;

// Effect's target (player, mob, etc.)
pub struct EffectTarget(pub Entity);

impl EffectTarget {
    pub fn entity(&self) -> Entity {
        self.0
    }
}

pub struct FromAbility(Entity);