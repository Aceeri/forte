
use bevy::prelude::*;

// Effect's target (player, mob, etc.)
pub struct EffectTarget(Entity);

// Control how stacking of the same effect works.
pub struct StunStacks(u16);

// Effect component.
pub struct Stun;

// Target effect component.
pub struct Stunned;

fn remove_stun_stack(
    mut commands: Commands,
    mut stacks: Query<&mut StunStacks>,
    mut target: Query<&EffectTarget>,
    mut removed_stuns: RemovedComponents<Stun>,
) {
    for entity in removed_stuns.iter() {
        if let Ok(effect_target) = target.get_component::<EffectTarget>(entity) {
            if let Ok(stacks) = stacks.get_component_mut::<StunStacks>(effect_target.0) {
                stacks.0.saturating_sub(1);
            }
        }
    }
}

fn apply_stun_stack(
    mut commands: Commands,
    mut stacks: Query<&mut StunStacks>,
    mut added_stuns: Query<(&EffectTarget, &mut StunStacks), Added<Stun>>,
) {
    for (target, stun) in added_stuns.iter_mut() {
        commands.entity(target.0);
    }
}

fn modified_stun_stacks(
    mut commands: Commands,
    mut stacks: Query<(&StunStacks, Entity), Changed<StunStacks>>,
) {
    for (stun_stack, entity) in stacks.iter() {
        if stun_stack.0 == 0 {
            commands.entity(entity).remove::<Stunned>();
        } else {
            commands.entity(entity).insert(Stunned);
        }
    }
}
