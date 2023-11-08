use bevy::prelude::*;

use crate::{
    common::{BoundingBox, Clickable},
    items::Inventory,
};

use super::{Structure, StructureId, StructureList, STRUCTURE_Z};

pub(super) fn spawn_assembler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
    transform: &Transform,
    position: &Vec2,
) {
    let texture = asset_server.load("assembler.png");
    let image_dimensions = assets.get(&texture).unwrap().size();
    let scaled_image_dimension = image_dimensions * transform.scale.truncate();
    let bounding_box =
        Rect::from_center_size(transform.translation.truncate(), scaled_image_dimension);
    commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, STRUCTURE_Z),
                    ..default()
                },
                ..default()
            },
            Name::new("Assembler"),
            Structure {
                structure_id: StructureId::new(StructureList::Assembler),
            },
            Inventory::new(2, vec![]),
            BoundingBox(bounding_box),
            Clickable {},
        ))
        .with_children(|child_builder| {});
}
