use crate::menu::common::{
    apply_panel_opacity, pick_random_background_image, ActiveMenuBackground, UiFadeAlphaTracker,
};
use crate::state::{AppState, GameSession, PreviousMenuState};
use crate::ui::{
    divider, headed_2x_panel_custom, text, ui_button, BIG_BUTTON_BORDER, BUTTON_BORDER, DIM_TEXT,
    GOLD_TEXT, HEAD_SIDE, Skins, Ui,
};
use bevy::app::AppExit;
use bevy::picking::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct MainMenuBgContainer;

#[derive(Component)]
pub struct MainMenuAnim {
    pub timer: f32,
    pub duration: f32,
    pub fading_out: bool,
    pub fade_whole_page: bool,
    pub next_target: Option<AppState>,
}

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui: Res<Ui>,
    skins: Res<Skins>,
    prev: Res<PreviousMenuState>,
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

    // Harita ekranından (ScenarioSelect / InGame) ana menüye dönülüyorsa tüm sayfa (arka plan dahil) 0 -> 100 opaklıkla açılır
    let fade_whole_page = matches!(prev.0, AppState::ScenarioSelect | AppState::InGame);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|root| {
            // 1. Arka Plan & Optimize Vinyet Katmanı
            // (Shader derleme gecikmesi olmayan, GPU doku sampleri ile arka planla tam senkronize tek konteyner)
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
                MainMenuBgContainer,
            ))
            .with_children(|bg_container| {
                // A) Arka Plan Görseli
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

                // B) Optimize Statik Vinyet & Sinematik Karartma (Önceden bellekte, anında ve arka planla tam aynı anda çizilir)
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

            // 2. Ana Menü Sayfa İçeriği (Header içi iki satırlı başlık + ideal boşluklu panel + sürüm bilgisi)
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                MainMenuAnim {
                    timer: 0.0,
                    duration: if fade_whole_page { 0.22 } else { 0.18 },
                    fading_out: false,
                    fade_whole_page,
                    next_target: None,
                },
            ))
            .with_children(|page| {
                // EU4 v3 Headed Panel: Yalnızca ana menüde 2x büyük (80px) altın başlık bandı
                headed_2x_panel_custom(
                    page,
                    &ui,
                    (Val::Px(360.0), Val::Px(424.0)),
                    UiRect::new(
                        Val::Px(HEAD_SIDE),
                        Val::Px(HEAD_SIDE),
                        Val::Px(16.0),
                        Val::Px(24.0),
                    ),
                    Val::Px(13.0),
                    |h| {
                        h.spawn((
                            Text::new("DAWN OF"),
                            TextFont {
                                font: ui.font.clone().into(),
                                font_size: FontSize::Px(19.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.88, 0.78, 0.52)),
                        ));
                        h.spawn((
                            Text::new("CIVILIZATION"),
                            TextFont {
                                font: ui.font.clone().into(),
                                font_size: FontSize::Px(29.0),
                                ..default()
                            },
                            TextColor(GOLD_TEXT),
                        ));
                    },
                    |p| {
                        let big = Vec2::new(260.0, 46.0);

                        // 1) Devam Et Butonu (Oyun başlatılmışsa aktif hale gelir)
                        let can_continue = prev.0 == AppState::InGame;
                        let mut continue_btn = ui_button(
                            p,
                            &ui,
                            if can_continue { &skins.big_primary } else { &skins.big_secondary },
                            BIG_BUTTON_BORDER,
                            "Devam Et",
                            big,
                            Some(if can_continue {
                                "Devam eden oyuna geri dön"
                            } else {
                                "Kayıtlı son oyundan devam et (Yakında)"
                            }),
                        );
                        if !can_continue {
                            continue_btn.insert(crate::ui::UiButton {
                                variant: crate::ui::ButtonVariant::Secondary,
                                size: crate::ui::ButtonSize::Large,
                                style: crate::ui::ButtonStyle::Solid,
                                disabled: true,
                                focused: false,
                            });
                        } else {
                            continue_btn.observe(
                                |_: On<PointerClick>,
                                 mut prev: ResMut<PreviousMenuState>,
                                 mut anim_q: Query<&mut MainMenuAnim>| {
                                    prev.0 = AppState::MainMenu;
                                    for mut anim in &mut anim_q {
                                        if !anim.fading_out {
                                            anim.fading_out = true;
                                            anim.fade_whole_page = true;
                                            anim.timer = 0.0;
                                            anim.duration = 0.18;
                                            anim.next_target = Some(AppState::InGame);
                                        }
                                    }
                                },
                            );
                        }

                        // 2) Yeni Oyun Başlat (Arka plandaki harita üzerine tüm menü 100->0 şeffaflaşır)
                        let mut new_game_btn = ui_button(
                            p,
                            &ui,
                            &skins.big_primary,
                            BIG_BUTTON_BORDER,
                            "Yeni Oyun Başlat",
                            big,
                            Some("Senaryo ve ülke seçim ekranına geç"),
                        );
                        new_game_btn.observe(
                            |_: On<PointerClick>,
                             session: Res<GameSession>,
                             scenario: Res<crate::scenario::ActiveScenario>,
                             map_data: Res<crate::map::WorldMapData>,
                             mut rig: ResMut<crate::map::CameraRig>,
                             mut prev: ResMut<PreviousMenuState>,
                             mut anim_q: Query<&mut MainMenuAnim>| {
                                if let Some(civ) = scenario.civs.get(&session.player_civ_id) {
                                    let (target_pos, target_scale) =
                                        crate::map::calculate_civ_camera_target(civ, &map_data);
                                    rig.target_pos = target_pos;
                                    rig.target_scale = target_scale;
                                }
                                prev.0 = AppState::MainMenu;
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

                        // 3) Senaryo Kataloğu
                        let mut catalog_btn = ui_button(
                            p,
                            &ui,
                            &skins.big_secondary,
                            BIG_BUTTON_BORDER,
                            "Senaryo Kataloğu",
                            big,
                            Some("Tarihî senaryo özetlerini incele"),
                        );
                        catalog_btn.observe(
                            |_: On<PointerClick>,
                             mut prev: ResMut<PreviousMenuState>,
                             mut anim_q: Query<&mut MainMenuAnim>| {
                                prev.0 = AppState::MainMenu;
                                for mut anim in &mut anim_q {
                                    if !anim.fading_out {
                                        anim.fading_out = true;
                                        anim.fade_whole_page = false;
                                        anim.timer = 0.0;
                                        anim.duration = 0.16;
                                        anim.next_target = Some(AppState::ScenarioCatalog);
                                    }
                                }
                            },
                        );

                        // 4) Oyun Ayarları (Önce sayfa 100->0 kaybolur, ardından ayarlar 0->100 açılır)
                        let mut settings_btn = ui_button(
                            p,
                            &ui,
                            &skins.big_secondary,
                            BIG_BUTTON_BORDER,
                            "Oyun Ayarları",
                            big,
                            Some("Ses, grafik ve oyun kurallarını yapılandır"),
                        );
                        settings_btn.observe(
                            |_: On<PointerClick>,
                             mut prev: ResMut<PreviousMenuState>,
                             mut anim_q: Query<&mut MainMenuAnim>| {
                                prev.0 = AppState::MainMenu;
                                for mut anim in &mut anim_q {
                                    if !anim.fading_out {
                                        anim.fading_out = true;
                                        anim.fade_whole_page = false;
                                        anim.timer = 0.0;
                                        anim.duration = 0.16;
                                        anim.next_target = Some(AppState::Settings);
                                    }
                                }
                            },
                        );

                        divider(p, &ui);

                        // 5) Oyundan Çık (Alt kenardan rahat nefes payı bırakıldı)
                        let mut exit_btn = ui_button(
                            p,
                            &ui,
                            &skins.danger,
                            BUTTON_BORDER,
                            "Oyundan Çık",
                            Vec2::new(160.0, 36.0),
                            Some("Masaüstüne dön"),
                        );
                        exit_btn.observe(
                            |_: On<PointerClick>, mut exit_writer: MessageWriter<AppExit>| {
                                exit_writer.write(AppExit::Success);
                            },
                        );
                    },
                );

                // C) Ekranın En Alt Sol Köşesinde Sürüm ve 8 Haneli Hex Build Kodu (Sayfayla birlikte şeffaflaşır)
                page.spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(16.0),
                    bottom: Val::Px(12.0),
                    ..default()
                })
                .with_children(|b| {
                    text(b, &ui, "v0.1.0-alpha (build 7e3f9a2b)", 13.0, DIM_TEXT);
                });
            });
        });
}

