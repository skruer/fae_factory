use bevy::prelude::*;

use self::{camera::FaeCameraPlugin, keyboard::FaeKeyboardPlugin, mouse::FaeMousePlugin};

pub mod camera;
pub mod keyboard;
pub mod mouse;

#[derive(Resource, Default)]
pub struct MyWorldCoords(pub Vec2);
pub struct FaeInputPlugin;

impl Plugin for FaeInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FaeCameraPlugin, FaeMousePlugin, FaeKeyboardPlugin));
    }
}
