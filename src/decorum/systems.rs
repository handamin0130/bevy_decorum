use super::DecorumSettings;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowLevel},
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

pub fn get_primary_window(
    mut primary_window_query: Query<Entity, With<PrimaryWindow>>,
    mut decorum_settings: ResMut<DecorumSettings>,
) {
    let primary_window_entity = primary_window_query.single_mut();

    decorum_settings.windows.push(primary_window_entity);
}
