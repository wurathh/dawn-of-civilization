use crate::map::{
    generate_random_civ_color, map_pixel_to_world, recolor_country_on_map, CameraRig, WorldMapData,
};
use crate::scenario::{format_number, ActiveScenario};
use crate::state::{AppState, GameSession, GameSettings, UiFonts};
use crate::ui::{
    spawn_button, spawn_gold_divider, ButtonSize, ButtonVariant, Theme,
};
use bevy::app::AppExit;
use bevy::picking::prelude::*;
use bevy::prelude::*;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
            .add_systems(OnEnter(AppState::ScenarioCatalog), setup_scenario_catalog)
            .add_systems(OnExit(AppState::ScenarioCatalog), cleanup_scenario_catalog)
            .add_systems(OnEnter(AppState::ScenarioSelect), setup_scenario_select)
            .add_systems(OnExit(AppState::ScenarioSelect), cleanup_scenario_select)
            .add_systems(
                Update,
                (
                    sync_selected_civ_ui,
                    sync_settings_modal_ui,
                )
                    .run_if(in_state(AppState::ScenarioSelect)),
            );
    }
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
struct ScenarioCatalogRoot;

#[derive(Component)]
struct ScenarioSelectRoot;

#[derive(Component)]
struct SettingsModalRoot;

#[derive(Component)]
enum CivInfoField {
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
enum SettingField {
    FogOfWar,
    Difficulty,
    AiAggressiveness,
    Ironman,
}

#[derive(Component)]
struct SelectedCivFlagNode;

#[derive(Component)]
struct SelectedCivColorBadge;

/// `assets/backgrounds` klasöründeki tüm resimleri (.png, .jpg, .jpeg, .webp) tarar
/// ve her çağrıldığında rastgele bir tanesinin asset yolunu döndürür.
fn pick_random_background_image() -> Option<String> {
    let dir = fs::read_dir("assets/backgrounds").ok()?;
    let mut files: Vec<String> = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            let ext = path.extension()?.to_str()?.to_ascii_lowercase();
            if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp") {
                let fname = path.file_name()?.to_str()?;
                Some(format!("backgrounds/{}", fname))
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        return None;
    }

    files.sort();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as usize + (d.as_secs() as usize) * 31)
        .unwrap_or(0);
    let idx = nanos % files.len();
    Some(files.swap_remove(idx))
}

// ============================================================================
// 1. ANA MENÜ (MAIN MENU)
// ============================================================================

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<UiFonts>,
    theme: Res<Theme>,
) {
    let bg_image_opt = pick_random_background_image();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::left(theme.spacing.val_xxl()),
                ..default()
            },
            BackgroundColor(theme.colors.bg_base),
            MainMenuRoot,
        ))
        .with_children(|root| {
            // 1. Tam Ekran Rastgele Arka Plan Görseli
            if let Some(bg_path) = bg_image_opt {
                root.spawn((
                    ImageNode {
                        image: asset_server.load(bg_path),
                        image_mode: NodeImageMode::Stretch,
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    Pickable::IGNORE,
                ));
            }

            // 2. Sinematik Koyu Karartma Katmanı
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(theme.colors.overlay_dim()),
                Pickable::IGNORE,
            ));

            // 3. Modern Yüzen (Floating) Ana Menü Kartı (Saf Rust %100 Opak Asil Obsidyen + Altın Kenarlık)
            root.spawn((
                Node {
                    width: Val::Px(460.0),
                    padding: UiRect::all(theme.spacing.val_xl()),
                    flex_direction: FlexDirection::Column,
                    row_gap: theme.spacing.val_lg(),
                    position_type: PositionType::Relative,
                    overflow: Overflow::clip(),
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.08, 0.11)),
                BorderColor::all(Color::srgb(0.78, 0.62, 0.28)),
            ))
            .with_children(|card| {
                // Başlık
                card.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: theme.spacing.val_xs(),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("DAWN OF\nCIVILIZATION"),
                        TextFont {
                            font: fonts.display.clone(),
                            font_size: FontSize::Px(theme.typography.display),
                            ..default()
                        },
                        TextColor(Color::srgb(0.94, 0.82, 0.45)),
                    ));

                    spawn_gold_divider(header, &theme, Val::Px(160.0));

                    header.spawn((
                        Text::new("Büyük Strateji ve Dünya Hakimiyeti"),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.body_md()),
                            ..default()
                        },
                        TextColor(theme.colors.text_secondary),
                    ));
                });

                // Menü Butonları: "Yeni Oyun Oluştur" ve "Oyundan Çık" (Zarif Solid Butonlar)
                card.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: theme.spacing.val_sm(),
                    width: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|buttons| {
                    let mut new_game_btn = spawn_button(
                        buttons,
                        &theme,
                        &fonts,
                        "YENİ OYUN OLUŞTUR",
                        ButtonVariant::Primary,
                        ButtonSize::Large,
                    );
                    new_game_btn.observe(
                        |_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
                            next_state.set(AppState::ScenarioCatalog);
                        },
                    );

                    let mut exit_btn = spawn_button(
                        buttons,
                        &theme,
                        &fonts,
                        "OYUNDAN ÇIK",
                        ButtonVariant::Secondary,
                        ButtonSize::Large,
                    );
                    exit_btn.observe(
                        |_: On<PointerClick>, mut exit_writer: MessageWriter<AppExit>| {
                            exit_writer.write(AppExit::Success);
                        },
                    );
                });

                // Alt Bilgi
                card.spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::top(theme.spacing.val_xs()),
                    ..default()
                })
                .with_children(|footer| {
                    footer.spawn((
                        Text::new("Sürüm 0.1.0"),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.caption),
                            ..default()
                        },
                        TextColor(theme.colors.text_disabled),
                    ));
                    footer.spawn((
                        Text::new("6.133 Şehir  •  203 Ülke"),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.caption),
                            ..default()
                        },
                        TextColor(theme.colors.accent_muted()),
                    ));
                });
            });
        });
}

