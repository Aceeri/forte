use bevy::{ecs::component::Component, prelude::*};

use crate::effect::*;

use std::marker::PhantomData;
use smolset::SmolSet;

pub trait EffectStack
where
    Self: 'static + Sized + Send + Sync,
{
    type EffectComponent: Component;
    type TargetEffectComponent: Component;

    fn apply(&mut self, comp: &Self::EffectComponent, entity: Entity);
    fn remove(&mut self, comp: &Self::EffectComponent, entity: Entity);
    fn alive(&self) -> bool;
    fn target_effect(&self) -> Self::TargetEffectComponent;

    fn remove_stack(
        mut stacks: Query<&mut Self>,
        removing: Query<
            (&Self::EffectComponent, &EffectTarget, Entity),
            Or<(Added<Despawn>, Added<Remove<Self::EffectComponent>>)>,
        >,
    ) {
        for (component, target, entity) in removing.iter() {
            if let Ok(mut stacks) = stacks.get_component_mut::<Self>(target.entity()) {
                stacks.remove(component, entity);
            }
        }
    }

    fn apply_stack(
        mut stacks: Query<&mut Self>,
        added: Query<(&Self::EffectComponent, &EffectTarget, Entity), Added<Self::EffectComponent>>,
    ) {
        for (component, target, entity) in added.iter() {
            if let Ok(mut stacks) = stacks.get_component_mut::<Self>(target.entity()) {
                stacks.apply(component, entity);
            }
        }
    }

    fn modified_stacks(mut commands: Commands, stacks: Query<(&Self, Entity), Changed<Self>>) {
        for (stack, entity) in stacks.iter() {
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

// Reduce a bunch of stacking effects into a single marker component.
#[derive(PartialEq, Clone, Debug)]
pub struct ReduceStack<Effect, TargetEffect> {
    set: SmolSet<[Entity; 4]>,
    phantom: PhantomData<(Effect, TargetEffect)>
}

impl<E, T> Default for ReduceStack<E, T> {
    fn default() -> Self {
        Self {
            set: SmolSet::new(),
            phantom: PhantomData,
        }
    }
}

impl<E, T> ReduceStack<E, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }
}

// Effect component.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Stun;

// Target effect component.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Stunned;

impl<E, T> EffectStack for ReduceStack<E, T>
where
    E: 'static + Send + Sync + Component,
    T: 'static + Send + Sync + Component + Default,
{
    type EffectComponent = E;
    type TargetEffectComponent = T;
    fn apply(&mut self, _comp: &Self::EffectComponent, entity: Entity) {
        self.set.insert(entity);
    }
    fn remove(&mut self, _comp: &Self::EffectComponent, entity: Entity) {
        self.set.remove(&entity);
    }
    fn alive(&self) -> bool {
        self.len() > 0
    }
    fn target_effect(&self) -> Self::TargetEffectComponent {
        T::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Clone, Debug, Default)]
    pub struct Stun;
    #[derive(PartialEq, Clone, Debug, Default)]
    pub struct Stunned;

    /// Example forwarding implementation of a reducing stack.
    #[derive(Clone, Debug, Default)]
    pub struct StunStacks(ReduceStack<Stun, Stunned>);

    impl EffectStack for StunStacks {
        type EffectComponent = Stun;
        type TargetEffectComponent = Stunned;
        fn apply(&mut self, comp: &Self::EffectComponent, entity: Entity) {
            self.0.apply(comp, entity);
        }
        fn remove(&mut self, comp: &Self::EffectComponent, entity: Entity) {
            self.0.remove(comp, entity);
        }
        fn alive(&self) -> bool {
            self.0.alive()
        }
        fn target_effect(&self) -> Self::TargetEffectComponent {
            self.0.target_effect()
        }
    }

    impl StunStacks {
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    #[test]
    fn stack_stuns() {
        let mut app_builder = App::build();
        let mut app = std::mem::take(
            &mut app_builder
                .add_plugins(MinimalPlugins)
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
                .add_system_to_stage(CoreStage::Last, cleanup_despawning.system())
                .add_system_to_stage(CoreStage::Last, cleanup_removing::<Stun>.system())
                .app,
        );

        app.update();

        let _ability = app.world.spawn().insert(Name::new("Ability")).id();
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
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().len(), 0);
        assert_eq!(app.world.get::<Stunned>(target), None);

        app.update();
        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().len(), 2);
        assert_eq!(app.world.get::<Stunned>(target), Some(&Stunned));

        app.update();
        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().len(), 2);
        assert_eq!(app.world.get::<Stunned>(target), Some(&Stunned));

        // Remove stun from effect.
        app.world
            .entity_mut(effect)
            .insert(Remove::<Stun>::default());

        app.update();
        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().len(), 1);
        assert!(app.world.get::<Stunned>(target).is_some());
        assert!(app.world.get_entity(effect).is_some()); // this effect is still alive, just removed the stun component

        // Remove stun from secondary effect.
        app.world.entity_mut(effect2).insert(Despawn);

        app.update();
        dbg!(app.world.get::<StunStacks>(target));
        assert_eq!(app.world.get::<StunStacks>(target).unwrap().len(), 0);
        assert!(app.world.get::<Stunned>(target).is_none());
        assert!(app.world.get_entity(effect2).is_none()); // this effect was completely killed
    }
}
