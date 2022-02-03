use bevy::app::Events;

use crate::ability::attribute::attribute::Attribute;

use super::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct Damageable<Stat>(PhantomData<Stat>);

#[derive(Debug, Copy, Clone)]
pub struct HitDamageable<Stat> {
    intersection: IntersectionEvent,
    phantom: PhantomData<Stat>,
}

impl<Stat> From<IntersectionEvent> for HitDamageable<Stat> {
    fn from(event: IntersectionEvent) -> Self {
        Self {
            intersection: event,
            phantom: PhantomData,
        }
    }
}

impl<Stat> ReactingEntity for HitDamageable<Stat> {
    fn reacting_entity(&self) -> Entity {
        self.intersection.collider1.entity()
    }
}

impl<Stat> AbsorbingEntity for HitDamageable<Stat> {
    fn absorbing_entity(&self) -> Entity {
        self.intersection.collider2.entity()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Health;

struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<HitDamageable<Health>>>()
            .add_system(EventDamage::<Health>::check);
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub struct EventDamage<Attribute, Event = HitDamageable<Attribute>>(
    PhantomData<(Attribute, Event)>,
)
where
    Attribute: 'static + Send + Sync,
    Event: 'static + Send + Sync + ReactingEntity + AbsorbingEntity;

impl<Attribute, Event> Default for EventDamage<Attribute, Event>
where
    Attribute: 'static + Send + Sync,
    Event: 'static + Send + Sync + ReactingEntity + AbsorbingEntity,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<A, Event> EventDamage<A, Event>
where
    A: 'static + Send + Sync + std::fmt::Debug,
    Event: 'static + Send + Sync + ReactingEntity + AbsorbingEntity,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(
        commands: Commands,
        mut events: EventReader<Event>,
        mut stat: Query<&mut Attribute<A>>,
    ) {
        for event in events.iter() {
            if let Ok(stat) = stat.get_component_mut::<Attribute<A>>(event.absorbing_entity()) {
                println!(
                    "we should damage {:?}, current attr info: {:?}",
                    event.absorbing_entity(),
                    stat
                );
            }
        }
    }
}
