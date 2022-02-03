use super::*;

// EventDespawn<HitWall>
#[derive(Component, Debug, Copy, Clone)]
pub struct EventDespawn<Event>(PhantomData<Event>)
where
    Event: 'static + Send + Sync + ReactingEntity;

impl<Event> Default for EventDespawn<Event>
where
    Event: 'static + Send + Sync + ReactingEntity,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Event> EventDespawn<Event>
where
    Event: 'static + Send + Sync + ReactingEntity,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(mut commands: Commands, mut events: EventReader<Event>) {
        for event in events.iter() {
            let entity = event.reacting_entity();
            commands.entity(entity).despawn_recursive();
        }
    }
}
