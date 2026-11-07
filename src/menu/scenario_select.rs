use crate::map::{
    generate_random_civ_color, recolor_country_on_map, rerender_full_world_map,
    CameraRig, WorldMapData,
};
use crate::menu::civ_info::{
    get_civ_diplomacy, get_civ_government, get_civ_leader_info, get_civ_religion,
    get_civ_technology, get_government_by_id, get_religion_by_id,
};
use crate::menu::common::{apply_panel_opacity, UiFadeAlphaTracker};
use crate::scenario::{format_number, ActiveScenario};
use crate::state::{AppState, GameSession, PreviousMenuState, UiFonts};
use crate::ui::{
    divider, headed_panel, inset_panel, plain_panel, sliced, spawn_icon_button, ui_button,
    ButtonSize, ButtonVariant, Skins, Theme, Ui, BIG_BUTTON_BORDER, BODY_TEXT, BUTTON_BORDER,
    DIM_TEXT, GOLD_TEXT, INSET_BORDER, PLAIN_BORDER,
};
use bevy::picking::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScenarioSelectRoot;

#[derive(Component)]
pub struct ScenarioSelectAnim {
    pub timer: f32,
    pub duration: f32,
    pub fading_out: bool,
    pub next_target: Option<AppState>,
}

#[derive(Component)]
pub enum CivInfoField {
    Name,
    Capital,
    Leader,
    Gov,
    Religion,
    Stats,
    Allies,
    Rivals,
}

#[derive(Component)]
pub struct SelectedCivFlagNode;

#[derive(Component)]
pub struct SelectedCivColorBadge;

#[derive(Component)]
pub struct ScenarioTitleText;

#[derive(Component)]
pub struct ScenarioDescText;

pub const SCENARIO_DEFS: [(&str, &str, &str); 3] = [
    (
        "modernworld",
        "assets/scenarios/modernworld/scenario.json",
        "Modern Dünya (2026)",
    ),
    (
        "ww1_1914",
        "assets/scenarios/ww1_1914/scenario.json",
        "Büyük Savaş (1914)",
    ),
    (
        "ww2_1939",
        "assets/scenarios/ww2_1939/scenario.json",
        "İkinci Dünya Savaşı (1939)",
    ),
];

pub fn truncate_scenario_description(desc: &str, max_chars: usize) -> String {
    let trimmed = desc.trim();
    if trimmed.chars().count() <= max_chars {
        trimmed.to_string()
    } else {
        let truncated: String = trimmed.chars().take(max_chars).collect();
        if let Some(last_space) = truncated.rfind(' ') {
            format!("{}...", &truncated[..last_space])
        } else {
            format!("{}...", truncated)
        }
    }
}

