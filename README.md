# 🚧 bevy_decorum

[![License: MIT/Apache](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](LICENSE)

A highly customizable window decoration plugin for the [Bevy engine](https://bevyengine.org),
inspired by [tauri-plugin-decorum](https://github.com/clearlysid/tauri-plugin-decorum).

## Features

- [] ...

## Usage

### Dependency

Add to `Cargo.toml`:

```toml
[dependecies]
bevy_decorum = { version = "0.1.0-dev", git = "https://github.com/handamin0130/bevy_decorum" }
```

### System setup

Add the `DecorumPlugin` to your app:

```rust
use bevu::prelude::*;
use bevy_decorum::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            transparent: true,
            #[cfg(target_os = "macos")]
            composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_plugins(DecorumPlugin::default());

    // your code

    app.run();
}
```

## Support table

### Bevy support

| bevy | bevy\_decorum | 
| ---: | --- |
| 0.13 | 🛠️ in progress |

### OS support

| os | bevy\_decorum |
| --- | ---: |
| windows | planned |
| macos | * |
