mod systems;

use bevy::prelude::*;

pub use systems::*;

/// Decorum APIs
#[derive(Resource, Default)]
pub struct Decorum {}

impl Decorum {}

/// Decorum Settings
#[derive(Resource, Default)]
pub struct DecorumSettings {
    pub windows: Vec<Entity>,
}
