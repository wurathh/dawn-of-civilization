pub mod components;
pub mod theme;

pub use components::*;
pub use theme::*;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Theme>()
            .init_resource::<Ui>()
            .init_resource::<Skins>()
            .add_observer(components::buttons::handle_button_pointer_over)
            .add_observer(components::buttons::handle_button_pointer_out)
            .add_observer(components::buttons::handle_button_pointer_press)
            .add_observer(components::buttons::handle_button_pointer_release)
            .add_observer(components::hud::handle_hud_item_pointer_over)
            .add_observer(components::hud::handle_hud_item_pointer_out)
            .add_observer(components::hud::handle_hud_item_pointer_press)
            .add_observer(components::hud::handle_hud_item_pointer_release)
            .add_systems(
                Update,
                (
                    components::buttons::attach_deferred_button_skins,
                    components::buttons::button_visuals,
                    components::buttons::update_button_visuals,
                    components::hud::hud_item_visuals,
                    components::hud::sync_deferred_inset_frames,
                    components::form::sync_checkbox_and_radio_icons,
                    components::lists::sync_deferred_tab_images,
                    components::feedback::sync_deferred_tooltip_frames,
                    components::feedback::tooltip_system,
                ),
            );
    }
}