pub fn setup_scenario_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario: Res<ActiveScenario>,
    session: Res<GameSession>,
    fonts: Res<UiFonts>,
    theme: Res<Theme>,
    ui: Res<Ui>,
    skins: Res<Skins>,
    map_data: Res<crate::map::WorldMapData>,
    mut rig: ResMut<crate::map::CameraRig>,
) {
    let selected_civ = scenario
        .civs
        .get(&session.player_civ_id)
        .or_else(|| scenario.civs.values().next())
        .unwrap();

    let (initial_cam_pos, initial_cam_scale) =
        crate::map::calculate_civ_camera_target(selected_civ, &map_data);
    rig.target_pos = initial_cam_pos;
    rig.target_scale = initial_cam_scale;

    let flag_path = ActiveScenario::get_flag_path(&selected_civ.tag, None);
    let (def_leader_name, leader_title) = get_civ_leader_info(&selected_civ.tag);
    let leader_name = selected_civ.leader.as_deref().unwrap_or(def_leader_name);
    let gov_type = selected_civ
        .government_id
        .map(get_government_by_id)
        .unwrap_or_else(|| get_civ_government(&selected_civ.tag));
    let religion = selected_civ
        .religion_id
        .map(get_religion_by_id)
        .unwrap_or_else(|| get_civ_religion(&selected_civ.tag));
    let tech_desc = get_civ_technology(&selected_civ.tag, selected_civ.economy);
    let (allies, rivals) = get_civ_diplomacy(&selected_civ.tag);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            Pickable::IGNORE,
            ScenarioSelectRoot,
            ScenarioSelectAnim {
                timer: 0.0,
                duration: 0.18,
                fading_out: false,
                next_target: None,
            },
        ))
        .with_children(|root| {
            // ================================================================
            // 1. ÜST PANEL: SENARYO BİLGİSİ VE ÖNCEKİ/SONRAKİ GEÇİŞ KONTROLLERİ
            // ================================================================
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Px(14.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-220.0)),
                ..default()
            })
            .with_children(|top_wrap| {
                plain_panel(
                    top_wrap,
                    &ui,
                    &ui.panel_plain,
                    PLAIN_BORDER,
                    (Val::Px(440.0), Val::Auto),
                    |top_bar| {
                        top_bar
                            .spawn(Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .with_children(|row| {
                                // Sol: Önceki Senaryo Butonu (<<)
                                ui_button(
                                    row,
                                    &ui,
                                    &skins.secondary,
                                    BUTTON_BORDER,
                                    "<<",
                                    Vec2::new(46.0, 36.0),
                                    Some("Önceki Senaryo"),
                                )
                                .observe(
                                    |_: On<PointerClick>,
                                     mut session: ResMut<GameSession>,
                                     mut scenario: ResMut<ActiveScenario>,
                                     map_data: Res<WorldMapData>,
                                     mut images: ResMut<Assets<Image>>| {
                                        let cur_idx = SCENARIO_DEFS
                                            .iter()
                                            .position(|(id, _, _)| *id == session.scenario_id)
                                            .unwrap_or(0);
                                        let prev_idx = if cur_idx == 0 {
                                            SCENARIO_DEFS.len() - 1
                                        } else {
                                            cur_idx - 1
                                        };
                                        let (sc_id, path, _) = SCENARIO_DEFS[prev_idx];
                                        *scenario = ActiveScenario::load_from_file(path);
                                        session.scenario_id = sc_id.to_string();
                                        session.year = scenario.year;
                                        session.month = scenario.month;
                                        session.day = scenario.day;
                                        rerender_full_world_map(&map_data, &scenario, &mut images);
                                    },
                                );

                                // Orta: Senaryo İsmi ve Alt Bilgi (Eyalet Sayısı | Yıl: ...)
                                row.spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    width: Val::Px(280.0),
                                    max_width: Val::Px(280.0),
                                    row_gap: Val::Px(2.0),
                                    overflow: Overflow::clip(),
                                    ..default()
                                })
                                .with_children(|center_col| {
                                    center_col.spawn((
                                        Text::new(scenario.name.clone()),
                                        TextFont {
                                            font: ui.font.clone().into(),
                                            font_size: FontSize::Px(17.0),
                                            ..default()
                                        },
                                        TextColor(GOLD_TEXT),
                                        ScenarioTitleText,
                                    ));

                                    let civ_count = scenario
                                        .civs
                                        .values()
                                        .filter(|c| !c.provinces.is_empty())
                                        .count();
                                    let meta_str =
                                        format!("{} Ülke  |  Yıl: {}", civ_count, scenario.year);

                                    center_col.spawn((
                                        Text::new(meta_str),
                                        TextFont {
                                            font: ui.font.clone().into(),
                                            font_size: FontSize::Px(12.0),
                                            ..default()
                                        },
                                        TextLayout::justify(Justify::Center),
                                        TextColor(DIM_TEXT),
                                        ScenarioDescText,
                                    ));
                                });

                                // Sağ: Sonraki Senaryo Butonu (>>)
                                ui_button(
                                    row,
                                    &ui,
                                    &skins.secondary,
                                    BUTTON_BORDER,
                                    ">>",
                                    Vec2::new(46.0, 36.0),
                                    Some("Sonraki Senaryo"),
                                )
                                .observe(
                                    |_: On<PointerClick>,
                                     mut session: ResMut<GameSession>,
                                     mut scenario: ResMut<ActiveScenario>,
                                     map_data: Res<WorldMapData>,
                                     mut images: ResMut<Assets<Image>>| {
                                        let cur_idx = SCENARIO_DEFS
                                            .iter()
                                            .position(|(id, _, _)| *id == session.scenario_id)
                                            .unwrap_or(0);
                                        let next_idx = (cur_idx + 1) % SCENARIO_DEFS.len();
                                        let (sc_id, path, _) = SCENARIO_DEFS[next_idx];
                                        *scenario = ActiveScenario::load_from_file(path);
                                        session.scenario_id = sc_id.to_string();
                                        session.year = scenario.year;
                                        session.month = scenario.month;
                                        session.day = scenario.day;
                                        rerender_full_world_map(&map_data, &scenario, &mut images);
                                    },
                                );
                            });
                    },
                );
            });

            // ================================================================
            // 2. SOL PANEL: SEÇİLEN ÜLKE BİLGİ KARTI (EU4 Headed Panel)
            // ================================================================
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Px(88.0),
                left: Val::Px(16.0),
                ..default()
            })
            .with_children(|left_wrap| {
                headed_panel(
                    left_wrap,
                    &ui,
                    &ui.headed,
                    "SEÇİLEN DEVLET",
                    (Val::Px(380.0), Val::Px(540.0)),
                    |card| {
                        // Başlık & Bayrak (flag_frame.png içinde)
                        card.spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(12.0),
                            ..default()
                        })
                        .with_children(|hdr| {
                            hdr.spawn((
                                sliced(&ui.flag_frame, INSET_BORDER),
                                Node {
                                    width: Val::Px(76.0),
                                    height: Val::Px(52.0),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|flag_box| {
                                flag_box.spawn((
                                    ImageNode {
                                        image: asset_server.load(flag_path),
                                        ..default()
                                    },
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    SelectedCivFlagNode,
                                ));
                                flag_box.spawn((
                                    ImageNode {
                                        image: asset_server.load("flags/flagOver.png"),
                                        ..default()
                                    },
                                    Node {
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(5.0),
                                        left: Val::Px(5.0),
                                        right: Val::Px(5.0),
                                        bottom: Val::Px(5.0),
                                        ..default()
                                    },
                                    Pickable::IGNORE,
                                ));
                            });

                            hdr.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(3.0),
                                flex_grow: 1.0,
                                ..default()
                            })
                            .with_children(|col| {
                                col.spawn((
                                    Text::new(&selected_civ.name),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(19.0),
                                        ..default()
                                    },
                                    TextColor(GOLD_TEXT),
                                    CivInfoField::Name,
                                ));

                                col.spawn((
                                    Text::new(format!("Başkent: {}", selected_civ.capital_name)),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(DIM_TEXT),
                                    CivInfoField::Capital,
                                ));
                            });
                        });

                        divider(card, &ui);

                        // Lider & Ünvan Kartı (inset_panel)
                        inset_panel(
                            card,
                            &ui,
                            (Val::Percent(100.0), Val::Auto),
                            UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            |box_col| {
                                box_col.spawn((
                                    Text::new(format!("Lider: {} ({})", leader_name, leader_title)),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(13.5),
                                        ..default()
                                    },
                                    TextColor(BODY_TEXT),
                                    CivInfoField::Leader,
                                ));
                                box_col.spawn((
                                    Text::new(format!("Yönetim: {}", gov_type)),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(GOLD_TEXT),
                                    CivInfoField::Gov,
                                ));
                                box_col.spawn((
                                    Text::new(format!("Dini: {}", religion)),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(DIM_TEXT),
                                    CivInfoField::Religion,
                                ));
                            },
                        );

                        // Ulusal İstatistikler (inset_panel)
                        inset_panel(
                            card,
                            &ui,
                            (Val::Percent(100.0), Val::Auto),
                            UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            |stats_box| {
                                stats_box.spawn((
                                    Text::new(format!(
                                        "Eyaletler: {} Şehir  •  Nüfus: {}\nEkonomi: {}  •  Teknoloji: {}",
                                        selected_civ.provinces.len(),
                                        format_number(selected_civ.population),
                                        format_number(selected_civ.economy as u64),
                                        tech_desc
                                    )),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(BODY_TEXT),
                                    CivInfoField::Stats,
                                ));
                            },
                        );

                        // Jeopolitik & İttifak Durumu (inset_panel)
                        inset_panel(
                            card,
                            &ui,
                            (Val::Percent(100.0), Val::Auto),
                            UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            |dip_box| {
                                dip_box.spawn((
                                    Text::new(format!("Müttefikler:\n{}", allies)),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(12.0),
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.55, 0.85, 0.55)),
                                    CivInfoField::Allies,
                                ));
                                dip_box.spawn((
                                    Text::new(format!("Rakipler:\n{}", rivals)),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(12.0),
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.95, 0.45, 0.45)),
                                    CivInfoField::Rivals,
                                ));
                            },
                        );

                        // Ülke Harita Rengi & Hızlı Renk Paleti (inset_panel)
                        inset_panel(
                            card,
                            &ui,
                            (Val::Percent(100.0), Val::Auto),
                            UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            |color_sec| {
                                color_sec
                                    .spawn(Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    })
                                    .with_children(|head| {
                                        head.spawn((
                                            Text::new("Harita Rengi"),
                                            TextFont {
                                                font: ui.font.clone().into(),
                                                font_size: FontSize::Px(13.0),
                                                ..default()
                                            },
                                            TextColor(DIM_TEXT),
                                        ));

                                        head.spawn((
                                            Node {
                                                width: Val::Px(28.0),
                                                height: Val::Px(16.0),
                                                border: UiRect::all(Val::Px(1.0)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb_u8(
                                                selected_civ.color[0],
                                                selected_civ.color[1],
                                                selected_civ.color[2],
                                            )),
                                            BorderColor::all(Color::WHITE),
                                            SelectedCivColorBadge,
                                        ));
                                    });

                                color_sec
                                    .spawn(Node {
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(6.0),
                                        align_items: AlignItems::Center,
                                        margin: UiRect::top(Val::Px(4.0)),
                                        ..default()
                                    })
                                    .with_children(|palette| {
                                        let preset_colors = [
                                            [210, 35, 42],
                                            [30, 80, 185],
                                            [35, 140, 55],
                                            [220, 175, 25],
                                            [120, 40, 150],
                                            [215, 110, 20],
                                            [130, 140, 150],
                                        ];

                                        for rgb in preset_colors {
                                            palette
                                                .spawn((
                                                    Node {
                                                        width: Val::Px(24.0),
                                                        height: Val::Px(24.0),
                                                        border: UiRect::all(Val::Px(1.0)),
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb_u8(
                                                        rgb[0], rgb[1], rgb[2],
                                                    )),
                                                    BorderColor::all(theme.colors.border_subtle),
                                                ))
                                                .observe(
                                                    |ev: On<PointerOver>,
                                                     mut bq: Query<&mut BorderColor>| {
                                                        if let Ok(mut b) = bq.get_mut(ev.entity) {
                                                            *b = BorderColor::all(Color::WHITE);
                                                        }
                                                    },
                                                )
                                                .observe({
                                                    let border_subtle = theme.colors.border_subtle;
                                                    move |ev: On<PointerOut>,
                                                          mut bq: Query<&mut BorderColor>| {
                                                        if let Ok(mut b) = bq.get_mut(ev.entity) {
                                                            *b = BorderColor::all(border_subtle);
                                                        }
                                                    }
                                                })
                                                .observe(
                                                    move |_: On<PointerClick>,
                                                          session: Res<GameSession>,
                                                          map_data: Res<WorldMapData>,
                                                          mut scenario: ResMut<ActiveScenario>,
                                                          mut images: ResMut<Assets<Image>>| {
                                                        let cid = session.player_civ_id;
                                                        if let Some(civ) =
                                                            scenario.civs.get_mut(&cid)
                                                        {
                                                            civ.color = rgb;
                                                        }
                                                        recolor_country_on_map(
                                                            &map_data, &scenario, &mut images, cid,
                                                        );
                                                    },
                                                );
                                        }

                                        ui_button(
                                            palette,
                                            &ui,
                                            &skins.secondary,
                                            BUTTON_BORDER,
                                            "Rastgele",
                                            Vec2::new(82.0, 28.0),
                                            Some("Rastgele Harita Rengi Ata"),
                                        )
                                        .observe(
                                            |_: On<PointerClick>,
                                             session: Res<GameSession>,
                                             map_data: Res<WorldMapData>,
                                             mut scenario: ResMut<ActiveScenario>,
                                             mut images: ResMut<Assets<Image>>| {
                                                let cid = session.player_civ_id;
                                                let rgb = generate_random_civ_color(cid);
                                                if let Some(civ) = scenario.civs.get_mut(&cid) {
                                                    civ.color = rgb;
                                                }
                                                recolor_country_on_map(
                                                    &map_data, &scenario, &mut images, cid,
                                                );
                                            },
                                        );
                                    });
                            },
                        );
                    },
                );
            });

            // ================================================================
            // 3. SAĞ ÜST BAR: OYUN AYARLARI TUŞU
            // ================================================================
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Px(14.0),
                right: Val::Px(16.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|top_right| {
                ui_button(
                    top_right,
                    &ui,
                    &skins.secondary,
                    BUTTON_BORDER,
                    "Oyun Ayarları",
                    Vec2::new(160.0, 38.0),
                    Some("Oyun ve Grafik Ayarlarını Aç"),
                )
                .observe(
                    |_: On<PointerClick>,
                     mut prev: ResMut<PreviousMenuState>,
                     mut anim_q: Query<&mut ScenarioSelectAnim>| {
                        prev.0 = AppState::ScenarioSelect;
                        for mut anim in &mut anim_q {
                            if !anim.fading_out {
                                anim.fading_out = true;
                                anim.timer = 0.0;
                                anim.duration = 0.16;
                                anim.next_target = Some(AppState::Settings);
                            }
                        }
                    },
                );
            });

            // ================================================================
            // 4. ALT BAR: GERİ (SOL), ÖNERİLEN GÜÇLER (ORTA), BAŞLAT (SAĞ)
            // ================================================================
            root.spawn((
                sliced(&ui.panel_plain, PLAIN_BORDER),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(12.0),
                    left: Val::Px(16.0),
                    right: Val::Px(16.0),
                    height: Val::Px(64.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                    ..default()
                },
            ))
            .with_children(|bar| {
                // Sol Alt: Geri Butonu (Ana Menüye Dön)
                let mut back_btn = spawn_icon_button(
                    bar,
                    &theme,
                    &fonts,
                    "Ana Menüye Dön",
                    "icons/arrow_left.png",
                    false,
                    13.0,
                    ButtonVariant::Secondary,
                    ButtonSize::Medium,
                    &asset_server,
                );
                back_btn.observe(
                    |_: On<PointerClick>,
                     mut prev: ResMut<PreviousMenuState>,
                     mut anim_q: Query<&mut ScenarioSelectAnim>| {
                        prev.0 = AppState::ScenarioSelect;
                        for mut anim in &mut anim_q {
                            if !anim.fading_out {
                                anim.fading_out = true;
                                anim.timer = 0.0;
                                anim.duration = 0.16;
                                anim.next_target = Some(AppState::MainMenu);
                            }
                        }
                    },
                );

                // Orta: Önerilen / En Güçlü Ülkeler (Hızlı Seçim Çubuğu)
                bar.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    ..default()
                })
                .with_children(|center| {
                    center.spawn((
                        Text::new("Önerilen Güçler:"),
                        TextFont {
                            font: ui.font.clone().into(),
                            font_size: FontSize::Px(13.0),
                            ..default()
                        },
                        TextColor(DIM_TEXT),
                    ));

                    let quick_tags = [
                        ("tur", "Türkiye"),
                        ("usa", "ABD"),
                        ("rus", "Rusya"),
                        ("chi_c", "Çin"),
                        ("ger", "Almanya"),
                        ("uni_m", "İngiltere"),
                        ("fra", "Fransa"),
                        ("aze", "Azerbaycan"),
                    ];

                    for (tag, label) in quick_tags {
                        if let Some(civ) = scenario
                            .civs
                            .values()
                            .find(|c| c.tag == tag || c.tag.starts_with(tag))
                        {
                            let cid = civ.id;
                            ui_button(
                                center,
                                &ui,
                                &skins.pill,
                                BUTTON_BORDER,
                                label,
                                Vec2::new(88.0, 32.0),
                                Some("Hızlı Ülke Seçimi"),
                            )
                            .observe(
                                move |_: On<PointerClick>,
                                      mut session: ResMut<GameSession>,
                                      scenario: Res<ActiveScenario>,
                                      map_data: Res<WorldMapData>,
                                      mut rig: ResMut<CameraRig>| {
                                    session.player_civ_id = cid;
                                    if let Some(c) = scenario.civs.get(&cid) {
                                        let (target_pos, target_scale) =
                                            crate::map::calculate_civ_camera_target(c, &map_data);
                                        rig.target_pos = target_pos;
                                        rig.target_scale = target_scale;
                                    }
                                },
                            );
                        }
                    }
                });

                // Sağ Alt: Başlat Butonu (EU4 Big Primary Button)
                ui_button(
                    bar,
                    &ui,
                    &skins.big_primary,
                    BIG_BUTTON_BORDER,
                    "OYUNU BAŞLAT",
                    Vec2::new(210.0, 44.0),
                    Some("Seçilen devlet ile seferi başlat"),
                )
                .observe(
                    |_: On<PointerClick>,
                     session: Res<GameSession>,
                     scenario: Res<ActiveScenario>,
                     map_data: Res<WorldMapData>,
                     mut rig: ResMut<CameraRig>,
                     mut prev: ResMut<PreviousMenuState>,
                     mut anim_q: Query<&mut ScenarioSelectAnim>| {
                        if let Some(civ) = scenario.civs.get(&session.player_civ_id) {
                            let (target_pos, target_scale) =
                                crate::map::calculate_civ_camera_target(civ, &map_data);
                            rig.target_pos = target_pos;
                            rig.target_scale = (target_scale * 0.90).clamp(0.25, 1.25);
                        }
                        prev.0 = AppState::ScenarioSelect;
                        for mut anim in &mut anim_q {
                            if !anim.fading_out {
                                anim.fading_out = true;
                                anim.timer = 0.0;
                                anim.duration = 0.16;
                                anim.next_target = Some(AppState::InGame);
                            }
                        }
                    },
                );
            });
        });
}

