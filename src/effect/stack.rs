use bevy::{ecs::component::Component, prelude::*};

use crate::effect::*;
use smolset::SmolSet;

pub trait EffectStack
where
    Self: 'static + Sized + Send + Sync,
{
    type EffectComponent: Component;
    type TargetEffectComponent: Component;

    fn apply(&mut self, comp: &Self::EffectComponent, entity: Entity);
    fn remove(&mut self, entity: Entity);
    fn alive(&self) -> bool;
    fn target_effect(&self) -> Self::TargetEffectComponent;

    fn remove_stack(
        mut stacks: Query<&mut Self>,
        relations: Res<Relations<EffectTarget>>,
        effect_targets: Query<&EffectTarget>,
        removed: RemovedComponents<Self::EffectComponent>,
    ) {
        for entity in removed.iter() {
            println!("catch removed");
            let effect_target = match effect_targets.get_component::<EffectTarget>(entity) {
                Ok(target) => Some(target.entity()),
                Err(_) => relations.get(&entity).cloned(),
            };

            if let Some(target_entity) = effect_target {
                if let Ok(mut stacks) = stacks.get_component_mut::<Self>(target_entity) {
                    stacks.remove(entity);
                }
            }
        }
    }

    fn apply_stack(
        mut stacks: Query<&mut Self>,
        added: Query<(&Self::EffectComponent, &EffectTarget, Entity), Added<Self::EffectComponent>>,
    ) {
        for (component, target, entity) in added.iter() {
            println!("apply");
            if let Ok(mut stacks) = stacks.get_component_mut::<Self>(target.entity()) {
                stacks.apply(component, entity);
            }
        }
    }

    fn modified_stacks(mut commands: Commands, stacks: Query<(&Self, Entity), Changed<Self>>) {
        for (stack, entity) in stacks.iter() {
            println!("modified");
            if stack.alive() {
                commands.entity(entity).insert(stack.target_effect());
            } else {
                commands
                    .entity(entity)
                    .remove::<Self::TargetEffectComponent>();
            }
        }
    }
}

// Control how stacking of the same effect works.
#[derive(PartialEq, Clone, Debug)]
pub struct StunStacks(SmolSet<[Entity; 4]>);

impl Default for StunStacks {
    fn default() -> Self {
        Self(SmolSet::new())
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct StunTimer(u32);

/*
fn stun_timer(time: Res<Time>, query: Query<&mut StunTimer>) {
    for timer in query.iter_mut() {
        time.
    }
}
*/

// Effect component.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Stun;

// Target effect component.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Stunned;

impl EffectStack for StunStacks {
    type EffectComponent = Stun;
    type TargetEffectComponent = Stunned;
    fn apply(&mut self, _comp: &Self::EffectComponent, entity: Entity) {
        self.0.insert(entity);
    }
    fn remove(&mut self, entity: Entity) {
        self.0.remove(&entity);
    }
    fn alive(&self) -> bool {
        self.0.len() > 0
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
    fn apply(&mut self, other: &Self::EffectComponent, entity: Entity) {
        if other.0 > self.0 {
            self.0 = other.0;
        }
    }
    fn remove(&mut self, _entity: Entity) {}
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
        let mut app_builder = App::build();
        let mut app = std::mem::take(
            &mut app_builder
                .add_plugins(MinimalPlugins)
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    Relations::<EffectTarget>::cache.system(),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    Relations::<EffectTarget>::cleanup.system(),
                )
                .add_system_to_stage(
                    CoreStage::Update,
                    StunStacks::apply_stack
                        .system()
                        .label("apply_stack")
                        .before("remove_stack"),
                )
                .add_system_to_stage(
                    CoreStage::PostUpdate,
                    StunStacks::modified_stacks.system().label("modified_stack"),
                )
                .add_system_to_stage(
                    CoreStage::PostUpdate,
                    StunStacks::remove_stack
                        .system()
                        .label("removed_stack")
                        .before("modified_stack"),
                )
                .app,
        );

        app.world.insert_resource(Relations::<EffectTarget>::default());

        app.update();

        let ability = app.world.spawn().insert(Name::new("Ability")).id();
        let target = app
            .world
            .spawn()
            .insert(StunStacks::default())
            .insert(Name::new("Target"))
            .id();
        let effect = app
            .world
            .spawn()
            .insert(EffectTarget(target))
            .insert(Stun)
            .insert(Name::new("Stun"))
            .id();

        let effect2 = app
            .world
            .spawn()
            .insert(EffectTarget(target))
            .insert(Stun)
            .insert(Name::new("Stun"))
            .id();

        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().0.len(), 0);
        assert_eq!(app.world.get::<Stunned>(target), None);

        app.update();
        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().0.len(), 2);
        assert_eq!(app.world.get::<Stunned>(target), Some(&Stunned));

        app.update();
        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().0.len(), 2);
        assert_eq!(app.world.get::<Stunned>(target), Some(&Stunned));

        // Remove effect from target.
        app.world.entity_mut(effect).despawn();

        app.update();
        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().0.len(), 1);
        assert!(app.world.get::<Stunned>(target).is_some());
        assert!(app.world.get_entity(effect).is_none());

        // Remove secondary effect from target.
        app.world.entity_mut(effect2).despawn();

        app.update();
        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().0.len(), 0);
        assert!(app.world.get::<Stunned>(target).is_none());
        assert!(app.world.get_entity(effect2).is_none());
    }
}
