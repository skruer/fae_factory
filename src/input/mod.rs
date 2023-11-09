use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use self::{camera::FaeCameraPlugin, keyboard::FaeKeyboardPlugin, mouse::FaeMousePlugin};

pub mod camera;
pub mod keyboard;
pub mod mouse;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, EnumIter)]
pub enum FaeEntityInputModifier {
    Shift,
    Ctrl,
    Alt,
}

impl FaeEntityInputModifier {
    fn check_pressed(&self, keys: &Res<Input<KeyCode>>) -> bool {
        use FaeEntityInputModifier::*;
        match self {
            Shift => keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight),
            Ctrl => keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight),
            Alt => keys.pressed(KeyCode::AltLeft) || keys.pressed(KeyCode::AltRight),
        }
    }
}

#[derive(Component, Reflect, Debug, Eq, PartialEq, Hash, Clone)]
pub struct FaeInputModifier(pub Vec<FaeEntityInputModifier>);

impl FaeInputModifier {
    fn new(modifiers: Vec<FaeEntityInputModifier>) -> Self {
        FaeInputModifier(modifiers)
    }

    pub fn check_all_pressed(&self, modifiers: &Vec<FaeEntityInputModifier>) -> bool {
        modifiers.iter().all(|modifier| self.0.contains(&modifier))
    }

    pub fn check_only_pressed(&self, modifiers: &Vec<FaeEntityInputModifier>) -> bool {
        FaeEntityInputModifier::iter().all(|modifier| match modifiers.contains(&modifier) {
            true => self.0.contains(&modifier),
            false => !self.0.contains(&modifier),
        })
    }
}

fn gather_modifiers(keys: &Res<Input<KeyCode>>) -> Vec<FaeEntityInputModifier> {
    use FaeEntityInputModifier::*;
    vec![Shift, Ctrl, Alt]
        .into_iter()
        .filter(|modifier| modifier.check_pressed(keys))
        .collect()
}

impl From<Res<'_, Input<KeyCode>>> for FaeInputModifier {
    fn from(keys: Res<Input<KeyCode>>) -> Self {
        FaeInputModifier::new(gather_modifiers(&keys))
    }
}
#[derive(Resource, Default)]
pub struct MyWorldCoords(pub Vec2);
pub struct FaeInputPlugin;

impl Plugin for FaeInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FaeCameraPlugin, FaeMousePlugin, FaeKeyboardPlugin));
    }
}