pub fn handle_scenario_select_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut prev: ResMut<PreviousMenuState>,
    mut anim_q: Query<&mut ScenarioSelectAnim>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        prev.0 = AppState::ScenarioSelect;
        for mut anim in &mut anim_q {
            if !anim.fading_out {
                anim.fading_out = true;
                anim.timer = 0.0;
                anim.duration = 0.16;
                anim.next_target = Some(AppState::MainMenu);
            }
        }
    }
}

pub fn update_scenario_select_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    mut anim_q: Query<(Entity, &mut ScenarioSelectAnim)>,
    children_q: Query<&Children>,
    fade_q: Query<&UiFadeAlphaTracker>,
    mut img_q: Query<&mut ImageNode>,
    mut text_q: Query<&mut TextColor>,
    mut bg_q: Query<&mut BackgroundColor>,
    mut border_q: Query<&mut BorderColor>,
) {
    for (root, mut anim) in &mut anim_q {
        let opacity = if !anim.fading_out {
            anim.timer += time.delta_secs();
            let progress = (anim.timer / anim.duration).clamp(0.0, 1.0);
            1.0 - (1.0 - progress).powi(3)
        } else {
            anim.timer += time.delta_secs();
            let progress = (anim.timer / anim.duration).clamp(0.0, 1.0);
            let ease = progress * progress;
            if progress >= 1.0 {
                if let Some(target) = anim.next_target.take() {
                    next_state.set(target);
                }
            }
            (1.0 - ease).clamp(0.0, 1.0)
        };

        apply_panel_opacity(
            root,
            opacity,
            &mut commands,
            &children_q,
            &fade_q,
            &mut img_q,
            &mut text_q,
            &mut bg_q,
            &mut border_q,
        );
    }
}

