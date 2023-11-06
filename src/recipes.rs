use crate::items::{ItemId, ItemList};
use bevy::prelude::*;
use core::fmt;

pub struct RecipeId(String);

pub struct Recipe {
    pub name: String,
    pub input: Vec<(ItemId, u32)>,  // Input cost
    pub output: Vec<(ItemId, u32)>, // Production
    pub cost: f32,                  // Time to craft
}

pub enum RecipeList {
    WoodToToy,
}

impl RecipeList {
    pub fn get_recipe(&self) -> Recipe {
        use ItemList::*;
        match self {
            // TODO: We should generate this from a recipe file.
            RecipeList::WoodToToy => Recipe {
                name: "wood-to-toy".to_string(),
                input: vec![(ItemId::new(Wood), 1)],
                output: vec![(ItemId::new(Toy), 1)],
                cost: 5.0,
            },
        }
    }
}

impl fmt::Display for RecipeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecipeList::WoodToToy => write!(f, "core::wood-to-toy"),
        }
    }
}
