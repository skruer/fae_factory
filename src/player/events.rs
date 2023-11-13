use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct PlayerMoveEvent(pub Vec2);
