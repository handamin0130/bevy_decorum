use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct DecorumPlugin {}

impl DecorumPlugin {
    pub fn new() -> Self {
        DecorumPlugin::default()
    }
}

impl Plugin for DecorumPlugin {
    fn build(&self, _app: &mut App) {}
}