fn cleanup_main_menu(mut commands: Commands, q: Query<Entity, With<MainMenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

// ============================================================================
// 2. SENARYO SEÇİM EKRANI (SCENARIO CATALOG)
// ============================================================================

fn setup_scenario_catalog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<UiFonts>,
    scenario: Res<ActiveScenario>,
    theme: Res<Theme>,
) {
    let bg_image_opt = pick_random_background_image();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(theme.spacing.val_xl()),
                ..default()
            },
            BackgroundColor(theme.colors.bg_base),
            ScenarioCatalogRoot,
        ))
        .with_children(|root| {
            // Arka Plan
            if let Some(bg_path) = bg_image_opt {
                root.spawn((
                    ImageNode {
                        image: asset_server.load(bg_path),
                        image_mode: NodeImageMode::Stretch,
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    Pickable::IGNORE,
                ));
            }

            // Karartma Katmanı
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(theme.colors.overlay_dim()),
                Pickable::IGNORE,
            ));

            // Üst Başlık
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: theme.spacing.val_xs(),
                ..default()
            })
            .with_children(|header| {
                header.spawn((
                    Text::new("SENARYO SEÇİMİ"),
                    TextFont {
                        font: fonts.display.clone(),
                        font_size: FontSize::Px(theme.typography.display),
                        ..default()
                    },
                    TextColor(theme.colors.accent),
                ));

                spawn_gold_divider(header, &theme, Val::Px(180.0));

                header.spawn((
                    Text::new("Yönetmek istediğiniz çağı ve jeopolitik senaryoyu belirleyin"),
                    TextFont {
                        font: fonts.medium.clone(),
                        font_size: FontSize::Px(theme.typography.body_md()),
                        ..default()
                    },
                    TextColor(theme.colors.text_secondary),
                ));
            });

            // Orta: Senaryo Kartı (Saf Rust %100 Opak Asil Obsidyen + Altın Kenarlık)
            root.spawn((
                Node {
                    width: Val::Px(640.0),
                    padding: UiRect::all(theme.spacing.val_xl()),
                    flex_direction: FlexDirection::Column,
                    row_gap: theme.spacing.val_lg(),
                    position_type: PositionType::Relative,
                    overflow: Overflow::clip(),
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.08, 0.11)),
                BorderColor::all(Color::srgb(0.78, 0.62, 0.28)),
            ))
            .with_children(|card| {

                // Senaryo Başlık Satırı
                card.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("🌍  Modern Dünya"),
                        TextFont {
                            font: fonts.display.clone(),
                            font_size: FontSize::Px(theme.typography.headline),
                            ..default()
                        },
                        TextColor(theme.colors.accent),
                    ));
                    row.spawn((
                        Node {
                            padding: UiRect::axes(theme.spacing.val_sm(), theme.spacing.val_xs()),
                            border_radius: BorderRadius::all(theme.radius.val_sm()),
                            border: UiRect::all(Val::Px(theme.borders.thin)),
                            ..default()
                        },
                        BackgroundColor(theme.colors.surface_panel()),
                        BorderColor::all(theme.colors.accent_muted()),
                    ))
                    .with_children(|badge| {
                        badge.spawn((
                            Text::new("2026 Çağı"),
                            TextFont {
                                font: fonts.heading.clone(),
                                font_size: FontSize::Px(theme.typography.label_md()),
                                ..default()
                            },
                            TextColor(theme.colors.accent_hover),
                        ));
                    });
                });

                // Senaryo Açıklaması
                card.spawn((
                    Text::new(
                        "Günümüzün jeopolitik güç dengeleri, nükleer caydırıcılık, küresel ticaret ağları ve bölgesel gerilimler. Dünyanın kaderini belirleyecek lider olun."
                    ),
                    TextFont {
                        font: fonts.medium.clone(),
                        font_size: FontSize::Px(theme.typography.body_md()),
                        ..default()
                    },
                    TextColor(theme.colors.text_secondary),
                ));

                // İstatistik Rozetleri
                card.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: theme.spacing.val_sm(),
                    margin: UiRect::vertical(theme.spacing.val_xs()),
                    ..default()
                })
                .with_children(|stats| {
                    let items = [
                        (format!("{} Ülke", scenario.civs.len()), "Bağımsız Egemen Devletler"),
                        ("6.133 Şehir".to_string(), "Gerçek Eyalet Sınırları"),
                        ("29 Ekim 2026".to_string(), "Başlangıç Tarihi"),
                    ];
                    for (val, desc) in items {
                        stats.spawn((
                            Node {
                                flex_grow: 1.0,
                                padding: UiRect::axes(theme.spacing.val_sm(), theme.spacing.val_sm()),
                                flex_direction: FlexDirection::Column,
                                row_gap: theme.spacing.val_xxs(),
                                border: UiRect::all(Val::Px(theme.borders.thin)),
                                border_radius: BorderRadius::all(theme.radius.val_md()),
                                ..default()
                            },
                            BackgroundColor(theme.colors.surface_panel()),
                            BorderColor::all(theme.colors.border_subtle),
                        ))
                        .with_children(|box_col| {
                            box_col.spawn((
                                Text::new(val),
                                TextFont {
                                    font: fonts.heading.clone(),
                                    font_size: FontSize::Px(theme.typography.title_sm()),
                                    ..default()
                                },
                                TextColor(theme.colors.accent),
                            ));
                            box_col.spawn((
                                Text::new(desc),
                                TextFont {
                                    font: fonts.medium.clone(),
                                    font_size: FontSize::Px(theme.typography.caption),
                                    ..default()
                                },
                                TextColor(theme.colors.text_secondary),
                            ));
                        });
                    }
                });

                // Senaryoyu Seç Butonu (Saf Rust Solid Primary Button)
                let mut start_btn = spawn_button(
                    card,
                    &theme,
                    &fonts,
                    "Senaryoyu Başlat & Haritaya Geç  ▶",
                    ButtonVariant::Primary,
                    ButtonSize::Large,
                );
                start_btn.observe(
                    |_: On<PointerClick>,
                     mut rig: ResMut<CameraRig>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        rig.target_pos = Vec2::new(850.0, 580.0);
                        rig.target_scale = 0.85;
                        next_state.set(AppState::ScenarioSelect);
                    },
                );
            });

            // Alt Bar: Ana Menüye Dön
            root.spawn(Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                ..default()
            })
            .with_children(|footer| {
                let mut back_btn = spawn_button(
                    footer,
                    &theme,
                    &fonts,
                    "◀  Ana Menüye Dön",
                    ButtonVariant::Secondary,
                    ButtonSize::Medium,
                );
                back_btn.observe(
                    |_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::MainMenu);
                    },
                );
            });
        });
}

