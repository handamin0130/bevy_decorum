use super::DecorumSettings;
use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};

#[allow(dead_code)]
pub fn off_decorations(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.decorations = false;
}

pub fn get_primary_window_id(
    mut primary_window: Query<Entity, With<PrimaryWindow>>,
    mut decorum_settings: ResMut<DecorumSettings>,
    winit_windows: NonSend<WinitWindows>,
) {
    let primary_window_entity = primary_window.single_mut();

    let primary_window_id = winit_windows
        .entity_to_winit
        .get(&primary_window_entity)
        .cloned();

    decorum_settings.primary_window_id = primary_window_id;
}
