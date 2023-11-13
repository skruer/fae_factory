use bevy::prelude::*;

use super::FaeInputModifier;
use crate::{
    common::{Clickable, Held, Holdable},
    map::grid::{GridPosition, HoveredGrid},
};

pub struct FaeMousePlugin;

#[derive(Event)]
pub struct FaeEntityClickEvent {
    pub entities: Vec<Entity>,
    pub modifiers: FaeInputModifier,
}

#[derive(Event)]
pub struct FaeEntityContextClickEvent {
    pub entities: Vec<Entity>,
    pub modifiers: FaeInputModifier,
}

#[derive(Component, Default, Reflect)]
pub struct Previewed(pub Option<Holdable>);

impl Plugin for FaeMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (handle_click, preview_held))
            .add_event::<FaeEntityClickEvent>()
            .add_event::<FaeEntityContextClickEvent>()
            .register_type::<Previewed>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Previewed(None), Name::from("None Held")));
}

fn get_clicked_entities(
    mouse_grid: Res<HoveredGrid>,
    sprite_query: Query<(&GridPosition, &Transform, Entity), With<Clickable>>,
) -> Vec<Entity> {
    sprite_query
        .iter()
        .filter(|(grid, _, _)| **grid == mouse_grid.0)
        .map(|(_, _, entity)| entity)
        .collect()
}

fn handle_click(
    mouse_grid: Res<HoveredGrid>,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    sprite_query: Query<(&GridPosition, &Transform, Entity), With<Clickable>>,
    mut left_click_writer: EventWriter<FaeEntityClickEvent>,
    mut right_click_writer: EventWriter<FaeEntityContextClickEvent>,
) {
    if mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        let clicked = get_clicked_entities(mouse_grid, sprite_query);
        println!("Clicked: {:?}", clicked);

        if mouse.just_pressed(MouseButton::Left) {
            left_click_writer.send(FaeEntityClickEvent {
                entities: clicked.clone(),
                modifiers: FaeInputModifier::from(&keys),
            });
        }
        if mouse.just_pressed(MouseButton::Right) {
            println!("Right click");
            right_click_writer.send(FaeEntityContextClickEvent {
                entities: clicked.clone(),
                modifiers: FaeInputModifier::from(&keys),
            });
        }
    }
}

pub(crate) fn preview_held(
    mut commands: Commands,
    mouse_grid: Res<HoveredGrid>,
    query: Query<&Held>,
    mut previewed: Query<(Entity, &mut Previewed)>,
    asset_server: Res<AssetServer>,
) {
    let (previewed_entity, previewed) = match previewed.get_single_mut() {
        Err(_) => {
            return;
        }
        Ok((previewed_entity, previewed)) => (previewed_entity, previewed),
    };
    if let Ok(held) = query.get_single() {
        let texture = match held.0 {
            Some(Holdable::Structure(structure)) => {
                if let Some(Holdable::Structure(shown_type)) = previewed.0 {
                    if shown_type == structure && !mouse_grid.is_changed() {
                        return;
                    }
                }
                Some(structure.asset_file())
            }
            _ => None,
        };

        commands.entity(previewed_entity).despawn_recursive();
        println!("Previewing: {:?}, {:?}", held.0, mouse_grid.is_changed());

        let held = held.0;
        if let Some(texture) = texture {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                        ..default()
                    },
                    texture: asset_server.load(texture),
                    transform: Transform {
                        translation: mouse_grid.0.sprite_translation_z(1.0),
                        ..default()
                    },
                    ..default()
                },
                Previewed(held),
            ));
        } else {
            commands.spawn((Previewed(None), Name::from("None Held")));
        }
    }
}