fn cleanup_scenario_catalog(mut commands: Commands, q: Query<Entity, With<ScenarioCatalogRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

// ============================================================================
// 3. ÜLKE & OYUN SEÇİM EKRANI (HARİTA ÜZERİNDE SCENARIO SELECT)
// ============================================================================

fn setup_scenario_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario: Res<ActiveScenario>,
    session: Res<GameSession>,
    _settings: Res<GameSettings>,
    fonts: Res<UiFonts>,
    theme: Res<Theme>,
) {
    let selected_civ = scenario
        .civs
        .get(&session.player_civ_id)
        .or_else(|| scenario.civs.values().next())
        .unwrap();

    let flag_path = ActiveScenario::get_flag_path(&selected_civ.tag, None);
    let (leader_name, leader_title) = get_civ_leader_info(&selected_civ.tag);
    let gov_type = get_civ_government(&selected_civ.tag);
    let religion = get_civ_religion(&selected_civ.tag);
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
        ))
        .with_children(|root| {
            // ----------------------------------------------------------------
            // 1. SOL TARAF: SEÇİLEN ÜLKE BİLGİ POPUP KARTI (Saf Rust %100 Opak Asil Obsidyen + Altın Çerçeve)
            // ----------------------------------------------------------------
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: theme.spacing.val_lg(),
                    left: theme.spacing.val_lg(),
                    bottom: Val::Px(88.0),
                    width: Val::Px(390.0),
                    padding: UiRect::all(theme.spacing.val_lg()),
                    flex_direction: FlexDirection::Column,
                    row_gap: theme.spacing.val_md(),
                    overflow: Overflow::clip_y(),
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.08, 0.11)),
                BorderColor::all(Color::srgb(0.78, 0.62, 0.28)),
            ))
            .with_children(|card| {
                // Başlık & Bayrak
                card.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: theme.spacing.val_md(),
                    ..default()
                })
                .with_children(|hdr| {
                    hdr.spawn(Node {
                        width: Val::Px(64.0),
                        height: Val::Px(42.0),
                        position_type: PositionType::Relative,
                        border: UiRect::all(Val::Px(theme.borders.thin)),
                        border_radius: BorderRadius::all(theme.radius.val_sm()),
                        overflow: Overflow::clip(),
                        ..default()
                    })
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
                                top: Val::Px(0.0),
                                left: Val::Px(0.0),
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            Pickable::IGNORE,
                        ));
                    });

                    hdr.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(2.0),
                        flex_grow: 1.0,
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn((
                            Text::new(&selected_civ.name),
                            TextFont {
                                font: fonts.display.clone(),
                                font_size: FontSize::Px(theme.typography.headline),
                                ..default()
                            },
                            TextColor(theme.colors.accent),
                            CivInfoField::Name,
                        ));

                        col.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: theme.spacing.val_xs(),
                            ..default()
                        })
                        .with_children(|sub| {
                            sub.spawn((
                                Node {
                                    width: Val::Px(12.0),
                                    height: Val::Px(12.0),
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb_u8(
                                    selected_civ.color[0],
                                    selected_civ.color[1],
                                    selected_civ.color[2],
                                )),
                                SelectedCivColorBadge,
                            ));
                            sub.spawn((
                                Text::new(format!("Başkent: {}", selected_civ.capital_name)),
                                TextFont {
                                    font: fonts.medium.clone(),
                                    font_size: FontSize::Px(theme.typography.label_md()),
                                    ..default()
                                },
                                TextColor(theme.colors.text_secondary),
                                CivInfoField::Capital,
                            ));
                        });
                    });
                });

                // Kutu 1: Lider, Yönetim & Din
                card.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(theme.spacing.val_md()),
                        flex_direction: FlexDirection::Column,
                        row_gap: theme.spacing.val_xs(),
                        border: UiRect::all(Val::Px(theme.borders.thin)),
                        border_radius: BorderRadius::all(theme.radius.val_md()),
                        ..default()
                    },
                    BackgroundColor(theme.colors.surface_panel()),
                    BorderColor::all(theme.colors.border_subtle),
                ))
                .with_children(|b| {
                    b.spawn((
                        Text::new(format!("Lider: {} ({})", leader_name, leader_title)),
                        TextFont {
                            font: fonts.heading.clone(),
                            font_size: FontSize::Px(theme.typography.label_lg()),
                            ..default()
                        },
                        TextColor(theme.colors.accent),
                        CivInfoField::Leader,
                    ));
                    b.spawn((
                        Text::new(format!("Yönetim: {}", gov_type)),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.body_sm()),
                            ..default()
                        },
                        TextColor(theme.colors.text_secondary),
                        CivInfoField::Gov,
                    ));
                    b.spawn((
                        Text::new(format!("Dini: {}", religion)),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.body_sm()),
                            ..default()
                        },
                        TextColor(theme.colors.text_secondary),
                        CivInfoField::Religion,
                    ));
                });

                // Kutu 2: Ulusal Göstergeler (Eyalet, Nüfus, Ekonomi, Teknoloji)
                card.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(theme.spacing.val_md()),
                        flex_direction: FlexDirection::Column,
                        row_gap: theme.spacing.val_xs(),
                        border: UiRect::all(Val::Px(theme.borders.thin)),
                        border_radius: BorderRadius::all(theme.radius.val_md()),
                        ..default()
                    },
                    BackgroundColor(theme.colors.surface_panel()),
                    BorderColor::all(theme.colors.border_subtle),
                ))
                .with_children(|b| {
                    b.spawn((
                        Text::new(format!(
                            "Eyaletler: {} Şehir  •  Nüfus: {}\nEkonomi: {}  •  Teknoloji: {}",
                            selected_civ.provinces.len(),
                            format_number(selected_civ.population),
                            format_number(selected_civ.economy as u64),
                            tech_desc
                        )),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.body_sm()),
                            ..default()
                        },
                        TextColor(theme.colors.text_primary),
                        CivInfoField::Stats,
                    ));
                });

                // Kutu 3: Diplomatik Durum (Müttefikler & Rakipler)
                card.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(theme.spacing.val_md()),
                        flex_direction: FlexDirection::Column,
                        row_gap: theme.spacing.val_xs(),
                        border: UiRect::all(Val::Px(theme.borders.thin)),
                        border_radius: BorderRadius::all(theme.radius.val_md()),
                        ..default()
                    },
                    BackgroundColor(theme.colors.surface_panel()),
                    BorderColor::all(theme.colors.border_subtle),
                ))
                .with_children(|b| {
                    b.spawn((
                        Text::new(format!("Müttefikler:\n{}", allies)),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.body_sm()),
                            ..default()
                        },
                        TextColor(theme.colors.success),
                        CivInfoField::Allies,
                    ));
                    b.spawn((
                        Text::new(format!("Rakipler:\n{}", rivals)),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.body_sm()),
                            ..default()
                        },
                        TextColor(theme.colors.danger),
                        CivInfoField::Rivals,
                    ));
                });

                // Kutu 4: Ülke Rengi Özelleştirme Paleti
                card.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: theme.spacing.val_xs(),
                    margin: UiRect::top(theme.spacing.val_xxs()),
                    ..default()
                })
                .with_children(|color_sec| {
                    color_sec.spawn((
                        Text::new("Harita Rengini Özelleştir:"),
                        TextFont {
                            font: fonts.body.clone(),
                            font_size: FontSize::Px(theme.typography.body_sm()),
                            ..default()
                        },
                        TextColor(theme.colors.text_secondary),
                    ));

                    color_sec
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: theme.spacing.val_sm(),
                            ..default()
                        })
                        .with_children(|palette| {
                            let preset_colors: [[u8; 3]; 6] = [
                                [205, 48, 48],  // Kırmızı
                                [38, 162, 154], // Turkuaz
                                [218, 165, 42], // Altın
                                [52, 108, 198], // Mavi
                                [46, 158, 78],  // Yeşil
                                [142, 68, 178], // Mor
                            ];

                            for rgb in preset_colors {
                                palette
                                    .spawn((
                                        Node {
                                            width: Val::Px(24.0),
                                            height: Val::Px(24.0),
                                            border: UiRect::all(Val::Px(theme.borders.thin)),
                                            border_radius: BorderRadius::all(theme.radius.val_sm()),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb_u8(rgb[0], rgb[1], rgb[2])),
                                        BorderColor::all(theme.colors.border_subtle),
                                    ))
                                    .observe(
                                        |ev: On<PointerOver>, mut bq: Query<&mut BorderColor>| {
                                            if let Ok(mut b) = bq.get_mut(ev.entity) {
                                                *b = BorderColor::all(Color::WHITE);
                                            }
                                        },
                                    )
                                    .observe(
                                        {
                                            let border_subtle = theme.colors.border_subtle;
                                            move |ev: On<PointerOut>, mut bq: Query<&mut BorderColor>| {
                                                if let Ok(mut b) = bq.get_mut(ev.entity) {
                                                    *b = BorderColor::all(border_subtle);
                                                }
                                            }
                                        },
                                    )
                                    .observe(
                                        move |_: On<PointerClick>,
                                              session: Res<GameSession>,
                                              map_data: Res<WorldMapData>,
                                              mut scenario: ResMut<ActiveScenario>,
                                              mut images: ResMut<Assets<Image>>| {
                                            let cid = session.player_civ_id;
                                            if let Some(civ) = scenario.civs.get_mut(&cid) {
                                                civ.color = rgb;
                                            }
                                            recolor_country_on_map(&map_data, &scenario, &mut images, cid);
                                        },
                                    );
                            }

                            palette
                                .spawn((
                                    Node {
                                        padding: UiRect::axes(theme.spacing.val_sm(), theme.spacing.val_xxs()),
                                        border: UiRect::all(Val::Px(theme.borders.thin)),
                                        border_radius: BorderRadius::all(theme.radius.val_sm()),
                                        ..default()
                                    },
                                    BackgroundColor(theme.colors.surface_panel()),
                                    BorderColor::all(theme.colors.border_subtle),
                                ))
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
                                        recolor_country_on_map(&map_data, &scenario, &mut images, cid);
                                    },
                                )
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("🎲"),
                                        TextFont {
                                            font: fonts.body.clone(),
                                            font_size: FontSize::Px(theme.typography.body_sm()),
                                            ..default()
                                        },
                                        TextColor(theme.colors.text_primary),
                                        Pickable::IGNORE,
                                    ));
                                });
                        });
                });
            });

            // ----------------------------------------------------------------
            // 2. SAĞ ÜST BAR: OYUN AYARLARI TUŞU & AÇILIR MODAL
            // ----------------------------------------------------------------
            let settings_btn_bg = theme.colors.surface_panel();
            let settings_btn_hover = theme.colors.surface_raised;
            let settings_btn_border = theme.colors.border_subtle;
            let settings_btn_border_hover = theme.colors.border_default;

            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: theme.spacing.val_lg(),
                right: theme.spacing.val_lg(),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: theme.spacing.val_sm(),
                ..default()
            })
            .with_children(|top_right| {
                // Ayarlar Butonu
                top_right
                    .spawn((
                        Node {
                            padding: UiRect::axes(theme.spacing.val_md(), theme.spacing.val_sm()),
                            border: UiRect::all(Val::Px(theme.borders.default)),
                            border_radius: BorderRadius::all(theme.radius.val_md()),
                            align_items: AlignItems::Center,
                            column_gap: theme.spacing.val_sm(),
                            ..default()
                        },
                        BackgroundColor(settings_btn_bg),
                        BorderColor::all(settings_btn_border),
                    ))
                    .observe(
                        move |ev: On<PointerOver>, mut q: Query<(&mut BackgroundColor, &mut BorderColor)>| {
                            if let Ok((mut bg, mut border)) = q.get_mut(ev.entity) {
                                bg.0 = settings_btn_hover;
                                *border = BorderColor::all(settings_btn_border_hover);
                            }
                        },
                    )
                    .observe(
                        move |ev: On<PointerOut>, mut q: Query<(&mut BackgroundColor, &mut BorderColor)>| {
                            if let Ok((mut bg, mut border)) = q.get_mut(ev.entity) {
                                bg.0 = settings_btn_bg;
                                *border = BorderColor::all(settings_btn_border);
                            }
                        },
                    )
                    .observe(
                        |_: On<PointerClick>, mut settings: ResMut<GameSettings>| {
                            settings.show_modal = !settings.show_modal;
                        },
                    )
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("⚙ Oyun Ayarları"),
                            TextFont {
                                font: fonts.heading.clone(),
                                font_size: FontSize::Px(theme.typography.label_md()),
                                ..default()
                            },
                            TextColor(theme.colors.accent),
                            Pickable::IGNORE,
                        ));
                    });

                // Oyun Ayarları Açılır Modalı (Settings Modal)
                top_right
                    .spawn((
                        Node {
                            width: Val::Px(350.0),
                            padding: UiRect::all(theme.spacing.val_lg()),
                            flex_direction: FlexDirection::Column,
                            row_gap: theme.spacing.val_md(),
                            border: UiRect::all(Val::Px(theme.borders.default)),
                            border_radius: BorderRadius::all(theme.radius.val_lg()),
                            ..default()
                        },
                        BackgroundColor(theme.colors.surface_raised),
                        BorderColor::all(theme.colors.border_subtle),
                        Visibility::Hidden,
                        SettingsModalRoot,
                    ))
                    .with_children(|modal| {
                        modal.spawn((
                            Text::new("OYUN AYARLARI"),
                            TextFont {
                                font: fonts.heading.clone(),
                                font_size: FontSize::Px(theme.typography.title_lg()),
                                ..default()
                            },
                            TextColor(theme.colors.accent),
                        ));

                        // 1. Savaş Sisi (Fog of War)
                        modal
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::axes(theme.spacing.val_md(), theme.spacing.val_sm()),
                                    border: UiRect::all(Val::Px(theme.borders.thin)),
                                    border_radius: BorderRadius::all(theme.radius.val_sm()),
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                BackgroundColor(theme.colors.surface_panel()),
                                BorderColor::all(theme.colors.border_subtle),
                            ))
                            .observe(
                                |_: On<PointerClick>, mut settings: ResMut<GameSettings>| {
                                    settings.fog_of_war = !settings.fog_of_war;
                                },
                            )
                            .with_children(|b| {
                                b.spawn((
                                    Text::new("Savaş Sisi: Kapalı"),
                                    TextFont {
                                        font: fonts.medium.clone(),
                                        font_size: FontSize::Px(theme.typography.body_md()),
                                        ..default()
                                    },
                                    TextColor(theme.colors.text_primary),
                                    SettingField::FogOfWar,
                                    Pickable::IGNORE,
                                ));
                            });

                        // 2. Oyun Zorluğu
                        modal
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::axes(theme.spacing.val_md(), theme.spacing.val_sm()),
                                    border: UiRect::all(Val::Px(theme.borders.thin)),
                                    border_radius: BorderRadius::all(theme.radius.val_sm()),
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                BackgroundColor(theme.colors.surface_panel()),
                                BorderColor::all(theme.colors.border_subtle),
                            ))
                            .observe(
                                |_: On<PointerClick>, mut settings: ResMut<GameSettings>| {
                                    settings.difficulty = match settings.difficulty.as_str() {
                                        "Kolay" => "Normal".to_string(),
                                        "Normal" => "Zor".to_string(),
                                        "Zor" => "Uzman".to_string(),
                                        _ => "Kolay".to_string(),
                                    };
                                },
                            )
                            .with_children(|b| {
                                b.spawn((
                                    Text::new("Zorluk: Normal"),
                                    TextFont {
                                        font: fonts.medium.clone(),
                                        font_size: FontSize::Px(theme.typography.body_md()),
                                        ..default()
                                    },
                                    TextColor(theme.colors.text_primary),
                                    SettingField::Difficulty,
                                    Pickable::IGNORE,
                                ));
                            });

                        // 3. Yapay Zeka Agresifliği
                        modal
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::axes(theme.spacing.val_md(), theme.spacing.val_sm()),
                                    border: UiRect::all(Val::Px(theme.borders.thin)),
                                    border_radius: BorderRadius::all(theme.radius.val_sm()),
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                BackgroundColor(theme.colors.surface_panel()),
                                BorderColor::all(theme.colors.border_subtle),
                            ))
                            .observe(
                                |_: On<PointerClick>, mut settings: ResMut<GameSettings>| {
                                    settings.ai_aggressiveness = match settings.ai_aggressiveness.as_str() {
                                        "Standart" => "Agresif".to_string(),
                                        "Agresif" => "Pasif".to_string(),
                                        _ => "Standart".to_string(),
                                    };
                                },
                            )
                            .with_children(|b| {
                                b.spawn((
                                    Text::new("Yapay Zeka: Standart"),
                                    TextFont {
                                        font: fonts.medium.clone(),
                                        font_size: FontSize::Px(theme.typography.body_md()),
                                        ..default()
                                    },
                                    TextColor(theme.colors.text_primary),
                                    SettingField::AiAggressiveness,
                                    Pickable::IGNORE,
                                ));
                            });

                        // 4. Demir Adam (Ironman)
                        modal
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::axes(theme.spacing.val_md(), theme.spacing.val_sm()),
                                    border: UiRect::all(Val::Px(theme.borders.thin)),
                                    border_radius: BorderRadius::all(theme.radius.val_sm()),
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                BackgroundColor(theme.colors.surface_panel()),
                                BorderColor::all(theme.colors.border_subtle),
                            ))
                            .observe(
                                |_: On<PointerClick>, mut settings: ResMut<GameSettings>| {
                                    settings.ironman = !settings.ironman;
                                },
                            )
                            .with_children(|b| {
                                b.spawn((
                                    Text::new("Demir Adam (Ironman): Kapalı"),
                                    TextFont {
                                        font: fonts.medium.clone(),
                                        font_size: FontSize::Px(theme.typography.body_md()),
                                        ..default()
                                    },
                                    TextColor(theme.colors.text_primary),
                                    SettingField::Ironman,
                                    Pickable::IGNORE,
                                ));
                            });

                        // Kapat Butonu
                        modal
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::vertical(theme.spacing.val_sm()),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border_radius: BorderRadius::all(theme.radius.val_md()),
                                    margin: UiRect::top(theme.spacing.val_xs()),
                                    ..default()
                                },
                                BackgroundColor(theme.colors.accent),
                            ))
                            .observe(
                                |_: On<PointerClick>, mut settings: ResMut<GameSettings>| {
                                    settings.show_modal = false;
                                },
                            )
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("✓ Ayarları Kaydet / Kapat"),
                                    TextFont {
                                        font: fonts.heading.clone(),
                                        font_size: FontSize::Px(theme.typography.label_md()),
                                        ..default()
                                    },
                                    TextColor(theme.colors.text_primary),
                                    Pickable::IGNORE,
                                ));
                            });
                    });
            });

            // ----------------------------------------------------------------
            // 3. ALT BAR: GERİ TUŞU (SOL), ÖNERİLEN ÜLKELER (ORTA), OYNA (SAĞ)
            // ----------------------------------------------------------------
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: theme.spacing.val_md(),
                    left: theme.spacing.val_lg(),
                    right: theme.spacing.val_lg(),
                    height: Val::Px(64.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(theme.spacing.val_md(), theme.spacing.val_sm()),
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.08, 0.11)),
                BorderColor::all(Color::srgb(0.78, 0.62, 0.28)),
            ))
            .with_children(|bar| {
                // Sol Alt: Geri Tuşu
                let mut back_btn = spawn_button(
                    bar,
                    &theme,
                    &fonts,
                    "← Senaryolara Dön",
                    ButtonVariant::Secondary,
                    ButtonSize::Small,
                );
                back_btn.observe(
                    |_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::ScenarioCatalog);
                    },
                );

                // Orta: Önerilen / En Güçlü Ülkeler (Hızlı Seçim Çubuğu)
                bar.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: theme.spacing.val_sm(),
                    ..default()
                })
                .with_children(|center| {
                    center.spawn((
                        Text::new("Önerilen Güçler:"),
                        TextFont {
                            font: fonts.medium.clone(),
                            font_size: FontSize::Px(theme.typography.label_md()),
                            ..default()
                        },
                        TextColor(theme.colors.text_secondary),
                    ));

                    // Süper güçler listesi: Türkiye, ABD, Rusya, Çin, Almanya, İngiltere, Fransa, Azerbaycan
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

                    let pill_bg = theme.colors.surface_panel();
                    let pill_hover = theme.colors.surface_raised;
                    let pill_border = theme.colors.border_subtle;
                    let pill_border_hover = theme.colors.border_default;

                    for (tag, label) in quick_tags {
                        if let Some(civ) = scenario.civs.values().find(|c| c.tag == tag || c.tag.starts_with(tag)) {
                            let cid = civ.id;
                            let flag_p = ActiveScenario::get_flag_path(&civ.tag, None);

                            center
                                .spawn((
                                    Node {
                                        padding: UiRect::axes(theme.spacing.val_sm(), theme.spacing.val_xs()),
                                        border: UiRect::all(Val::Px(theme.borders.thin)),
                                        border_radius: BorderRadius::all(theme.radius.val_sm()),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        column_gap: theme.spacing.val_xs(),
                                        ..default()
                                    },
                                    BackgroundColor(pill_bg),
                                    BorderColor::all(pill_border),
                                ))
                                .observe(
                                    move |ev: On<PointerOver>, mut q: Query<(&mut BackgroundColor, &mut BorderColor)>| {
                                        if let Ok((mut bg, mut border)) = q.get_mut(ev.entity) {
                                            bg.0 = pill_hover;
                                            *border = BorderColor::all(pill_border_hover);
                                        }
                                    },
                                )
                                .observe(
                                    move |ev: On<PointerOut>, mut q: Query<(&mut BackgroundColor, &mut BorderColor)>| {
                                        if let Ok((mut bg, mut border)) = q.get_mut(ev.entity) {
                                            bg.0 = pill_bg;
                                            *border = BorderColor::all(pill_border);
                                        }
                                    },
                                )
                                .observe(
                                    move |_: On<PointerClick>,
                                          mut session: ResMut<GameSession>,
                                          scenario: Res<ActiveScenario>,
                                          map_data: Res<WorldMapData>,
                                          mut rig: ResMut<CameraRig>| {
                                        session.player_civ_id = cid;
                                        if let Some(c) = scenario.civs.get(&cid) {
                                            if let Some(cap_prov) = map_data.provinces.get(&c.capital_province_id) {
                                                rig.target_pos = map_pixel_to_world(cap_prov.city_px[0], cap_prov.city_px[1]);
                                                rig.target_scale = 0.65;
                                            }
                                        }
                                    },
                                )
                                .with_children(|pill| {
                                    pill.spawn((
                                        ImageNode {
                                            image: asset_server.load(flag_p),
                                            ..default()
                                        },
                                        Node {
                                            width: Val::Px(16.0),
                                            height: Val::Px(11.0),
                                            border_radius: BorderRadius::all(theme.radius.val_xs()),
                                            ..default()
                                        },
                                        Pickable::IGNORE,
                                    ));
                                    pill.spawn((
                                        Text::new(label),
                                        TextFont {
                                            font: fonts.heading.clone(),
                                            font_size: FontSize::Px(theme.typography.label_sm()),
                                            ..default()
                                        },
                                        TextColor(theme.colors.text_primary),
                                        Pickable::IGNORE,
                                    ));
                                });
                        }
                    }
                });

                // Sağ Alt: OYNA Butonu (Solid Primary Button - Dark text on glowing gold)
                let mut oyna_btn = spawn_button(
                    bar,
                    &theme,
                    &fonts,
                    "OYNA  ▶",
                    ButtonVariant::Primary,
                    ButtonSize::Large,
                );
                oyna_btn.observe(
                    |_: On<PointerClick>,
                     session: Res<GameSession>,
                     scenario: Res<ActiveScenario>,
                     map_data: Res<WorldMapData>,
                     mut rig: ResMut<CameraRig>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        if let Some(civ) = scenario.civs.get(&session.player_civ_id) {
                            if let Some(cap_prov) = map_data.provinces.get(&civ.capital_province_id) {
                                rig.target_pos = map_pixel_to_world(cap_prov.city_px[0], cap_prov.city_px[1]);
                                rig.target_scale = 0.55;
                            }
                        }
                        next_state.set(AppState::InGame);
                    },
                );
            });
        });
}

