use bevy::prelude::*;

pub mod grid;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(grid::GridPlugin);
    }
}
