//! # Bevy Decorum
//!

mod plugin;

// -- export
pub use plugin::DecorumPlugin;

pub mod prelude {
    pub use crate::DecorumPlugin;
}
