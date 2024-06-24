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
bevy_decorum = "0.1.0-dev"
```

### System setup

Add the `DecorumPlugin` to your app:

```rust
use bevu::prelude::*;
use bevy_decorum::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
       .add_plugins(DecorumPlugin::default());

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
