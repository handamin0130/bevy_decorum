use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct DecorumPlugin {}

impl DecorumPlugin {
    pub fn new() -> Self {
        DecorumPlugin::default()
    }
}

impl Plugin for DecorumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::decorum::Decorum>()
            .init_resource::<crate::decorum::DecorumSettings>();

        app.add_systems(Startup, crate::decorum::get_primary_window_id);

        #[cfg(target_os = "macos")]
        app.add_systems(
            PostStartup,
            crate::decorum_traffic::setup_traffic_light_positioner,
        );
    }
}
