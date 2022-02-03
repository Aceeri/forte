use bevy::prelude::*;
use bevy_rapier3d::{physics::IntoEntity, prelude::IntersectionEvent};

use std::marker::PhantomData;

pub mod damage;
pub mod despawn;
pub mod wall;

pub trait React {
    type Marker;
    type Event;
    fn react(&self, commands: &mut Commands, event: &Self::Event, m: &Self::Marker);
}

// Basic intersection detection, takes a marker component and an event.
//
// If the entity intersected something with the specified marker component an event will be sent.
pub fn intersection_event<Marker, Event>(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut events: EventWriter<Event>,
    markers: Query<&Marker>,
) where
    Marker: Component,
    Event: 'static + Send + Sync + From<IntersectionEvent>,
{
    for intersection_event in intersection_events.iter() {
        let _reacting_entity = intersection_event.collider1.entity();
        let absorbing_entity = intersection_event.collider2.entity();
        if markers.get_component::<Marker>(absorbing_entity).is_ok() {
            events.send(Event::from(*intersection_event));
        }
    }
}

pub trait ReactingEntity {
    fn reacting_entity(&self) -> Entity;
}

pub trait AbsorbingEntity {
    fn absorbing_entity(&self) -> Entity;
}
