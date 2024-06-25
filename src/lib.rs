//! # Bevy Decorum
//!
//! A highly customizable window decoration plugin for the Bevy engine,inspired by tauri-plugin-decorum
//!
//! ## Example
//!
//! Add bevy_decorum to your bevy app:
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_decorum::prelude::*;
//!
//! fn main() {
//!     let mut app = App::new();
//!
//!     app.add_plugins(DefaultPlugins)
//!         .add_plugins(DecorumPlugin::default());
//!
//!     app.run();
//! }
//!
//! ```

pub mod decorum;
#[cfg(target_os = "macos")]
mod decorum_traffic;
pub mod plugin;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod prelude {
    pub use crate::{
        decorum::{Decorum, DecorumSettings},
        plugin::DecorumPlugin,
    };
}
