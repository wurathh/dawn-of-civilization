pub mod camera;
pub mod hud;
pub mod interaction;
pub mod labels;
pub mod rendering;
pub mod setup;
pub mod types;

pub use camera::*;
pub use hud::*;
pub use interaction::*;
pub use labels::*;
pub use rendering::*;
pub use setup::*;
pub use types::*;

use crate::scenario::ActiveScenario;
use crate::state::AppState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let scenario = ActiveScenario::load_from_file("assets/scenarios/modernworld/scenario.json");
        app.insert_resource(scenario)
            .insert_resource(CameraRig {
                target_pos: Vec2::new(850.0, 560.0),
                target_scale: 0.85,
            })
            .init_resource::<MapDragState>()
            .init_resource::<ProvinceHoverTooltipTimer>()
            .add_systems(Startup, setup_world_map)
            .add_systems(OnEnter(AppState::InGame), setup_ingame_hud)
            .add_systems(OnExit(AppState::InGame), cleanup_ingame_hud)
            .add_systems(
                Update,
                handle_ingame_input.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    camera_controller.run_if(is_map_active),
                    update_visual_lod,
                    province_interaction.run_if(is_map_active),
                    update_province_overlays,
                    update_province_hover_tooltip.run_if(is_map_active),
                    update_country_labels_visibility,
                )
                    .chain(),
            );
    }
}
