use bevy::{prelude::*, transform::commands, utils::HashSet};

use crate::recipes::RecipeType;
use strum::IntoEnumIterator;

pub struct ResearchPlugin;

#[derive(Resource, Reflect)]
pub struct AvailableRecipes(pub HashSet<RecipeType>);

impl Plugin for ResearchPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AvailableRecipes(
            RecipeType::iter().collect::<HashSet<RecipeType>>(),
        ));
    }
}
