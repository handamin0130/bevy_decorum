mod systems;

use bevy::prelude::*;
use winit::window::WindowId;

pub use systems::*;

/// Decorum APIs
///
///
#[derive(Resource, Default)]
pub struct Decorum {}

impl Decorum {}

/// Decorum Settings
#[derive(Resource, Default)]
pub struct DecorumSettings {
    pub primary_window_id: Option<WindowId>,
}
