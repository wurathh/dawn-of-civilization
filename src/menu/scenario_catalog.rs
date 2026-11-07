use crate::map::CameraRig;
use crate::menu::common::{
    apply_panel_opacity, pick_random_background_image, ActiveMenuBackground, UiFadeAlphaTracker,
};
use crate::scenario::ActiveScenario;
use crate::state::{AppState, PreviousMenuState};
use crate::ui::{
    divider, frame, headed_panel, text, ui_button, BIG_BUTTON_BORDER, BODY_TEXT, BUTTON_BORDER,
    DIM_TEXT, GOLD_TEXT, HEADER_BORDER, INSET_BORDER, Skins, Ui,
};
use bevy::picking::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScenarioCatalogRoot;

#[derive(Component)]
pub struct ScenarioCatalogAnim {
    pub timer: f32,
    pub duration: f32,
    pub fading_out: bool,
    pub fade_whole_page: bool,
    pub next_target: Option<AppState>,
}

pub fn setup_scenario_catalog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui: Res<Ui>,
    skins: Res<Skins>,
    scenario: Res<ActiveScenario>,
    mut active_bg: ResMut<ActiveMenuBackground>,
) {
    let bg_handle = if let Some(ref handle) = active_bg.0 {
        handle.clone()
    } else {
        let handle = if let Some(bg_path) = pick_random_background_image() {
            asset_server.load(bg_path)
        } else {
            ui.menu_bg.clone()
        };
        active_bg.0 = Some(handle.clone());
        handle
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                ..default()
            },
            ScenarioCatalogRoot,
        ))
        .with_children(|root| {
            // 1. Arka Plan & Optimize Vinyet Katmanı
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .with_children(|bg_container| {
                bg_container.spawn((
                    ImageNode {
                        image: bg_handle,
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

                bg_container.spawn((
                    ImageNode {
                        image: ui.vignette.clone(),
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
            });

            // 2. Katalog Sayfa İçeriği
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ScenarioCatalogAnim {
                    timer: 0.0,
                    duration: 0.18,
                    fading_out: false,
                    fade_whole_page: false,
                    next_target: None,
                },
            ))
            .with_children(|page| {
                // 3. Üst Bar (`topbar.png`)
                page.spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(54.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(Val::Px(22.0)),
                    position_type: PositionType::Relative,
                    ..default()
                })
                .with_children(|topbar| {
                    frame(topbar, &ui.topbar, HEADER_BORDER);
                    text(topbar, &ui, "SENARYO KATALOĞU", 20.0, GOLD_TEXT);
                    text(
                        topbar,
                        &ui,
                        "Yönetmek istediğiniz çağı ve jeopolitik senaryoyu belirleyin",
                        14.5,
                        DIM_TEXT,
                    );
                });

                // 4. Orta: EU4 Headed Panel Senaryo Kartı
                headed_panel(
                    page,
                    &ui,
                    &ui.headed,
                    "MODERN DÜNYA (2026 ÇAĞI)",
                    (Val::Px(640.0), Val::Px(380.0)),
                    |card| {
                        text(
                            card,
                            &ui,
                            "Günümüzün jeopolitik güç dengeleri, nükleer caydırıcılık, küresel ticaret ağları ve bölgesel gerilimler. Dünyanın kaderini belirleyecek lider olun.",
                            16.0,
                            BODY_TEXT,
                        );

                        divider(card, &ui);

                        // 3 adet Inset istatistik kutusu
                        card.spawn(Node {
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            column_gap: Val::Px(12.0),
                            ..default()
                        })
                        .with_children(|stats| {
                            let items = [
                                (
                                    format!("{} Ülke", scenario.civs.len()),
                                    "Bağımsız Egemen Devletler",
                                ),
                                ("6.133 Şehir".to_string(), "Gerçek Eyalet Sınırları"),
                                ("29 Ekim 2026".to_string(), "Başlangıç Tarihi"),
                            ];
                            for (val, desc) in items {
                                stats
                                    .spawn(Node {
                                        flex_grow: 1.0,
                                        height: Val::Px(78.0),
                                        padding: UiRect::all(Val::Px(12.0)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        row_gap: Val::Px(4.0),
                                        position_type: PositionType::Relative,
                                        ..default()
                                    })
                                    .with_children(|box_col| {
                                        frame(box_col, &ui.inset, INSET_BORDER);
                                        text(box_col, &ui, &val, 18.0, GOLD_TEXT);
                                        text(box_col, &ui, desc, 13.0, DIM_TEXT);
                                    });
                            }
                        });

                        divider(card, &ui);

                        let mut start_btn = ui_button(
                            card,
                            &ui,
                            &skins.big_primary,
                            BIG_BUTTON_BORDER,
                            "Senaryoyu Seç & Haritaya Geç",
                            Vec2::new(320.0, 48.0),
                            Some("Harita üzerinde ülke seçimine geç"),
                        );
                        start_btn.observe(
                            |_: On<PointerClick>,
                             mut rig: ResMut<CameraRig>,
                             mut prev: ResMut<PreviousMenuState>,
                             mut anim_q: Query<&mut ScenarioCatalogAnim>| {
                                rig.target_pos = Vec2::new(850.0, 580.0);
                                rig.target_scale = 0.85;
                                prev.0 = AppState::ScenarioCatalog;
                                for mut anim in &mut anim_q {
                                    if !anim.fading_out {
                                        anim.fading_out = true;
                                        anim.fade_whole_page = true;
                                        anim.timer = 0.0;
                                        anim.duration = 0.22;
                                        anim.next_target = Some(AppState::ScenarioSelect);
                                    }
                                }
                            },
                        );
                    },
                );

                // 5. Alt Bar: Ana Menüye Dön
                page.spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                })
                .with_children(|footer| {
                    let mut back_btn = ui_button(
                        footer,
                        &ui,
                        &skins.secondary,
                        BUTTON_BORDER,
                        "Ana Menüye Dön",
                        Vec2::new(170.0, 38.0),
                        None,
                    );
                    back_btn.observe(
                        |_: On<PointerClick>,
                         mut prev: ResMut<PreviousMenuState>,
                         mut anim_q: Query<&mut ScenarioCatalogAnim>| {
                            prev.0 = AppState::ScenarioCatalog;
                            for mut anim in &mut anim_q {
                                if !anim.fading_out {
                                    anim.fading_out = true;
                                    anim.fade_whole_page = false;
                                    anim.timer = 0.0;
                                    anim.duration = 0.16;
                                    anim.next_target = Some(AppState::MainMenu);
                                }
                            }
                        },
                    );
                });
            });
        });
}

pub fn update_scenario_catalog_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    root_q: Query<Entity, With<ScenarioCatalogRoot>>,
    mut anim_q: Query<(Entity, &mut ScenarioCatalogAnim)>,
    children_q: Query<&Children>,
    fade_q: Query<&UiFadeAlphaTracker>,
    mut img_q: Query<&mut ImageNode>,
    mut text_q: Query<&mut TextColor>,
    mut bg_q: Query<&mut BackgroundColor>,
    mut border_q: Query<&mut BorderColor>,
) {
    let catalog_root_opt = root_q.iter().next();

    for (panel_root, mut anim) in &mut anim_q {
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

        let target_entity = if anim.fade_whole_page {
            catalog_root_opt.unwrap_or(panel_root)
        } else {
            panel_root
        };

        apply_panel_opacity(
            target_entity,
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

pub fn cleanup_scenario_catalog(
    mut commands: Commands,
    q: Query<Entity, With<ScenarioCatalogRoot>>,
) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
