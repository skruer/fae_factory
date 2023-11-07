use assembler::AssemblerPlugin;
use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use input::{FaeInputPlugin, MainCamera};
use items::ItemPlugin;
use player::PlayerPlugin;
use structure::StructurePlugin;

mod assembler;
mod common;
mod input;
mod items;
mod player;
mod recipes;
mod structure;

#[derive(Component)]
pub struct Speed(pub f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Farming Roguelike".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_plugins((
            PlayerPlugin,
            AssemblerPlugin,
            ItemPlugin,
            StructurePlugin,
            FaeInputPlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    commands.spawn((camera, MainCamera {}));
}
