use bevy::prelude::*;

use crate::player::{Player, PlayerMove};

pub(super) struct FaeKeyboardPlugin;

impl Plugin for FaeKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_input);
    }
}

pub(super) fn handle_keyboard_input(
    key_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut PlayerMove), With<Player>>,
) {
    let keys = vec![
        (KeyCode::W, Vec2::new(0.0, 1.0)),
        (KeyCode::S, Vec2::new(0.0, -1.0)),
        (KeyCode::D, Vec2::new(1.0, 0.0)),
        (KeyCode::A, Vec2::new(-1.0, 0.0)),
    ];

    let direction = keys
        .iter()
        .map(|(key, direction)| match key_input.pressed(*key) {
            true => *direction,
            _ => Vec2::new(0.0, 0.0),
        })
        .fold(Vec2::new(0.0, 0.0), |acc, direction| acc + direction)
        .try_normalize();
    let mut player = player.single_mut();
    *player = PlayerMove(direction);
}
