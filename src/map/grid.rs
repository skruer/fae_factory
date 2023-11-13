use bevy::prelude::*;

use crate::input::camera::my_cursor_system;
use crate::input::MyWorldCoords;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridTile>()
            .register_type::<TileType>()
            .register_type::<GridPosition>()
            .add_systems(
                PreUpdate,
                // Ordered to ensure that we're using this frame's mouse position
                update_mouse_grid_location.after(my_cursor_system),
            )
            .insert_resource(HoveredGrid::new());
    }
}

pub struct Chunk(pub IVec2);

impl Chunk {
    pub const CHUNK_SIZE: usize = 16;
}

#[derive(Component, Reflect, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct GridPosition(pub IVec2);

impl GridPosition {
    pub const PIXELS_PER_TILE: i32 = 32;

    pub fn from_position(position: Vec2) -> Self {
        let position = (position / Self::PIXELS_PER_TILE as f32) + Vec2::new(0.5, 0.5);
        GridPosition(IVec2::new(position.x as i32, position.y as i32))
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self::from_position(translation.truncate())
    }

    pub fn to_chunk(&self) -> Chunk {
        Chunk(IVec2::new(
            self.0.x / Chunk::CHUNK_SIZE as i32,
            self.0.y / Chunk::CHUNK_SIZE as i32,
        ))
    }

    pub fn sprite_translation(&self) -> Vec3 {
        Vec3::new(
            (self.0.x * Self::PIXELS_PER_TILE) as f32,
            (self.0.y * Self::PIXELS_PER_TILE) as f32,
            0.0,
        )
    }

    pub fn sprite_translation_z(&self, z: f32) -> Vec3 {
        Vec3::new(
            (self.0.x * Self::PIXELS_PER_TILE) as f32,
            (self.0.y * Self::PIXELS_PER_TILE) as f32,
            z,
        )
    }

    pub fn value(&self) -> IVec2 {
        self.0
    }
}

#[derive(Resource)]
pub struct HoveredGrid(pub GridPosition);

impl HoveredGrid {
    pub fn new() -> Self {
        HoveredGrid(GridPosition(IVec2::new(0, 0)))
    }

    pub fn value(&self) -> IVec2 {
        self.0.value()
    }
}

#[derive(Component, Reflect, Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TileType {
    Grass,
    Water,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct GridTile {
    pub display_priority: i32,
    pub tile_type: TileType,
}

fn update_mouse_grid_location(mycoords: Res<MyWorldCoords>, mut hovered_grid: ResMut<HoveredGrid>) {
    let new_grid = GridPosition::from_position(mycoords.0);
    if new_grid != hovered_grid.0 {
        hovered_grid.0 = new_grid;
    }
}
