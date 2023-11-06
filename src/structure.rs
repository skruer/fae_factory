use core::fmt;

use bevy::prelude::*;

struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Reflect)]
struct Structure {
    id: StructureId,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Reflect)]
struct StructureId {
    id: String,
}

#[derive(Component, Debug, Reflect)]
enum StructureList {
    Assembler,
    Conveyor,
    Miner,
    Splitter,
    Storage,
}

impl fmt::Display for StructureList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StructureList::*;
        match self {
            Assembler => write!(f, "assembler"),
            Conveyor => write!(f, "conveyor"),
            Miner => write!(f, "miner"),
            Splitter => write!(f, "splitter"),
            Storage => write!(f, "storage"),
        }
    }
}
