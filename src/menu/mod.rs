pub mod civ_info;
pub mod common;
pub mod main_menu;
pub mod scenario_catalog;
pub mod scenario_select;
pub mod settings_modal;

pub use civ_info::*;
pub use common::*;
pub use main_menu::*;
pub use scenario_catalog::*;
pub use scenario_select::*;
pub use settings_modal::*;

use crate::state::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveMenuBackground>()
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
            .add_systems(OnEnter(AppState::ScenarioCatalog), setup_scenario_catalog)
            .add_systems(OnExit(AppState::ScenarioCatalog), cleanup_scenario_catalog)
            .add_systems(OnEnter(AppState::ScenarioSelect), setup_scenario_select)
            .add_systems(OnExit(AppState::ScenarioSelect), cleanup_scenario_select)
            .add_systems(OnEnter(AppState::Settings), setup_settings_page)
            .add_systems(OnExit(AppState::Settings), cleanup_settings_page)
            .add_systems(
                Update,
                update_main_menu_animation.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                update_scenario_catalog_animation.run_if(in_state(AppState::ScenarioCatalog)),
            )
            .add_systems(
                Update,
                (
                    update_settings_animation,
                    sync_settings_page_ui,
                    handle_settings_input,
                )
                    .run_if(in_state(AppState::Settings)),
            )
            .add_systems(
                Update,
                (
                    update_scenario_select_animation,
                    handle_scenario_select_input,
                    sync_selected_civ_ui,
                )
                    .run_if(in_state(AppState::ScenarioSelect)),
            );
    }
}