fn sync_selected_civ_ui(
    session: Res<GameSession>,
    scenario: Res<ActiveScenario>,
    asset_server: Res<AssetServer>,
    mut text_fields_q: Query<(&CivInfoField, &mut Text)>,
    mut badge_q: Query<&mut BackgroundColor, With<SelectedCivColorBadge>>,
    mut flag_q: Query<&mut ImageNode, With<SelectedCivFlagNode>>,
) {
    if !session.is_changed() && !scenario.is_changed() {
        return;
    }
    if let Some(civ) = scenario.civs.get(&session.player_civ_id) {
        let (leader_name, leader_title) = get_civ_leader_info(&civ.tag);
        let gov_type = get_civ_government(&civ.tag);
        let religion = get_civ_religion(&civ.tag);
        let tech_desc = get_civ_technology(&civ.tag, civ.economy);
        let (allies, rivals) = get_civ_diplomacy(&civ.tag);

        for (field, mut text) in &mut text_fields_q {
            match field {
                CivInfoField::Name => text.0 = civ.name.clone(),
                CivInfoField::Capital => text.0 = format!("Başkent: {}", civ.capital_name),
                CivInfoField::Leader => text.0 = format!("Lider: {} ({})", leader_name, leader_title),
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

fn sync_settings_modal_ui(
    settings: Res<GameSettings>,
    mut modal_q: Query<&mut Visibility, With<SettingsModalRoot>>,
    mut setting_texts_q: Query<(&SettingField, &mut Text)>,
) {
    if !settings.is_changed() {
        return;
    }

    for mut vis in &mut modal_q {
        let target = if settings.show_modal {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *vis != target {
            *vis = target;
        }
    }

    for (field, mut text) in &mut setting_texts_q {
        match field {
            SettingField::FogOfWar => {
                text.0 = format!("Savaş Sisi: {}", if settings.fog_of_war { "AÇIK" } else { "KAPALI" });
            }
            SettingField::Difficulty => {
                text.0 = format!("Zorluk: {}", settings.difficulty);
            }
            SettingField::AiAggressiveness => {
                text.0 = format!("Yapay Zeka: {}", settings.ai_aggressiveness);
            }
            SettingField::Ironman => {
                text.0 = format!("Demir Adam (Ironman): {}", if settings.ironman { "AÇIK" } else { "KAPALI" });
            }
        }
    }
}

fn cleanup_scenario_select(mut commands: Commands, q: Query<Entity, With<ScenarioSelectRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

// ============================================================================
// 4. DETAYLI ÜLKE BİLGİ VE JEOFİZİK VERİLERİ (Dossier Helpers)
// ============================================================================

pub fn get_civ_leader_info(tag: &str) -> (&'static str, &'static str) {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "tur" => ("Recep Tayyip Erdoğan", "Cumhurbaşkanı"),
        "usa" => ("Joe Biden", "Başkan"),
        "rus" => ("Vladimir Putin", "Devlet Başkanı"),
        "chi" | "chn" => ("Xi Jinping", "Devlet Başkanı"),
        "ger" | "deu" => ("Olaf Scholz", "Federal Şansölye"),
        "uni" | "gbr" => ("Keir Starmer", "Başbakan"),
        "fra" => ("Emmanuel Macron", "Cumhurbaşkanı"),
        "ita" => ("Giorgia Meloni", "Başbakan"),
        "jap" => ("Shigeru Ishiba", "Başbakan"),
        "ind" => ("Narendra Modi", "Başbakan"),
        "aze" => ("İlham Aliyev", "Cumhurbaşkanı"),
        "ukr" => ("Volodimir Zelenski", "Devlet Başkanı"),
        "irn" => ("Mesud Pezeşkiyan", "Cumhurbaşkanı"),
        "sau" => ("Muhammed bin Selman", "Veliaht Prens & Başbakan"),
        "egy" => ("Abdülfettah es-Sisi", "Cumhurbaşkanı"),
        "bra" => ("Luiz Inácio Lula da Silva", "Devlet Başkanı"),
        "can" => ("Justin Trudeau", "Başbakan"),
        "aus" => ("Anthony Albanese", "Başbakan"),
        "spa" | "esp" => ("Pedro Sánchez", "Başbakan"),
        "sok" => ("Yoon Suk Yeol", "Devlet Başkanı"),
        "prk" => ("Kim Jong-un", "Yüce Lider"),
        "isr" => ("Binyamin Netanyahu", "Başbakan"),
        "pal" => ("Mahmud Abbas", "Devlet Başkanı"),
        "gre" => ("Kiryakos Miçotakis", "Başbakan"),
        "pol" => ("Donald Tusk", "Başbakan"),
        "kaz" => ("Kasım Cömert Tokayev", "Cumhurbaşkanı"),
        "uzb" => ("Şevket Mirziyoyev", "Cumhurbaşkanı"),
        "pak" => ("Şahbaz Şerif", "Başbakan"),
        "alg" => ("Abdülmecid Tebbun", "Cumhurbaşkanı"),
        "mor" => ("VI. Muhammed", "Kral"),
        "syr" => ("Beşar Esad", "Devlet Başkanı"),
        "irq" => ("Muhammed Şiya es-Sudani", "Başbakan"),
        "cyn" => ("Ersin Tatar", "Cumhurbaşkanı"),
        "cpr" => ("Nikos Hristodulidis", "Cumhurbaşkanı"),
        "mex" => ("Claudia Sheinbaum", "Devlet Başkanı"),
        "arg" => ("Javier Milei", "Devlet Başkanı"),
        "swe" => ("Ulf Kristersson", "Başbakan"),
        "nor" => ("Jonas Gahr Støre", "Başbakan"),
        "fin" => ("Alexander Stubb", "Cumhurbaşkanı"),
        "nld" => ("Dick Schoof", "Başbakan"),
        "bel" => ("Alexander De Croo", "Başbakan"),
        _ => ("Devlet Başkanı", "Hükümet Lideri"),
    }
}

pub fn get_civ_government(tag: &str) -> &'static str {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "tur" => "Cumhurbaşkanlığı Hükümet Sistemi",
        "usa" => "Federal Başkanlık Cumhuriyeti",
        "rus" => "Federal Yarı Başkanlık Cumhuriyeti",
        "chi" | "chn" => "Tek Partili Sosyalist Cumhuriyet",
        "uni" | "gbr" | "jap" | "spa" | "esp" | "nld" | "bel" | "swe" | "nor" => {
            "Meşruti Monarşi (Demokrasi)"
        }
        "sau" | "uae" | "oma" | "qat" | "kwt" | "bhr" => "Monarşi / Emirlik",
        "irn" => "İslam Cumhuriyeti",
        "prk" => "Tek Partili Sosyalist Diktatörlük",
        "ger" | "deu" | "ita" | "ind" | "isr" => "Federal / Parlamenter Cumhuriyet",
        "cyn" => "Yarı Başkanlık Cumhuriyeti",
        _ => "Üniter Cumhuriyet",
    }
}

pub fn get_civ_religion(tag: &str) -> &'static str {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "tur" | "aze" | "sau" | "egy" | "alg" | "mor" | "pak" | "tun" | "lby" | "jor"
        | "syr" | "irq" | "cyn" => "İslam (Sünni / Çoğulcu)",
        "irn" => "İslam (Caferi Şiilik)",
        "isr" => "Musevilik (Yahudilik)",
        "rus" | "gre" | "geo" | "srb" | "bul" | "rom" | "ukr" | "cpr" | "blr" => {
            "Hristiyanlık (Ortodoks)"
        }
        "ita" | "spa" | "esp" | "fra" | "pol" | "bra" | "mex" | "arg" | "prt" | "por"
        | "irl" | "aut" | "hrv" => "Hristiyanlık (Katolik)",
        "usa" | "uni" | "gbr" | "ger" | "deu" | "swe" | "nor" | "fin" | "dnk" | "can"
        | "aus" | "nzl" => "Hristiyanlık (Protestan / Çoğulcu)",
        "chi" | "chn" | "jap" | "vie" | "tha" | "mmr" | "sok" => "Budizm / Şintoizm & Seküler",
        "ind" => "Hinduizm (%80) & İslam (%14)",
        _ => "Seküler / Çoğulcu",
    }
}

pub fn get_civ_technology(tag: &str, economy: u32) -> &'static str {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "usa" | "jap" | "ger" | "deu" => "Seviye 2.45 (Küresel İleri Teknoloji & Yapay Zeka)",
        "chi" | "chn" | "uni" | "gbr" | "fra" | "sok" => "Seviye 2.30 (Yüksek Teknoloji & Nükleer)",
        "tur" | "rus" | "ita" | "isr" | "can" | "swe" => "Seviye 2.10 (Modern Sanayi & Savunma)",
        "ind" | "bra" | "spa" | "esp" | "aus" | "sau" | "pol" => "Seviye 1.90 (Gelişmiş Ağır Sanayi)",
        _ => {
            if economy > 1000 {
                "Seviye 1.65 (Gelişmekte Olan Sanayi)"
            } else {
                "Seviye 1.30 (Standart Sanayi)"
            }
        }
    }
}

