#![allow(deprecated)]

use crate::menu::common::{
    apply_panel_opacity, pick_random_background_image, ActiveMenuBackground, UiFadeAlphaTracker,
};
use crate::state::{AppState, GameSettings, PreviousMenuState};
use crate::ui::{
    divider, frame, headed_panel, sliced, spawn_slider_interactive, text, ui_button, BODY_TEXT,
    BUTTON_BORDER, DIM_TEXT, GOLD_TEXT, INSET_BORDER, Skins, TAB_BORDER, TabButton, TabText, Ui,
    VolumeFill, VolumeHandle, VolumeLabel,
};
use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

#[derive(Component)]
pub struct SettingsPageRoot;

#[derive(Component)]
pub struct SettingsPanelAnim {
    pub timer: f32,
    pub duration: f32,
    pub closing: bool,
}

#[derive(Component, Clone, Copy)]
pub enum SettingField {
    FogOfWar,
    Difficulty,
    AiAggressiveness,
    Ironman,
    Fullscreen,
    Music,
}

#[derive(Component, Clone, Copy)]
pub enum SettingCheckIcon {
    FogOfWar,
    Ironman,
    Fullscreen,
    Music,
}

pub fn setup_settings_page(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui: Res<Ui>,
    skins: Res<Skins>,
    settings: Res<GameSettings>,
    prev: Res<PreviousMenuState>,
    mut active_bg: ResMut<ActiveMenuBackground>,
) {
    let is_from_map = matches!(prev.0, AppState::ScenarioSelect | AppState::InGame);

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
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            SettingsPageRoot,
        ))
        .with_children(|root| {
            // 1. Arka Plan & Vinyet Katmanı
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
                if is_from_map {
                    // Oyun içinden veya senaryo seçiminden gelindiyse harita arka planda görünsün:
                    bg_container.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            left: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
                        Pickable::IGNORE,
                    ));
                } else {
                    // A) Sabit Arka Plan Görseli
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

                    // B) Optimize Statik Vinyet & Sinematik Karartma
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
                }
            });

            // 4. Ayarlar Sayfa İçeriği (Tüm sayfa tek bir kapsayıcıda birlikte gerçek şeffaflıkla açılır/kapanır)
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
                SettingsPanelAnim {
                    timer: 0.0,
                    duration: 0.18,
                    closing: false,
                },
            ))
            .with_children(|page| {
                headed_panel(
                    page,
                    &ui,
                    &ui.headed,
                    "OYUN AYARLARI",
                    (Val::Px(580.0), Val::Px(490.0)),
                    |p| {
                        // Üst Sekmeler ("Oyun", "Ses", "Grafik")
                        p.spawn(Node {
                            column_gap: Val::Px(6.0),
                            flex_shrink: 0.0,
                            ..default()
                        })
                        .with_children(|row| {
                            for (i, name) in ["Oyun", "Ses", "Grafik"].iter().enumerate() {
                                let is_on = i == settings.tab;
                                let tab_img = if is_on { &ui.tab_on } else { &ui.tab_off };
                                let col = if is_on { GOLD_TEXT } else { BODY_TEXT };
                                row.spawn((
                                    Interaction::default(),
                                    FocusPolicy::Block,
                                    TabButton(i),
                                    Node {
                                        width: Val::Px(112.0),
                                        height: Val::Px(34.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    sliced(tab_img, TAB_BORDER),
                                ))
                                .observe(move |_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                    s.tab = i;
                                })
                                .with_children(|t| text(t, &ui, name, 17.0, col));
                            }
                        });

                        // İç Kutu (inset.png)
                        p.spawn(Node {
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(20.0)),
                            row_gap: Val::Px(14.0),
                            position_type: PositionType::Relative,
                            ..default()
                        })
                        .with_children(|c| {
                            frame(c, &ui.inset, INSET_BORDER);

                            // Checkbox Satırı: Savaş Sisi & Demir Adam
                            c.spawn(Node {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .with_children(|r| {
                                // 1. Savaş Sisi
                                r.spawn((
                                    Interaction::default(),
                                    FocusPolicy::Block,
                                    Node {
                                        align_items: AlignItems::Center,
                                        column_gap: Val::Px(10.0),
                                        ..default()
                                    },
                                ))
                                .observe(|_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                    s.fog_of_war = !s.fog_of_war;
                                })
                                .with_children(|chk| {
                                    let img = if settings.fog_of_war {
                                        ui.check_on.clone()
                                    } else {
                                        ui.check_off.clone()
                                    };
                                    chk.spawn((
                                        SettingCheckIcon::FogOfWar,
                                        Node {
                                            width: Val::Px(24.0),
                                            height: Val::Px(24.0),
                                            ..default()
                                        },
                                        ImageNode::new(img),
                                        Pickable::IGNORE,
                                    ));
                                    chk.spawn((
                                        SettingField::FogOfWar,
                                        Text::new("Savaş Sisi (Fog of War)"),
                                        TextFont {
                                            font: ui.font.clone().into(),
                                            font_size: FontSize::Px(16.0),
                                            ..default()
                                        },
                                        TextColor(BODY_TEXT),
                                        Pickable::IGNORE,
                                    ));
                                });

                                // 2. Demir Adam (Ironman)
                                r.spawn((
                                    Interaction::default(),
                                    FocusPolicy::Block,
                                    Node {
                                        align_items: AlignItems::Center,
                                        column_gap: Val::Px(10.0),
                                        ..default()
                                    },
                                ))
                                .observe(|_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                    s.ironman = !s.ironman;
                                })
                                .with_children(|chk| {
                                    let img = if settings.ironman {
                                        ui.check_on.clone()
                                    } else {
                                        ui.check_off.clone()
                                    };
                                    chk.spawn((
                                        SettingCheckIcon::Ironman,
                                        Node {
                                            width: Val::Px(24.0),
                                            height: Val::Px(24.0),
                                            ..default()
                                        },
                                        ImageNode::new(img),
                                        Pickable::IGNORE,
                                    ));
                                    chk.spawn((
                                        SettingField::Ironman,
                                        Text::new("Demir Adam (Ironman)"),
                                        TextFont {
                                            font: ui.font.clone().into(),
                                            font_size: FontSize::Px(16.0),
                                            ..default()
                                        },
                                        TextColor(BODY_TEXT),
                                        Pickable::IGNORE,
                                    ));
                                });
                            });

                            // Checkbox Satırı 2: Tam Ekran & Müzik
                            c.spawn(Node {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .with_children(|r| {
                                r.spawn((
                                    Interaction::default(),
                                    FocusPolicy::Block,
                                    Node {
                                        align_items: AlignItems::Center,
                                        column_gap: Val::Px(10.0),
                                        ..default()
                                    },
                                ))
                                .observe(|_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                    s.fullscreen = !s.fullscreen;
                                })
                                .with_children(|chk| {
                                    let img = if settings.fullscreen {
                                        ui.check_on.clone()
                                    } else {
                                        ui.check_off.clone()
                                    };
                                    chk.spawn((
                                        SettingCheckIcon::Fullscreen,
                                        Node {
                                            width: Val::Px(24.0),
                                            height: Val::Px(24.0),
                                            ..default()
                                        },
                                        ImageNode::new(img),
                                        Pickable::IGNORE,
                                    ));
                                    text(chk, &ui, "Tam Ekran Modu", 16.0, BODY_TEXT);
                                });

                                r.spawn((
                                    Interaction::default(),
                                    FocusPolicy::Block,
                                    Node {
                                        align_items: AlignItems::Center,
                                        column_gap: Val::Px(10.0),
                                        ..default()
                                    },
                                ))
                                .observe(|_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                    s.music = !s.music;
                                })
                                .with_children(|chk| {
                                    let img = if settings.music {
                                        ui.check_on.clone()
                                    } else {
                                        ui.check_off.clone()
                                    };
                                    chk.spawn((
                                        SettingCheckIcon::Music,
                                        Node {
                                            width: Val::Px(24.0),
                                            height: Val::Px(24.0),
                                            ..default()
                                        },
                                        ImageNode::new(img),
                                        Pickable::IGNORE,
                                    ));
                                    text(chk, &ui, "Oyun Müziği", 16.0, BODY_TEXT);
                                });
                            });

                            // Zorluk & Yapay Zeka Seçim Düğmeleri
                            c.spawn(Node {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(12.0),
                                ..default()
                            })
                            .with_children(|r| {
                                let mut diff_btn = ui_button(
                                    r,
                                    &ui,
                                    &skins.secondary,
                                    BUTTON_BORDER,
                                    &format!("Zorluk: {}", settings.difficulty),
                                    Vec2::new(235.0, 36.0),
                                    Some("Oyun zorluğunu değiştirmek için tıkla"),
                                );
                                diff_btn.observe(|_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                    s.difficulty = match s.difficulty.as_str() {
                                        "Kolay" => "Normal".to_string(),
                                        "Normal" => "Zor".to_string(),
                                        "Zor" => "Uzman".to_string(),
                                        _ => "Kolay".to_string(),
                                    };
                                });
                                diff_btn.with_children(|b| {
                                    b.spawn((
                                        SettingField::Difficulty,
                                        Text::new(""),
                                        Node {
                                            display: Display::None,
                                            ..default()
                                        },
                                    ));
                                });

                                let mut ai_btn = ui_button(
                                    r,
                                    &ui,
                                    &skins.secondary,
                                    BUTTON_BORDER,
                                    &format!("Yapay Zeka: {}", settings.ai_aggressiveness),
                                    Vec2::new(235.0, 36.0),
                                    Some("Yapay zeka saldırganlığını değiştir"),
                                );
                                ai_btn.observe(|_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                    s.ai_aggressiveness = match s.ai_aggressiveness.as_str() {
                                        "Standart" => "Agresif".to_string(),
                                        "Agresif" => "Pasif".to_string(),
                                        _ => "Standart".to_string(),
                                    };
                                });
                            });

                            // Ses Slider'ı
                            c.spawn(Node {
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(12.0),
                                ..default()
                            })
                            .with_children(|r| {
                                text(r, &ui, "Ses Düzeyi", 16.0, BODY_TEXT);
                                spawn_slider_interactive(
                                    r,
                                    &ui,
                                    &skins,
                                    220.0,
                                    (settings.volume * 100.0).clamp(0.0, 100.0),
                                    |down| {
                                        down.observe(|_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                            s.volume = (s.volume - 0.1).max(0.0);
                                        });
                                    },
                                    |up| {
                                        up.observe(|_: On<PointerClick>, mut s: ResMut<GameSettings>| {
                                            s.volume = (s.volume + 0.1).min(1.0);
                                        });
                                    },
                                );
                            });

                            c.spawn((
                                TabText,
                                Text::new("Oyun: zorluk, savaş sisi ve otomatik kayıt tercihleri."),
                                TextFont {
                                    font: ui.font.clone().into(),
                                    font_size: FontSize::Px(14.5),
                                    ..default()
                                },
                                TextColor(DIM_TEXT),
                            ));
                        });

                        divider(p, &ui);

                        // Alt Butonlar: Kaydet ve Dön
                        let mut back_btn = ui_button(
                            p,
                            &ui,
                            &skins.primary,
                            BUTTON_BORDER,
                            "Kaydet ve Dön",
                            Vec2::new(180.0, 40.0),
                            Some("Ayarları kaydet ve önceki ekrana dön"),
                        );
                        back_btn.observe(
                            |_: On<PointerClick>, mut anim_q: Query<&mut SettingsPanelAnim>| {
                                for mut anim in &mut anim_q {
                                    if !anim.closing {
                                        anim.closing = true;
                                        anim.timer = 0.0;
                                        anim.duration = 0.18;
                                    }
                                }
                            },
                        );
                    },
                );
            });
        });
}

