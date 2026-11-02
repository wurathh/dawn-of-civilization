use bevy::prelude::*;
use bevy::text::FontSource;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    ScenarioCatalog,
    ScenarioSelect,
    InGame,
}

#[derive(Resource, Debug, Clone)]
pub struct GameSettings {
    pub fog_of_war: bool,
    pub difficulty: String,
    pub ai_aggressiveness: String,
    pub ironman: bool,
    pub show_modal: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fog_of_war: false,
            difficulty: "Normal".to_string(),
            ai_aggressiveness: "Standart".to_string(),
            ironman: false,
            show_modal: false,
        }
    }
}

#[derive(Resource, Clone)]
pub struct UiFonts {
    /// Başlıklar için klasik-modern imparatorluk fontu (EB Garamond ExtraBold)
    pub display: FontSource,
    /// Butonlar ve alt başlıklar için kalın modern font (Inter Bold)
    pub heading: FontSource,
    /// Genel arayüz metinleri için (Inter SemiBold)
    pub body: FontSource,
    /// İkincil detay metinleri için (Inter Medium)
    pub medium: FontSource,
    /// Harita üzerindeki ülke ve imparatorluk isimleri için klasik monumental font (Cinzel Bold)
    pub map_country: FontSource,
}

impl FromWorld for UiFonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            display: FontSource::Handle(asset_server.load("fonts/EBGaramond-ExtraBold.ttf")),
            heading: FontSource::Handle(asset_server.load("fonts/Inter-Bold.ttf")),
            body: FontSource::Handle(asset_server.load("fonts/Inter-SemiBold.ttf")),
            medium: FontSource::Handle(asset_server.load("fonts/Inter-Medium.ttf")),
            map_country: FontSource::Handle(asset_server.load("fonts/EBGaramond-ExtraBold.ttf")),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct GameSession {
    pub scenario_id: String,
    pub player_civ_id: u32,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub turn: u32,
}

impl Default for GameSession {
    fn default() -> Self {
        Self {
            scenario_id: "modernworld".to_string(),
            player_civ_id: 1, // Varsayılan: Türkiye
            year: 2026,
            month: 10,
            day: 29,
            turn: 1,
        }
    }
}
