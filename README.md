# bevy_decorum

[![License: MIT/Apache](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](LICENSE)

#

A highly customizable window decoration plugin for the [Bevy engine](https://bevyengine.org),
inspired by [tauri-plugin-decorum](https://github.com/clearlysid/tauri-plugin-decorum).

> [!WARNING]  
> This library is EXPERIMENTAL.

## Usage

### Dependency

Add to `Cargo.toml`:

```toml
[dependecies]
bevy_decorum = { version = "0.0.1" }
```

### System setup

Add the `DecorumPlugin` to your app:

```rust
use bevu::prelude::*;
use bevy_decorum::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins(DecorumPlugin::default());

    // your code

    app.run();
}
```

## Versions

| bevy | bevy\_decorum |
| ---: | --- |
| 0.13.2 | 0.0.1 |

## Licensing

bevy_decorum is dual-licensed under either [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses.