pub fn get_civ_diplomacy(tag: &str) -> (&'static str, &'static str) {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "tur" => (
            "Azerbaycan, KKTC, Pakistan, Katar",
            "Yunanistan, GKRY, Suriye",
        ),
        "usa" => (
            "NATO, Birleşik Krallık, İsrail, Japonya, G. Kore",
            "Rusya, Çin, İran, K. Kore",
        ),
        "rus" => (
            "Belarus, Kazakistan, Çin, İran, K. Kore",
            "ABD, NATO, Ukrayna",
        ),
        "chi" | "chn" => (
            "Rusya, Pakistan, K. Kore, Kamboçya",
            "ABD, Tayvan, Hindistan, Japonya",
        ),
        "ger" | "deu" => ("Fransa, AB Müttefikleri, NATO, ABD", "Rusya, K. Kore"),
        "uni" | "gbr" => (
            "ABD, İngiliz Milletler Topluluğu, NATO",
            "Rusya, Çin, İran",
        ),
        "fra" => ("Almanya, AB Müttefikleri, NATO, ABD", "Rusya"),
        "aze" => ("Türkiye, İsrail, Pakistan", "Ermenistan, İran"),
        "arm" => ("Yunanistan, Fransa, Rusya", "Azerbaycan, Türkiye"),
        "isr" => (
            "ABD, İngiltere, Almanya",
            "İran, Filistin, Suriye, Lübnan",
        ),
        "pal" => ("Arap Birliği, Türkiye, İran", "İsrail, ABD"),
        "gre" => ("GKRY, Fransa, ABD, İsrail", "Türkiye, KKTC"),
        "cyn" => ("Türkiye", "GKRY, Yunanistan"),
        "cpr" => ("Yunanistan, Fransa, İsrail", "KKTC, Türkiye"),
        "ukr" => ("NATO, ABD, Polonya, İngiltere", "Rusya, Belarus"),
        "irn" => (
            "Suriye, Rusya, Çin, Husiler",
            "ABD, İsrail, Suudi Arabistan",
        ),
        "sau" => ("B.A.E., Bahreyn, ABD, Mısır", "İran, Yemen (Husiler)"),
        "ind" => ("Rusya, ABD, Fransa, Japonya", "Pakistan, Çin"),
        "pak" => ("Çin, Türkiye, Suudi Arabistan", "Hindistan"),
        _ => ("Bölgesel Komşular, BM Ortakları", "Jeopolitik Rakipler"),
    }
}
