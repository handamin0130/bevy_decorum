# bevy_decorum

<div align="center">
  <a href="https://crates.io/crates/bevy_decorum"><img src="https://img.shields.io/crates/v/bevy_decorum?label=version&color=d69039"></a>
  <a href="./LICENSE-MIT/Apache"><img src="https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg?label=license&color=9fcec4"></a>
</div>

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

bevy_decorum is dual-licensed under either [APACHE](./LICENSE-APACHE2) and [MIT](./LICENSE-MIT) licenses.
