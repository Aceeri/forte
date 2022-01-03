
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
            dbg!();
            if let Ok(effect_target) = effect_targets.get_component::<EffectTarget>(entity) {
                if let Ok(mut stacks) = stacks.get_component_mut::<Self>(effect_target.entity()) {
                    stacks.remove(entity);
                }
            }
        }
    }

    fn apply_stack(
        mut stacks: Query<&mut Self>,
        added: Query<(&Self::EffectComponent, &EffectTarget), Added<Self::EffectComponent>>,
    ) {
        for (component, target) in added.iter() {
            if let Ok(mut stacks) = stacks.get_component_mut::<Self>(target.entity()) {
                dbg!();
                stacks.apply(component);
            }
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
#[derive(Clone, Debug, Default)]
pub struct StunStacks(u16);

// Effect component.
#[derive(Clone, Debug, Default)]
pub struct Stun;

// Target effect component.
#[derive(Clone, Debug, Default)]
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

// Re-applying the same effect will only up the timer.
pub struct Burn(u64); // time in milliseconds.

impl EffectStack for Burn {
    type EffectComponent = Burn;
    type TargetEffectComponent = Burn;
    fn apply(&mut self, other: &Self::EffectComponent) {
        if other.0 > self.0 {
            self.0 = other.0;
        }
    }
    fn remove(&mut self, _entity: Entity) { }
    fn alive(&self) -> bool {
        self.0 > 0
    }
    fn target_effect(&self) -> Self::TargetEffectComponent {
        Burn(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_stuns() {
        let mut world = World::default();
        
        let mut stage = SystemStage::parallel();
        stage.add_system(StunStacks::apply_stack.system());
        stage.add_system(StunStacks::remove_stack.system());
        stage.add_system(StunStacks::modified_stacks.system());

        stage.run(&mut world);

        let ability = world.spawn().id();
        let target = world.spawn()
            .insert(StunStacks::default())
            .id();
        let effect = world.spawn()
            .insert(EffectTarget(target))
            .insert(Stun)
            .id();

        dbg!(world.get::<StunStacks>(target));
        assert_eq!(world.get::<StunStacks>(target).unwrap().0, 0);

        stage.run(&mut world);
        dbg!(world.get::<StunStacks>(target));
        assert_eq!(world.get::<StunStacks>(target).unwrap().0, 1);

        stage.run(&mut world);
        dbg!(world.get::<StunStacks>(target));
        assert_eq!(world.get::<StunStacks>(target).unwrap().0, 1);
    }
}
