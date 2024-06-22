use bevy::prelude::*;
use bevy_decorum::prelude::*;

fn main() {
    let mut app = App::new();

    // set bevy plugin
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "example: arrch64-apple-darwin".to_string(),
            decorations: false,
            ..Default::default()
        }),
        ..Default::default()
    }));

    // set bevy_decorum plugin
    app.add_plugins(DecorumPlugin::default());

    app.run();
}
