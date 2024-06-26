use bevy::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct WindowDecorum {
    pub visible_title: bool,
    pub visible_traffic: bool,
    pub decoration: bool,
}

impl Default for WindowDecorum {
    fn default() -> Self {
        WindowDecorum {
            visible_title: true,
            visible_traffic: true,
            decoration: true,
        }
    }
}