pub fn sync_selected_civ_ui(
    session: Res<GameSession>,
    scenario: Res<ActiveScenario>,
    asset_server: Res<AssetServer>,
    mut text_fields_q: Query<(&CivInfoField, &mut Text)>,
    mut badge_q: Query<&mut BackgroundColor, With<SelectedCivColorBadge>>,
    mut flag_q: Query<&mut ImageNode, With<SelectedCivFlagNode>>,
    mut sc_title_q: Query<
        &mut Text,
        (
            With<ScenarioTitleText>,
            Without<CivInfoField>,
            Without<ScenarioDescText>,
        ),
    >,
    mut sc_desc_q: Query<
        &mut Text,
        (
            With<ScenarioDescText>,
            Without<CivInfoField>,
            Without<ScenarioTitleText>,
        ),
    >,
) {
    if !session.is_changed() && !scenario.is_changed() {
        return;
    }

    for mut t in &mut sc_title_q {
        t.0 = scenario.name.clone();
    }
    for mut t in &mut sc_desc_q {
        let civ_count = scenario
            .civs
            .values()
            .filter(|c| !c.provinces.is_empty())
            .count();
        t.0 = format!("{} Ülke  |  Yıl: {}", civ_count, scenario.year);
    }

    if let Some(civ) = scenario.civs.get(&session.player_civ_id) {
        let (def_leader_name, leader_title) = get_civ_leader_info(&civ.tag);
        let leader_name = civ.leader.as_deref().unwrap_or(def_leader_name);
        let gov_type = civ
            .government_id
            .map(get_government_by_id)
            .unwrap_or_else(|| get_civ_government(&civ.tag));
        let religion = civ
            .religion_id
            .map(get_religion_by_id)
            .unwrap_or_else(|| get_civ_religion(&civ.tag));
        let tech_desc = get_civ_technology(&civ.tag, civ.economy);
        let (allies, rivals) = get_civ_diplomacy(&civ.tag);

        for (field, mut text) in &mut text_fields_q {
            match field {
                CivInfoField::Name => text.0 = civ.name.clone(),
                CivInfoField::Capital => text.0 = format!("Başkent: {}", civ.capital_name),
                CivInfoField::Leader => {
                    text.0 = format!("Lider: {} ({})", leader_name, leader_title)
                }
                CivInfoField::Gov => text.0 = format!("Yönetim: {}", gov_type),
                CivInfoField::Religion => text.0 = format!("Dini: {}", religion),
                CivInfoField::Stats => {
                    text.0 = format!(
                        "Eyaletler: {} Şehir  •  Nüfus: {}\nEkonomi: {}  •  Teknoloji: {}",
                        civ.provinces.len(),
                        format_number(civ.population),
                        format_number(civ.economy as u64),
                        tech_desc
                    );
                }
                CivInfoField::Allies => text.0 = format!("Müttefikler:\n{}", allies),
                CivInfoField::Rivals => text.0 = format!("Rakipler:\n{}", rivals),
            }
        }

        for mut bg in &mut badge_q {
            bg.0 = Color::srgb_u8(civ.color[0], civ.color[1], civ.color[2]);
        }

        for mut img in &mut flag_q {
            let flag_path = ActiveScenario::get_flag_path(&civ.tag, None);
            img.image = asset_server.load(flag_path);
        }
    }
}

pub fn cleanup_scenario_select(mut commands: Commands, q: Query<Entity, With<ScenarioSelectRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
