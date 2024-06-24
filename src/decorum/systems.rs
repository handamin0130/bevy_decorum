use super::DecorumSettings;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowLevel},
    winit::WinitWindows,
};

#[allow(dead_code)]
pub fn off_decorations(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = primary_window.single_mut();

    primary_window.decorations = false;
}

pub fn set_window_level(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = primary_window.single_mut();

    primary_window.window_level = WindowLevel::AlwaysOnTop;
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
