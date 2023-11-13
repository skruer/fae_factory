use bevy::{prelude::*, window::PrimaryWindow};

use super::MyWorldCoords;

pub struct FaeCameraPlugin;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

impl Plugin for FaeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyWorldCoords(Vec2::new(0.0, 0.0)))
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, my_cursor_system);
    }
}

fn setup(mut commands: Commands) {
    let camera = Camera2dBundle::default();

    commands.spawn((camera, MainCamera {}));
}

pub(crate) fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}