pub fn handle_settings_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut anim_q: Query<&mut SettingsPanelAnim>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        for mut anim in &mut anim_q {
            if !anim.closing {
                anim.closing = true;
                anim.timer = 0.0;
                anim.duration = 0.18;
            }
        }
    }
}

pub fn update_settings_animation(
    mut commands: Commands,
    time: Res<Time>,
    prev: Res<PreviousMenuState>,
    mut next_state: ResMut<NextState<AppState>>,
    root_q: Query<Entity, With<SettingsPageRoot>>,
    mut panel_q: Query<(Entity, &mut SettingsPanelAnim)>,
    children_q: Query<&Children>,
    fade_q: Query<&UiFadeAlphaTracker>,
    mut img_q: Query<&mut ImageNode>,
    mut text_q: Query<&mut TextColor>,
    mut bg_q: Query<&mut BackgroundColor>,
    mut border_q: Query<&mut BorderColor>,
) {
    let settings_root_opt = root_q.iter().next();
    let fade_whole_page = matches!(prev.0, AppState::ScenarioSelect | AppState::InGame);

    for (panel_root, mut anim) in &mut panel_q {
        let opacity = if !anim.closing {
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
                next_state.set(prev.0);
            }
            (1.0 - ease).clamp(0.0, 1.0)
        };

        let target_entity = if fade_whole_page {
            settings_root_opt.unwrap_or(panel_root)
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

pub fn sync_settings_page_ui(
    settings: Res<GameSettings>,
    ui: Option<Res<Ui>>,
    mut check_icons_q: Query<(&SettingCheckIcon, &mut ImageNode), Without<TabButton>>,
    mut tabs_q: Query<(&TabButton, &mut ImageNode)>,
    mut tab_label_q: Query<&mut Text, (With<TabText>, Without<VolumeLabel>)>,
    mut fill_q: Query<(&VolumeFill, &mut Node), (Without<VolumeHandle>, Without<SettingsPageRoot>)>,
    mut handle_q: Query<&mut Node, (With<VolumeHandle>, Without<VolumeFill>, Without<SettingsPageRoot>)>,
    mut vol_label_q: Query<&mut Text, (With<VolumeLabel>, Without<TabText>)>,
) {
    if !settings.is_changed() {
        return;
    }

    if let Some(ref ui) = ui {
        for (icon_kind, mut img) in &mut check_icons_q {
            let on = match icon_kind {
                SettingCheckIcon::FogOfWar => settings.fog_of_war,
                SettingCheckIcon::Ironman => settings.ironman,
                SettingCheckIcon::Fullscreen => settings.fullscreen,
                SettingCheckIcon::Music => settings.music,
            };
            img.image = if on {
                ui.check_on.clone()
            } else {
                ui.check_off.clone()
            };
        }

        for (tab, mut img) in &mut tabs_q {
            img.image = if tab.0 == settings.tab {
                ui.tab_on.clone()
            } else {
                ui.tab_off.clone()
            };
        }

        for mut text in &mut tab_label_q {
            text.0 = match settings.tab {
                0 => "Oyun: Zorluk, savaş sisi ve demir adam modu tercihleri.".to_string(),
                1 => "Ses: Müzik ve efekt ses düzeylerini ayarlayabilirsiniz.".to_string(),
                2 => "Grafik: Tam ekran ve çözünürlük ayarları.".to_string(),
                _ => "".to_string(),
            };
        }

        let pct = (settings.volume * 100.0).clamp(0.0, 100.0);
        for (fill, mut node) in &mut fill_q {
            node.width = Val::Px((fill.0 * (pct / 100.0)).round());
        }

        for mut node in &mut handle_q {
            node.left = Val::Percent(pct);
        }

        for mut text in &mut vol_label_q {
            text.0 = format!("%{:.0}", pct);
        }
    }
}

pub fn cleanup_settings_page(mut commands: Commands, q: Query<Entity, With<SettingsPageRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
