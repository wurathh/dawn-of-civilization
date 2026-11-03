pub mod components;
pub mod textures;
pub mod theme;

pub use components::*;
pub use textures::*;
pub use theme::*;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Theme>()
            .init_resource::<UiTextures>()
            .add_observer(components::buttons::handle_button_pointer_over)
            .add_observer(components::buttons::handle_button_pointer_out)
            .add_observer(components::buttons::handle_button_pointer_press)
            .add_observer(components::buttons::handle_button_pointer_release)
            .add_systems(Update, components::buttons::update_button_visuals);
    }
}
