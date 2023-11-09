use crate::{items::ItemType, research::AvailableRecipes};
use bevy::prelude::*;
use core::fmt;

use strum_macros::EnumIter;

#[derive(Reflect, Clone, Debug)]
pub struct Recipe {
    pub recipe_type: RecipeType,
    pub input: Vec<(ItemType, u32)>,  // Input cost
    pub output: Vec<(ItemType, u32)>, // Production
    pub cost: f32,                    // Time to craft
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, EnumIter, Resource)]
pub enum RecipeType {
    WoodToToy,
    CrystalToToy,
}

impl Iterator for RecipeType {
    type Item = RecipeType;

    fn next(&mut self) -> Option<Self::Item> {
        use RecipeType::*;
        match self {
            WoodToToy => Some(CrystalToToy),
            CrystalToToy => Some(WoodToToy),
        }
    }
}

impl RecipeType {
    pub fn next_available_recipe(self, available_recipes: &AvailableRecipes) -> RecipeType {
        match self
            .into_iter()
            .filter(|r| available_recipes.0.contains(r))
            .next()
        {
            Some(recipe) => recipe,
            None => {
                println!("");
                RecipeType::WoodToToy
            }
        }
    }
}

impl From<RecipeType> for Recipe {
    fn from(recipe_type: RecipeType) -> Self {
        use ItemType::*;
        use RecipeType::*;
        match recipe_type {
            WoodToToy => Recipe {
                recipe_type,
                input: vec![(Wood, 3)],
                output: vec![(Toy, 1)],
                cost: 5.0,
            },
            CrystalToToy => Recipe {
                recipe_type,
                input: vec![(Crystal, 1)],
                output: vec![(Toy, 1)],
                cost: 10.0,
            },
        }
    }
}

impl fmt::Display for RecipeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecipeType::WoodToToy => write!(f, "core::wood-to-toy"),
            RecipeType::CrystalToToy => write!(f, "core::crystal-to-toy"),
        }
    }
}
