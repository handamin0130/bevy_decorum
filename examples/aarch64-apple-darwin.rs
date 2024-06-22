use bevy::prelude::*;
use bevy_decorum::prelude::*;

fn main() {
    let mut app = App::new();

    // set bevy plugin
    app.add_plugins(DefaultPlugins);

    // set bevy_decorum plugin
    app.add_plugins(DecorumPlugin::default());

    app.run();
}
