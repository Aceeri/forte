use bevy::app::Events;
use bevy::prelude::*;

use super::despawn::EventDespawn;
use crate::ability::react::{AbsorbingEntity, ReactingEntity};
use bevy_rapier3d::{physics::IntoEntity, prelude::IntersectionEvent};

#[derive(Component, Debug, Copy, Clone)]
pub struct Wall;

#[derive(Debug, Copy, Clone)]
pub struct HitWall {
    intersection: IntersectionEvent,
}

impl From<IntersectionEvent> for HitWall {
    fn from(event: IntersectionEvent) -> Self {
        Self {
            intersection: event,
        }
    }
}

impl ReactingEntity for HitWall {
    fn reacting_entity(&self) -> Entity {
        self.intersection.collider1.entity()
    }
}

impl AbsorbingEntity for HitWall {
    fn absorbing_entity(&self) -> Entity {
        self.intersection.collider2.entity()
    }
}

struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<HitWall>>()
            .add_system(EventDespawn::<HitWall>::check);
    }
}
