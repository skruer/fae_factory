use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crafting::CraftingPlugin;
use input::FaeInputPlugin;
use items::ItemPlugin;
use player::PlayerPlugin;
use research::ResearchPlugin;
use structures::StructurePlugin;

mod common;
mod crafting;
mod input;
mod items;
mod player;
mod recipes;
mod research;
mod structures;

#[derive(Component)]
pub struct Speed(pub f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fae Factory".into(),
                        resolution: (1000.0, 750.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins((
            PlayerPlugin,
            CraftingPlugin,
            ItemPlugin,
            StructurePlugin,
            FaeInputPlugin,
            ResearchPlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .run();
}