pub fn cleanup_main_menu(mut commands: Commands, q: Query<Entity, With<MainMenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

pub fn update_main_menu_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    root_q: Query<Entity, With<MainMenuRoot>>,
    bg_container_q: Query<Entity, With<MainMenuBgContainer>>,
    mut anim_q: Query<(Entity, &mut MainMenuAnim)>,
    children_q: Query<&Children>,
    fade_q: Query<&UiFadeAlphaTracker>,
    mut img_q: Query<&mut ImageNode>,
    mut text_q: Query<&mut TextColor>,
    mut bg_q: Query<&mut BackgroundColor>,
    mut border_q: Query<&mut BorderColor>,
) {
    let menu_root_opt = root_q.iter().next();
    let bg_root_opt = bg_container_q.iter().next();

    for (panel_root, mut anim) in &mut anim_q {
        let opacity = if !anim.fading_out {
            // Açılış Fade-in: Gerçek şeffaflık 0.0 -> 1.0 yükselir
            anim.timer += time.delta_secs();
            let progress = (anim.timer / anim.duration).clamp(0.0, 1.0);
            1.0 - (1.0 - progress).powi(3)
        } else {
            // Kapanış Fade-out: Gerçek şeffaflık 1.0 -> 0.0 düşer
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
            menu_root_opt.unwrap_or(panel_root)
        } else {
            if let Some(bg_root) = bg_root_opt {
                apply_panel_opacity(
                    bg_root,
                    1.0,
                    &mut commands,
                    &children_q,
                    &fade_q,
                    &mut img_q,
                    &mut text_q,
                    &mut bg_q,
                    &mut border_q,
                );
            }
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
