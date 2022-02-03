use bevy::prelude::*;
use std::marker::PhantomData;

use super::attribute::{Amount, Attribute, Max};

pub struct Regen<A>(PhantomData<A>);

pub fn regen<A>(
    mut query: Query<(
        &mut Attribute<A>,
        &Attribute<Regen<A>>,
        Option<&Attribute<Max<A>>>,
    )>,
) where
    A: 'static + Send + Sync,
{
    for (mut attribute, regen, max) in query.iter_mut() {
        let mut result = attribute.amount() + regen.amount();
        if let Some(max) = max {
            if result > *max.amount() {
                result = *max.amount();
            }
        }

        if *attribute.amount() != result {
            attribute.set_amount(result);
        }
    }
}

pub fn regen_unless_zero<A>(
    mut query: Query<(
        &mut Attribute<A>,
        &Attribute<Regen<A>>,
        Option<&Attribute<Max<A>>>,
    )>,
) where
    A: 'static + Send + Sync,
{
    for (mut attribute, regen, max) in query.iter_mut() {
        if *attribute.amount() <= Amount::ZERO {
            continue;
        }

        let mut result = attribute.amount() + regen.amount();
        if let Some(max) = max {
            if result > *max.amount() {
                result = *max.amount();
            }
        }

        if *attribute.amount() != result {
            attribute.set_amount(result);
        }
    }
}
