
use bevy::{prelude::*, ecs::component::Component};

use crate::effect::*;

pub trait EffectStack
where
    Self: 'static + Sized + Send + Sync,
{
    type EffectComponent: Component;
    type TargetEffectComponent: Component;

    fn apply(&mut self, comp: &Self::EffectComponent);
    fn remove(&mut self, entity: Entity);
    fn alive(&self) -> bool;
    fn target_effect(&self) -> Self::TargetEffectComponent;

    fn remove_stack(
        mut stacks: Query<&mut Self>,
        effect_targets: Query<&EffectTarget>,
        removed: RemovedComponents<Self::EffectComponent>,
    ) {
        for entity in removed.iter() {
            if let Ok(effect_target) = effect_targets.get_component::<EffectTarget>(entity) {
                if let Ok(mut stacks) = stacks.get_component_mut::<StunStacks>(effect_target.entity()) {
                    stacks.remove(entity);
                }
            }
        }
    }

    fn apply_stack(
        mut added: Query<(&mut Self, &Self::EffectComponent, With<EffectTarget>), Added<Self::EffectComponent>>,
    ) {
        for (mut stacks, effect_component, _) in added.iter_mut() {
            stacks.apply(effect_component);
        }
    }

    fn modified_stacks(
        mut commands: Commands,
        stacks: Query<(&Self, Entity), Changed<Self>>,
    ) {
        for (stack, entity) in stacks.iter() {
            if stack.alive() {
                commands.entity(entity).remove::<Self::TargetEffectComponent>();
            } else {
                commands.entity(entity).insert(stack.target_effect());
            }
        }
    }
}


// Control how stacking of the same effect works.
pub struct StunStacks(u16);

// Effect component.
pub struct Stun;

// Target effect component.
pub struct Stunned;

impl EffectStack for StunStacks {
    type EffectComponent = Stun;
    type TargetEffectComponent = Stunned;
    fn apply(&mut self, _comp: &Self::EffectComponent) {
        self.0 = self.0.saturating_add(1);
    }
    fn remove(&mut self, _entity: Entity) {
        self.0 = self.0.saturating_sub(1);
    }
    fn alive(&self) -> bool {
        self.0 > 0
    }
    fn target_effect(&self) -> Self::TargetEffectComponent {
        Stunned
    }
}
