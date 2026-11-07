//! EU4 tarzı UI Katalog / Showcase + Örnek Ekranlar:
//!  - 5 Sayfalık Katalog (Showcase):
//!      Sayfa 1: Tüm Butonlar (4 tür x 2 boyut x 4 durum = 32 varyant + etkileşimli)
//!      Sayfa 2: Başlıklı Paneller (Üstten & İçeriden)
//!      Sayfa 3: Başlıksız (Headersız) Paneller
//!      Sayfa 4: Kontroller & Çubuklar (Sekmeler, Checkbox, Radio, Kapat, Slider, Progress, Scrollbar)
//!      Sayfa 5: İkonlar, Çerçeveler, L-Biçimli HUD, Tarih & Hız Barı
//!  - Örnek Oyun Ekranları: Ana Menü, Ayarlar, Oyun İçi HUD
//! Klavye: + / - tuşları UI ölçeğini değiştirir, Sol/Sağ ok tuşları katalog sayfalarını gezer.

#![allow(deprecated, dead_code)]

use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::ui::FocusPolicy;
use dawn_of_civilization::ui::{
    button, checkbox_row, divider, frame, headed_panel, header_bar_with, plain_panel, progress,
    sliced, spawn_clipped_bar, spawn_date_and_speed_hud, spawn_flag_and_ribbon_hud,
    spawn_slider_with_actions, static_button, text, BAR_BORDER, BIG_BUTTON_BORDER, BODY_TEXT,
    BUTTON_BORDER, DARK_TEXT, DIM_TEXT, GOLD_TEXT, HEADER_BORDER, INSET_BORDER, PANEL_BORDER,
    PLAIN_BORDER, Skins, SpeedPip, TAB_BORDER, TIP_BORDER, TIP_DEFAULT, TabButton, TabText,
    TipText, Ui, UiPlugin, VolumeFill, VolumeHandle, VolumeLabel,
};

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
enum Screen {
    Showcase(usize), // 0..5
    Main,
    Settings,
    Hud,
}

#[derive(Resource)]
struct Current(Screen);

#[derive(Resource)]
struct Settings {
    fullscreen: bool,
    music: bool,
    volume: f32,
    tab: usize,
    event_open: bool,
    speed: usize,
}

#[derive(Component, Clone, Copy)]
enum Action {
    Goto(Screen),
    Quit,
    ToggleFullscreen,
    ToggleMusic,
    VolUp,
    VolDown,
    SpeedUp,
    SpeedDown,
    Tab(usize),
    ShowEvent,
    CloseEvent,
    Noop,
}

#[derive(Component)]
struct EventPopup;

#[derive(Component, Clone, Copy)]
enum CheckBox {
    Fullscreen,
    Music,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dawn of Civilization — EU4 UI Katalog & Örnek".into(),
                resolution: (1280u32, 720u32).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(UiScale(1.0))
        .insert_resource(Current(Screen::Showcase(0)))
        .insert_resource(Settings {
            fullscreen: false,
            music: true,
            volume: 0.7,
            tab: 0,
            event_open: true,
            speed: 3,
        })
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_clicks,
                apply_screen,
                apply_event,
                checkbox_visuals,
                tab_visuals,
                volume_visuals,
                speed_visuals,
                scale_and_nav_keys,
                debug_capture,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>, ui: Res<Ui>, skins: Res<Skins>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(ui.menu_bg.clone()),
    ));

    build_showcase_pages(&mut commands, &ui, &skins, &server);
    build_main(&mut commands, &ui, &skins);
    build_settings(&mut commands, &ui, &skins);
    build_hud(&mut commands, &ui, &skins, &server);
}

fn showcase_shell(
    commands: &mut Commands,
    ui: &Ui,
    skins: &Skins,
    page: usize,
    title: &str,
    content: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let display = if page == 0 {
        Display::Flex
    } else {
        Display::None
    };
    commands
        .spawn((
            Screen::Showcase(page),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(14.0)),
                row_gap: Val::Px(10.0),
                display,
                ..default()
            },
        ))
        .with_children(|root| {
            // Üst gezinme çubuğu
            root.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(52.0),
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(18.0)),
                column_gap: Val::Px(8.0),
                flex_shrink: 0.0,
                ..default()
            })
            .with_children(|bar| {
                frame(bar, &ui.topbar, HEADER_BORDER);
                text(bar, ui, title, 18.0, GOLD_TEXT);
                bar.spawn(Node {
                    flex_grow: 1.0,
                    ..default()
                });
                let pages = [
                    (0, "1: Butonlar"),
                    (1, "2: Başlıklı"),
                    (2, "3: Headersız"),
                    (3, "4: Kontroller"),
                    (4, "5: İkon & Bar"),
                ];
                for (idx, lbl) in pages {
                    let sk = if idx == page {
                        &skins.primary
                    } else {
                        &skins.secondary
                    };
                    button(
                        bar,
                        ui,
                        sk,
                        BUTTON_BORDER,
                        lbl,
                        Vec2::new(118.0, 34.0),
                        Action::Goto(Screen::Showcase(idx)),
                        None,
                    );
                }
                button(
                    bar,
                    ui,
                    &skins.success,
                    BUTTON_BORDER,
                    "Oyun Menüleri",
                    Vec2::new(136.0, 34.0),
                    Action::Goto(Screen::Main),
                    None,
                );
            });

            // Orta içerik alanı
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(18.0),
                ..default()
            })
            .with_children(content);
        });
}

fn build_showcase_pages(commands: &mut Commands, ui: &Ui, skins: &Skins, server: &AssetServer) {
    // ---------------- SAYFA 1: TÜM BUTONLAR ----------------
    showcase_shell(commands, ui, skins, 0, "KATALOG 1/5 — TÜM BUTONLAR", |mid| {
        let kinds = [
            (
                "secondary",
                "Secondary (Gri)",
                BODY_TEXT,
                &skins.secondary,
                &skins.big_secondary,
            ),
            (
                "primary",
                "Primary (Sarı)",
                DARK_TEXT,
                &skins.primary,
                &skins.big_primary,
            ),
            (
                "danger",
                "Danger (Kırmızı)",
                BODY_TEXT,
                &skins.danger,
                &skins.big_danger,
            ),
            (
                "success",
                "Success (Yeşil)",
                BODY_TEXT,
                &skins.success,
                &skins.big_success,
            ),
        ];
        let states = [
            ("normal", "Normal"),
            ("hover", "Hover"),
            ("pressed", "Pressed"),
            ("disabled", "Disabled"),
        ];

        // Sol: Standart Butonlar (4 tür x 4 durum + Canlı)
        headed_panel(
            mid,
            ui,
            &ui.headed,
            "STANDART BUTONLAR (4 Tür × 4 Durum + Canlı)",
            (Val::Px(605.0), Val::Px(610.0)),
            |b| {
                b.spawn(Node {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|r| {
                    r.spawn(Node {
                        width: Val::Px(112.0),
                        ..default()
                    })
                    .with_children(|c| text(c, ui, "Tür", 15.0, GOLD_TEXT));
                    for (_, st_lbl) in states {
                        r.spawn(Node {
                            width: Val::Px(86.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_children(|c| text(c, ui, st_lbl, 15.0, GOLD_TEXT));
                    }
                    r.spawn(Node {
                        width: Val::Px(86.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|c| text(c, ui, "Canlı Test", 15.0, GOLD_TEXT));
                });

                for (k_id, k_lbl, tcol, sk, _) in &kinds {
                    b.spawn(Node {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|r| {
                        r.spawn(Node {
                            width: Val::Px(112.0),
                            ..default()
                        })
                        .with_children(|c| text(c, ui, k_lbl, 15.0, BODY_TEXT));
                        for (st_id, st_lbl) in states {
                            let col = if st_id == "disabled" { DIM_TEXT } else { *tcol };
                            let img = server.load(format!("ui/button_{k_id}_{st_id}.png"));
                            static_button(
                                r,
                                ui,
                                img,
                                BUTTON_BORDER,
                                st_lbl,
                                Vec2::new(86.0, 36.0),
                                col,
                            );
                        }
                        button(
                            r,
                            ui,
                            sk,
                            BUTTON_BORDER,
                            "Canlı",
                            Vec2::new(86.0, 36.0),
                            Action::Noop,
                            None,
                        );
                    });
                }

                divider(b, ui);
                text(
                    b,
                    ui,
                    "Farklı Genişlik Testi (9-Slice Bozulmama Kontrolü):",
                    15.0,
                    GOLD_TEXT,
                );
                b.spawn(Node {
                    column_gap: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|r| {
                    button(
                        r,
                        ui,
                        &skins.primary,
                        BUTTON_BORDER,
                        "Dar",
                        Vec2::new(80.0, 36.0),
                        Action::Noop,
                        None,
                    );
                    button(
                        r,
                        ui,
                        &skins.secondary,
                        BUTTON_BORDER,
                        "Orta Boyut",
                        Vec2::new(150.0, 36.0),
                        Action::Noop,
                        None,
                    );
                    button(
                        r,
                        ui,
                        &skins.danger,
                        BUTTON_BORDER,
                        "Çok Geniş Aksiyon Düğmesi",
                        Vec2::new(280.0, 36.0),
                        Action::Noop,
                        None,
                    );
                });
            },
        );

        // Sağ: Köşe Süslü Büyük Butonlar (4 tür x 4 durum + Canlı)
        headed_panel(
            mid,
            ui,
            &ui.headed,
            "KÖŞE SÜSLÜ BÜYÜK BUTONLAR (Küçültülmüş Süs)",
            (Val::Px(615.0), Val::Px(610.0)),
            |b| {
                b.spawn(Node {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|r| {
                    r.spawn(Node {
                        width: Val::Px(82.0),
                        ..default()
                    })
                    .with_children(|c| text(c, ui, "Tür", 15.0, GOLD_TEXT));
                    for (_, st_lbl) in states {
                        r.spawn(Node {
                            width: Val::Px(94.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_children(|c| text(c, ui, st_lbl, 15.0, GOLD_TEXT));
                    }
                    r.spawn(Node {
                        width: Val::Px(94.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|c| text(c, ui, "Canlı Test", 15.0, GOLD_TEXT));
                });

                for (k_id, k_lbl, tcol, _, big_sk) in &kinds {
                    let short_lbl = k_lbl.split(' ').next().unwrap_or(k_lbl);
                    b.spawn(Node {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|r| {
                        r.spawn(Node {
                            width: Val::Px(82.0),
                            ..default()
                        })
                        .with_children(|c| text(c, ui, short_lbl, 15.0, BODY_TEXT));
                        for (st_id, st_lbl) in states {
                            let col = if st_id == "disabled" { DIM_TEXT } else { *tcol };
                            let img = server.load(format!("ui/button_big_{k_id}_{st_id}.png"));
                            static_button(
                                r,
                                ui,
                                img,
                                BIG_BUTTON_BORDER,
                                st_lbl,
                                Vec2::new(94.0, 44.0),
                                col,
                            );
                        }
                        button(
                            r,
                            ui,
                            big_sk,
                            BIG_BUTTON_BORDER,
                            "Deneme",
                            Vec2::new(94.0, 44.0),
                            Action::Noop,
                            None,
                        );
                    });
                }

                divider(b, ui);
                text(
                    b,
                    ui,
                    "Büyük Buton Genişlik Testi (4 Köşe Süsü Simetrik):",
                    15.0,
                    GOLD_TEXT,
                );
                b.spawn(Node {
                    column_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|r| {
                    button(
                        r,
                        ui,
                        &skins.big_primary,
                        BIG_BUTTON_BORDER,
                        "Yeni Sefer Başlat",
                        Vec2::new(250.0, 46.0),
                        Action::Noop,
                        None,
                    );
                    button(
                        r,
                        ui,
                        &skins.big_danger,
                        BIG_BUTTON_BORDER,
                        "Savaş İlan Et",
                        Vec2::new(250.0, 46.0),
                        Action::Noop,
                        None,
                    );
                });
            },
        );
    });

    // ---------------- SAYFA 2: TÜM PANELLER ----------------
    showcase_shell(
        commands,
        ui,
        skins,
        1,
        "KATALOG 2/5 — BAŞLIKLI PANELLER (Üstten & İçeriden)",
        |mid| {
            mid.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(14.0),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|col| {
                col.spawn(Node {
                    column_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    headed_panel(
                        row,
                        ui,
                        &ui.headed,
                        "SARI BAŞLIKLI",
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            text(b, ui, "panel_headed.png", 14.0, GOLD_TEXT);
                            text(b, ui, "• Klasik sarı/kahve üst başlık.", 14.0, BODY_TEXT);
                            text(b, ui, "• Köşeden uzaklaştırılmış süs.", 14.0, BODY_TEXT);
                            divider(b, ui);
                            button(
                                b,
                                ui,
                                &skins.primary,
                                BUTTON_BORDER,
                                "Onayla",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );
                    headed_panel(
                        row,
                        ui,
                        &ui.headed_slate,
                        "PANEL RENGİ BAŞLIK",
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            text(b, ui, "panel_headed_slate.png", 14.0, GOLD_TEXT);
                            text(
                                b,
                                ui,
                                "• Başlık sarı değil, panelin kendi arduvaz renginde.",
                                14.0,
                                BODY_TEXT,
                            );
                            divider(b, ui);
                            button(
                                b,
                                ui,
                                &skins.secondary,
                                BUTTON_BORDER,
                                "Ayrıntılar",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );
                    headed_panel(
                        row,
                        ui,
                        &ui.headed_alert,
                        "KIRMIZI BAŞLIKLI",
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            text(b, ui, "panel_headed_alert.png", 14.0, GOLD_TEXT);
                            text(b, ui, "• Savaş / tehlike paneli.", 14.0, BODY_TEXT);
                            text(b, ui, "• Açık kırmızı-bakır süsler.", 14.0, BODY_TEXT);
                            divider(b, ui);
                            button(
                                b,
                                ui,
                                &skins.danger,
                                BUTTON_BORDER,
                                "Reddet",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );
                    headed_panel(
                        row,
                        ui,
                        &ui.headed_success,
                        "YEŞİL BAŞLIKLI",
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            text(b, ui, "panel_headed_success.png", 14.0, GOLD_TEXT);
                            text(b, ui, "• Barış / zafer / onay paneli.", 14.0, BODY_TEXT);
                            text(b, ui, "• Açık zümrüt-altın süsler.", 14.0, BODY_TEXT);
                            divider(b, ui);
                            button(
                                b,
                                ui,
                                &skins.success,
                                BUTTON_BORDER,
                                "Kabul Et",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );
                });

                col.spawn(Node {
                    column_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    plain_panel(
                        row,
                        ui,
                        &ui.panel,
                        PANEL_BORDER,
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            header_bar_with(b, ui, &ui.header, "SARI İÇ HEADER");
                            text(b, ui, "panel.png + header_bar.png", 14.0, GOLD_TEXT);
                            b.spawn(Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(74.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .with_children(|ins| {
                                frame(ins, &ui.inset, INSET_BORDER);
                                text(ins, ui, "İç Kutu (inset.png)", 14.0, DIM_TEXT);
                            });
                        },
                    );

                    plain_panel(
                        row,
                        ui,
                        &ui.panel,
                        PANEL_BORDER,
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            header_bar_with(b, ui, &ui.header_slate, "PANEL RENGİ HEADER");
                            text(b, ui, "panel.png + header_bar_slate.png", 14.0, GOLD_TEXT);
                            text(
                                b,
                                ui,
                                "İçerideki başlık sarı değil, panelin kendi renginde.",
                                14.0,
                                BODY_TEXT,
                            );
                            button(
                                b,
                                ui,
                                &skins.secondary,
                                BUTTON_BORDER,
                                "Seçenek",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );

                    plain_panel(
                        row,
                        ui,
                        &ui.panel_alert,
                        PANEL_BORDER,
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            header_bar_with(b, ui, &ui.header_alert, "KIRMIZI İÇ HEADER");
                            text(b, ui, "panel_alert + header_bar_alert", 14.0, GOLD_TEXT);
                            text(b, ui, "İçerideki header da kırmızı.", 14.0, BODY_TEXT);
                            button(
                                b,
                                ui,
                                &skins.danger,
                                BUTTON_BORDER,
                                "Savaş İlanı",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );

                    plain_panel(
                        row,
                        ui,
                        &ui.panel_success,
                        PANEL_BORDER,
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            header_bar_with(b, ui, &ui.header_success, "YEŞİL İÇ HEADER");
                            text(b, ui, "panel_success + header_bar_success", 14.0, GOLD_TEXT);
                            text(b, ui, "İçerideki header da yeşil.", 14.0, BODY_TEXT);
                            button(
                                b,
                                ui,
                                &skins.success,
                                BUTTON_BORDER,
                                "İttifak Kur",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );
                });
            });
        },
    );

    // ---------------- SAYFA 3: BAŞLIKSIZ (HEADERSIZ) PANELLER ----------------
    showcase_shell(
        commands,
        ui,
        skins,
        2,
        "KATALOG 3/5 — HEADERSIZ (BAŞLIKSIZ) PANELLER",
        |mid| {
            mid.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(14.0),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|col| {
                col.spawn(Node {
                    column_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    plain_panel(
                        row,
                        ui,
                        &ui.panel,
                        PANEL_BORDER,
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            text(b, ui, "HEADERSIZ SÜSLÜ PANEL", 17.0, GOLD_TEXT);
                            text(b, ui, "panel.png (Başlıksız)", 14.0, DIM_TEXT);
                            divider(b, ui);
                            text(
                                b,
                                ui,
                                "• Üstte veya içeride header bar olmadan doğrudan gövde kullanımı.",
                                14.0,
                                BODY_TEXT,
                            );
                            text(
                                b,
                                ui,
                                "• 4 köşede simetrik altın süsler ve kesintisiz çapraz doku.",
                                14.0,
                                BODY_TEXT,
                            );
                            button(
                                b,
                                ui,
                                &skins.primary,
                                BUTTON_BORDER,
                                "Onayla",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );

                    plain_panel(
                        row,
                        ui,
                        &ui.panel_alert,
                        PANEL_BORDER,
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            text(b, ui, "HEADERSIZ KIRMIZI PANEL", 17.0, GOLD_TEXT);
                            text(b, ui, "panel_alert.png (Başlıksız)", 14.0, DIM_TEXT);
                            divider(b, ui);
                            text(
                                b,
                                ui,
                                "• Başlık çubuğu olmadan hızlı uyarı / tehlike kutusu.",
                                14.0,
                                BODY_TEXT,
                            );
                            text(b, ui, "• 4 köşede kırmızı-mercan süsler.", 14.0, BODY_TEXT);
                            button(
                                b,
                                ui,
                                &skins.danger,
                                BUTTON_BORDER,
                                "Reddet",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );

                    plain_panel(
                        row,
                        ui,
                        &ui.panel_success,
                        PANEL_BORDER,
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            text(b, ui, "HEADERSIZ YEŞİL PANEL", 17.0, GOLD_TEXT);
                            text(b, ui, "panel_success.png (Başlıksız)", 14.0, DIM_TEXT);
                            divider(b, ui);
                            text(
                                b,
                                ui,
                                "• Başlık çubuğu olmadan zafer / bildirim / onay kutusu.",
                                14.0,
                                BODY_TEXT,
                            );
                            text(b, ui, "• 4 köşede zümrüt yeşili süsler.", 14.0, BODY_TEXT);
                            button(
                                b,
                                ui,
                                &skins.success,
                                BUTTON_BORDER,
                                "Kabul Et",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );

                    plain_panel(
                        row,
                        ui,
                        &ui.panel_plain,
                        PLAIN_BORDER,
                        (Val::Px(298.0), Val::Px(295.0)),
                        |b| {
                            text(b, ui, "HEADERSIZ SADE PANEL", 17.0, GOLD_TEXT);
                            text(
                                b,
                                ui,
                                "panel_plain.png (Başlıksız & Süssüz)",
                                14.0,
                                DIM_TEXT,
                            );
                            divider(b, ui);
                            text(
                                b,
                                ui,
                                "• Ne başlık ne köşe süsü içeren tamamen sade çerçeve.",
                                14.0,
                                BODY_TEXT,
                            );
                            text(
                                b,
                                ui,
                                "• Alt barlar, yan paneller ve bilgi kartları için.",
                                14.0,
                                BODY_TEXT,
                            );
                            button(
                                b,
                                ui,
                                &skins.secondary,
                                BUTTON_BORDER,
                                "Kapat",
                                Vec2::new(140.0, 36.0),
                                Action::Noop,
                                None,
                            );
                        },
                    );
                });

                col.spawn(Node {
                    column_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    plain_panel(
                        row,
                        ui,
                        &ui.panel,
                        PANEL_BORDER,
                        (Val::Px(608.0), Val::Px(295.0)),
                        |b| {
                            text(
                                b,
                                ui,
                                "GENİŞ HEADERSIZ PANEL + İÇ KUTU (inset.png)",
                                17.0,
                                GOLD_TEXT,
                            );
                            b.spawn(Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(135.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(6.0),
                                padding: UiRect::all(Val::Px(12.0)),
                                ..default()
                            })
                            .with_children(|ins| {
                                frame(ins, &ui.inset, INSET_BORDER);
                                text(
                                    ins,
                                    ui,
                                    "İç Kutu (inset.png) — Headersız panel içinde gömülü liste veya açıklama alanı.",
                                    15.0,
                                    BODY_TEXT,
                                );
                                text(
                                    ins,
                                    ui,
                                    "Çapraz doku tüm panel yüzeyini boşluksuz kaplar.",
                                    14.0,
                                    DIM_TEXT,
                                );
                            });
                            b.spawn(Node {
                                column_gap: Val::Px(12.0),
                                ..default()
                            })
                            .with_children(|r| {
                                button(
                                    r,
                                    ui,
                                    &skins.primary,
                                    BUTTON_BORDER,
                                    "Kaydet",
                                    Vec2::new(140.0, 36.0),
                                    Action::Noop,
                                    None,
                                );
                                button(
                                    r,
                                    ui,
                                    &skins.secondary,
                                    BUTTON_BORDER,
                                    "İptal",
                                    Vec2::new(140.0, 36.0),
                                    Action::Noop,
                                    None,
                                );
                            });
                        },
                    );

                    plain_panel(
                        row,
                        ui,
                        &ui.panel_plain,
                        PLAIN_BORDER,
                        (Val::Px(608.0), Val::Px(295.0)),
                        |b| {
                            text(
                                b,
                                ui,
                                "HEADERSIZ SADE PANEL + TOOLTIP KUTULARI (tooltip.png)",
                                17.0,
                                GOLD_TEXT,
                            );
                            for txt in [
                                "Tooltip 1: Hazine +14.2 altın/ay (Vergi: +9.4, Üretim: +4.8)",
                                "Tooltip 2: Ordu Bakımı -6.5 altın/ay (Toplam 24.000 Asker)",
                                "Tooltip 3: İstikrar Etkisi: Ulusal Vergi Geliri +%10",
                            ] {
                                b.spawn(Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(48.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                })
                                .with_children(|tp| {
                                    frame(tp, &ui.tooltip, TIP_BORDER);
                                    text(tp, ui, txt, 15.0, BODY_TEXT);
                                });
                            }
                        },
                    );
                });
            });
        },
    );

    // ---------------- SAYFA 4: KONTROLLER & ÇUBUKLAR ----------------
    showcase_shell(
        commands,
        ui,
        skins,
        3,
        "KATALOG 4/5 — KONTROLLER, SLIDER, PROGRESS & SCROLLBAR",
        |mid| {
            // Sol: Seçim kontrolleri
            headed_panel(
                mid,
                ui,
                &ui.headed,
                "SEKMELER, CHECKBOX, RADIO & KAPAT",
                (Val::Px(590.0), Val::Px(560.0)),
                |b| {
                    text(b, ui, "Sekmeler (tab_active.png / tab_inactive.png):", 16.0, GOLD_TEXT);
                    b.spawn(Node {
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|r| {
                        for (i, lbl) in ["Ekonomi", "Ordu", "Diplomasi", "Teknoloji"]
                            .iter()
                            .enumerate()
                        {
                            let img = if i == 0 { &ui.tab_on } else { &ui.tab_off };
                            r.spawn((
                                Node {
                                    width: Val::Px(112.0),
                                    height: Val::Px(36.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                sliced(img, TAB_BORDER),
                            ))
                            .with_children(|t| {
                                text(t, ui, lbl, 16.0, if i == 0 { GOLD_TEXT } else { BODY_TEXT })
                            });
                        }
                    });

                    divider(b, ui);
                    text(
                        b,
                        ui,
                        "Onay Kutuları & Radyo Düğmeleri (2x Keskin):",
                        16.0,
                        GOLD_TEXT,
                    );
                    b.spawn(Node {
                        column_gap: Val::Px(28.0),
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|r| {
                        for (img, lbl) in [
                            (&ui.check_off, "Seçili Değil"),
                            (&ui.check_on, "Seçili (On)"),
                            (&ui.radio_off, "Radyo Boş"),
                            (&ui.radio_on, "Radyo Dolu"),
                        ] {
                            r.spawn(Node {
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(8.0),
                                ..default()
                            })
                            .with_children(|c| {
                                c.spawn((
                                    Node {
                                        width: Val::Px(24.0),
                                        height: Val::Px(24.0),
                                        ..default()
                                    },
                                    ImageNode::new((*img).clone()),
                                ));
                                text(c, ui, lbl, 16.0, BODY_TEXT);
                            });
                        }
                    });

                    divider(b, ui);
                    text(b, ui, "Kapat (X) Düğmesi & Sıralama Okları:", 16.0, GOLD_TEXT);
                    b.spawn(Node {
                        column_gap: Val::Px(24.0),
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|r| {
                        r.spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|c| {
                            c.spawn((
                                Node {
                                    width: Val::Px(28.0),
                                    height: Val::Px(28.0),
                                    ..default()
                                },
                                ImageNode::new(ui.close_normal.clone()),
                            ));
                            text(c, ui, "Normal", 16.0, BODY_TEXT);
                        });
                        r.spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|c| {
                            c.spawn((
                                Node {
                                    width: Val::Px(28.0),
                                    height: Val::Px(28.0),
                                    ..default()
                                },
                                ImageNode::new(ui.close_hover.clone()),
                            ));
                            text(c, ui, "Hover", 16.0, BODY_TEXT);
                        });
                        r.spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|c| {
                            c.spawn((
                                Node {
                                    width: Val::Px(16.0),
                                    height: Val::Px(16.0),
                                    ..default()
                                },
                                ImageNode::new(ui.arrow_up.clone()),
                            ));
                            c.spawn((
                                Node {
                                    width: Val::Px(16.0),
                                    height: Val::Px(16.0),
                                    ..default()
                                },
                                ImageNode::new(ui.arrow_down.clone()),
                            ));
                            text(c, ui, "Oklar", 16.0, BODY_TEXT);
                        });
                    });

                    divider(b, ui);
                    text(b, ui, "Canlı Etkileşimli Kontroller:", 16.0, GOLD_TEXT);
                    b.spawn(Node {
                        column_gap: Val::Px(28.0),
                        ..default()
                    })
                    .with_children(|r| {
                        checkbox_row(
                            r,
                            ui,
                            "Tam Ekran",
                            CheckBox::Fullscreen,
                            Action::ToggleFullscreen,
                        );
                        checkbox_row(
                            r,
                            ui,
                            "Oyun Müziği",
                            CheckBox::Music,
                            Action::ToggleMusic,
                        );
                    });
                },
            );

            // Sağ: Slider, Progress, Scrollbar
            headed_panel(
                mid,
                ui,
                &ui.headed,
                "SLIDER, İLERLEME ÇUBUKLARI & SCROLLBAR",
                (Val::Px(590.0), Val::Px(560.0)),
                |b| {
                    text(
                        b,
                        ui,
                        "Etkileşimli Slider (%0–%100 Arası Köşe & Doku Sabit):",
                        16.0,
                        GOLD_TEXT,
                    );
                    spawn_slider_with_actions(
                        b,
                        ui,
                        skins,
                        280.0,
                        70.0,
                        Action::VolDown,
                        Action::VolUp,
                    );
                    b.spawn(Node {
                        column_gap: Val::Px(12.0),
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|r| {
                        for p_val in [0.0, 12.0, 50.0, 100.0] {
                            r.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(3.0),
                                ..default()
                            })
                            .with_children(|c| {
                                text(c, ui, &format!("%{}", p_val as i32), 13.0, DIM_TEXT);
                                spawn_clipped_bar(c, &ui.track, &ui.fill, 118.0, p_val, false);
                            });
                        }
                    });

                    divider(b, ui);
                    text(
                        b,
                        ui,
                        "İlerleme Çubukları (Yeşil / Mavi / Kırmızı / Altın):",
                        16.0,
                        GOLD_TEXT,
                    );
                    progress(b, ui, "İstikrar / Ordu Morali (%82)", &ui.bar_green, 82.0);
                    progress(b, ui, "Teknoloji / Kurum İlerlemesi (%58)", &ui.bar_blue, 58.0);
                    progress(b, ui, "Savaş Yorgunluğu / İsyan Riski (%34)", &ui.bar_red, 34.0);
                    progress(b, ui, "Hazine Doluluğu (%65)", &ui.fill, 65.0);

                    divider(b, ui);
                    text(
                        b,
                        ui,
                        "Kaydırma Çubuğu (scroll_track + scroll_thumb) & İç Liste:",
                        16.0,
                        GOLD_TEXT,
                    );
                    b.spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(110.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    })
                    .with_children(|box_node| {
                        frame(box_node, &ui.inset, INSET_BORDER);
                        box_node
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(4.0),
                                ..default()
                            })
                            .with_children(|lst| {
                                text(
                                    lst,
                                    ui,
                                    "1. İstanbul — Vergi: 18.4, Üretim: 14.2",
                                    15.0,
                                    BODY_TEXT,
                                );
                                text(lst, ui, "2. Edirne — Vergi: 11.0, Üretim: 9.6", 15.0, BODY_TEXT);
                                text(lst, ui, "3. Bursa — Vergi: 13.5, Üretim: 15.1", 15.0, BODY_TEXT);
                            });
                        box_node
                            .spawn((
                                Node {
                                    width: Val::Px(14.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                sliced(&ui.scroll_track, BAR_BORDER),
                            ))
                            .with_child((
                                Node {
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(8.0),
                                    left: Val::Px(0.0),
                                    width: Val::Px(14.0),
                                    height: Val::Px(38.0),
                                    ..default()
                                },
                                sliced(&ui.scroll_thumb, BAR_BORDER),
                            ));
                    });
                },
            );
        },
    );

    // ---------------- SAYFA 5: SOL ÜST L-BİÇİMLİ BAYRAK + YATAY/DİKEY BAR HUD & İKONLAR ----------------
    showcase_shell(
        commands,
        ui,
        skins,
        4,
        "KATALOG 5/5 — SOL ÜST BAYRAK, YATAY KAYNAK ÇUBUĞU, DİKEY SEKME BARI & İKONLAR",
        |mid| {
            mid.spawn(Node {
                width: Val::Px(1236.0),
                height: Val::Px(610.0),
                ..default()
            })
            .with_children(|canvas| {
                // Sol üstte L-biçimli Bayrak + Yatay Kaynak Barı + Sol Dikey İkon Barı
                spawn_flag_and_ribbon_hud(canvas, ui, skins, server, Action::Noop, None);

                // Sağ üstte Çift Katmanlı Tarih & Oyun Hızı Barı
                canvas
                    .spawn(Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(12.0),
                        top: Val::Px(2.0),
                        ..default()
                    })
                    .with_children(|sp_wrap| {
                        spawn_date_and_speed_hud(
                            sp_wrap,
                            ui,
                            skins,
                            3,
                            "11 Kasım 1444",
                            Action::SpeedDown,
                            Action::SpeedUp,
                        );
                    });

                // L-biçimli HUD'un iç boşluğuna oturan İkon & Çerçeve Kataloğu
                canvas
                    .spawn(Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(66.0),
                        top: Val::Px(84.0),
                        ..default()
                    })
                    .with_children(|col| {
                        headed_panel(
                            col,
                            ui,
                            &ui.headed,
                            "9 ALTIN İKON (icon_frame.png) & HOVER / PARÇA ÖNİZLEMESİ",
                            (Val::Px(1160.0), Val::Px(510.0)),
                            |b| {
                                text(
                                    b,
                                    ui,
                                    "Üstte yatay kaynak çubuğu (hud_ribbon_bar.png) ve solda dikey sekme çubuğu (hud_vertical_bar.png) bayrak kutusunun (hud_flag_crest.png) arkasına birleşik girer:",
                                    15.0,
                                    BODY_TEXT,
                                );
                                b.spawn(Node {
                                    column_gap: Val::Px(22.0),
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(4.0)),
                                    ..default()
                                })
                                .with_children(|row| {
                                    let icons = [
                                        ("coin", "Hazine"),
                                        ("manpower", "İnsan Gücü"),
                                        ("shield", "Savunma"),
                                        ("prestige", "Prestij"),
                                        ("population", "Nüfus"),
                                        ("crown", "Meşruiyet"),
                                        ("science", "Bilim"),
                                        ("flag", "Diplomat"),
                                        ("production", "Üretim"),
                                    ];
                                    for (ic, name) in icons {
                                        row.spawn(Node {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(6.0),
                                            ..default()
                                        })
                                        .with_children(|cell| {
                                            cell.spawn(Node {
                                                width: Val::Px(54.0),
                                                height: Val::Px(54.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            })
                                            .with_children(|fr| {
                                                frame(fr, &ui.icon_frame, INSET_BORDER);
                                                fr.spawn((
                                                    Node {
                                                        width: Val::Px(30.0),
                                                        height: Val::Px(30.0),
                                                        ..default()
                                                    },
                                                    ImageNode::new(
                                                        server.load(format!("ui/icon_{ic}.png")),
                                                    ),
                                                ));
                                            });
                                            text(cell, ui, name, 14.0, BODY_TEXT);
                                        });
                                    }
                                });

                                divider(b, ui);
                                text(
                                    b,
                                    ui,
                                    "Ayrı Parçalar: Boş Bayrak Konteyneri (hud_flag_crest.png), Ayrı Bayrak (flag_sample.png), Hover Durumları ve Çerçeveler:",
                                    15.0,
                                    GOLD_TEXT,
                                );
                                b.spawn(Node {
                                    column_gap: Val::Px(18.0),
                                    align_items: AlignItems::Center,
                                    ..default()
                                })
                                .with_children(|r| {
                                    r.spawn(Node {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        row_gap: Val::Px(4.0),
                                        ..default()
                                    })
                                    .with_children(|c| {
                                        c.spawn((
                                            Node {
                                                width: Val::Px(106.0),
                                                height: Val::Px(66.0),
                                                ..default()
                                            },
                                            ImageNode::new(ui.hud_crest.clone()),
                                        ));
                                        text(c, ui, "hud_flag_crest", 12.5, DIM_TEXT);
                                    });
                                    r.spawn(Node {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        row_gap: Val::Px(4.0),
                                        ..default()
                                    })
                                    .with_children(|c| {
                                        c.spawn((
                                            Node {
                                                width: Val::Px(86.0),
                                                height: Val::Px(52.0),
                                                ..default()
                                            },
                                            ImageNode::new(ui.flag_sample.clone()),
                                        ));
                                        text(c, ui, "flag_sample", 12.5, DIM_TEXT);
                                    });
                                    for (lbl_st, bg_col) in [
                                        ("Normal (Şeffaf)", Color::NONE),
                                        ("Hover (Hafif Siyah)", Color::srgba(0.0, 0.0, 0.0, 0.38)),
                                        ("Pressed (Koyu Siyah)", Color::srgba(0.0, 0.0, 0.0, 0.55)),
                                    ] {
                                        r.spawn(Node {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(4.0),
                                            ..default()
                                        })
                                        .with_children(|c| {
                                            c.spawn((
                                                Node {
                                                    height: Val::Px(34.0),
                                                    padding: UiRect::axes(
                                                        Val::Px(8.0),
                                                        Val::Px(2.0),
                                                    ),
                                                    align_items: AlignItems::Center,
                                                    column_gap: Val::Px(6.0),
                                                    border_radius: BorderRadius::all(Val::Px(4.0)),
                                                    ..default()
                                                },
                                                BackgroundColor(bg_col),
                                            ))
                                            .with_children(|it| {
                                                it.spawn((
                                                    Node {
                                                        width: Val::Px(22.0),
                                                        height: Val::Px(22.0),
                                                        ..default()
                                                    },
                                                    ImageNode::new(server.load("ui/icon_coin.png")),
                                                ));
                                                it.spawn(Node {
                                                    flex_direction: FlexDirection::Column,
                                                    ..default()
                                                })
                                                .with_children(|col| {
                                                    text(col, ui, "59.817", 14.0, BODY_TEXT);
                                                    text(col, ui, "+5.155", 11.5, DIM_TEXT);
                                                });
                                            });
                                            text(c, ui, lbl_st, 13.0, DIM_TEXT);
                                        });
                                    }
                                    for lbl in ["Osmanlı", "Bizans"] {
                                        r.spawn(Node {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(4.0),
                                            ..default()
                                        })
                                        .with_children(|c| {
                                            c.spawn(Node {
                                                width: Val::Px(80.0),
                                                height: Val::Px(52.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            })
                                            .with_children(|ff| {
                                                frame(ff, &ui.flag_frame, INSET_BORDER);
                                                ff.spawn((
                                                    Node {
                                                        width: Val::Px(30.0),
                                                        height: Val::Px(30.0),
                                                        ..default()
                                                    },
                                                    ImageNode::new(server.load("ui/icon_flag.png")),
                                                ));
                                            });
                                            text(c, ui, lbl, 14.0, BODY_TEXT);
                                        });
                                    }
                                });
                            },
                        );
                    });
            });
        },
    );
}

fn build_main(commands: &mut Commands, ui: &Ui, skins: &Skins) {
    commands
        .spawn((
            Screen::Main,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
        ))
        .with_children(|root| {
            headed_panel(
                root,
                ui,
                &ui.headed,
                "ANA MENÜ",
                (Val::Px(380.0), Val::Px(460.0)),
                |p| {
                    let big = Vec2::new(250.0, 48.0);
                    button(
                        p,
                        ui,
                        &skins.big_primary,
                        BIG_BUTTON_BORDER,
                        "Yeni Oyun",
                        big,
                        Action::Goto(Screen::Hud),
                        None,
                    );
                    button(
                        p,
                        ui,
                        &skins.big_secondary,
                        BIG_BUTTON_BORDER,
                        "Oyunu Yükle",
                        big,
                        Action::Goto(Screen::Main),
                        None,
                    );
                    button(
                        p,
                        ui,
                        &skins.big_secondary,
                        BIG_BUTTON_BORDER,
                        "Ayarlar",
                        big,
                        Action::Goto(Screen::Settings),
                        None,
                    );
                    button(
                        p,
                        ui,
                        &skins.big_secondary,
                        BIG_BUTTON_BORDER,
                        "UI Katalog (5 Sayfa)",
                        big,
                        Action::Goto(Screen::Showcase(0)),
                        None,
                    );
                    divider(p, ui);
                    button(
                        p,
                        ui,
                        &skins.danger,
                        BUTTON_BORDER,
                        "Çıkış",
                        Vec2::new(140.0, 38.0),
                        Action::Quit,
                        None,
                    );
                },
            );
        });
}

fn build_settings(commands: &mut Commands, ui: &Ui, skins: &Skins) {
    commands
        .spawn((
            Screen::Settings,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
        ))
        .with_children(|root| {
            headed_panel(
                root,
                ui,
                &ui.headed,
                "AYARLAR",
                (Val::Px(560.0), Val::Px(460.0)),
                |p| {
                    p.spawn(Node {
                        column_gap: Val::Px(6.0),
                        flex_shrink: 0.0,
                        ..default()
                    })
                    .with_children(|row| {
                        for (i, name) in ["Ses", "Grafik", "Oyun"].iter().enumerate() {
                            row.spawn((
                                Interaction::default(),
                                FocusPolicy::Block,
                                Action::Tab(i),
                                TabButton(i),
                                Node {
                                    width: Val::Px(104.0),
                                    height: Val::Px(34.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                sliced(&ui.tab_off, TAB_BORDER),
                            ))
                            .with_children(|t| text(t, ui, name, 18.0, BODY_TEXT));
                        }
                    });

                    p.spawn(Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(16.0),
                        ..default()
                    })
                    .with_children(|c| {
                        frame(c, &ui.inset, INSET_BORDER);
                        checkbox_row(
                            c,
                            ui,
                            "Tam ekran",
                            CheckBox::Fullscreen,
                            Action::ToggleFullscreen,
                        );
                        checkbox_row(c, ui, "Müzik", CheckBox::Music, Action::ToggleMusic);

                        c.spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(12.0),
                            ..default()
                        })
                        .with_children(|r| {
                            text(r, ui, "Ses", 18.0, BODY_TEXT);
                            spawn_slider_with_actions(
                                r,
                                ui,
                                skins,
                                220.0,
                                70.0,
                                Action::VolDown,
                                Action::VolUp,
                            );
                        });

                        c.spawn((
                            TabText,
                            Text::new(""),
                            TextFont {
                                font: ui.font.clone().into(),
                                font_size: FontSize::Px(16.0),
                                ..default()
                            },
                            TextColor(DIM_TEXT),
                        ));
                    });

                    button(
                        p,
                        ui,
                        &skins.secondary,
                        BUTTON_BORDER,
                        "Geri",
                        Vec2::new(134.0, 38.0),
                        Action::Goto(Screen::Main),
                        None,
                    );
                },
            );
        });
}

fn build_hud(commands: &mut Commands, ui: &Ui, skins: &Skins, server: &AssetServer) {
    commands
        .spawn((
            Screen::Hud,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                display: Display::None,
                ..default()
            },
        ))
        .with_children(|root| {
            // Sol üstte L-biçimli bayrak + yatay kaynak barı + sol dikey sekme barı
            spawn_flag_and_ribbon_hud(
                root,
                ui,
                skins,
                server,
                Action::Noop,
                Some(("Katalog", Action::Goto(Screen::Showcase(4)))),
            );

            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(64.0),
                top: Val::Px(84.0),
                ..default()
            })
            .with_children(|mid| {
                headed_panel(
                    mid,
                    ui,
                    &ui.headed,
                    "ÜLKE DURUMU",
                    (Val::Px(310.0), Val::Px(430.0)),
                    |p| {
                        progress(p, ui, "İstikrar", &ui.bar_green, 78.0);
                        progress(p, ui, "Savaş yorgunluğu", &ui.bar_red, 24.0);
                        progress(p, ui, "Teknoloji ilerlemesi", &ui.bar_blue, 52.0);
                        divider(p, ui);
                        button(
                            p,
                            ui,
                            &skins.primary,
                            BUTTON_BORDER,
                            "Olay Penceresi",
                            Vec2::new(180.0, 38.0),
                            Action::ShowEvent,
                            Some("Bekleyen olayı yeniden aç"),
                        );
                        button(
                            p,
                            ui,
                            &skins.secondary,
                            BUTTON_BORDER,
                            "Ayarlar",
                            Vec2::new(180.0, 38.0),
                            Action::Goto(Screen::Settings),
                            Some("Ayarlar penceresini aç"),
                        );
                        button(
                            p,
                            ui,
                            &skins.secondary,
                            BUTTON_BORDER,
                            "Ana Menü",
                            Vec2::new(180.0, 38.0),
                            Action::Goto(Screen::Main),
                            Some("Ana menüye dön"),
                        );
                    },
                );
            });

            root.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            // Ekranın TAM ALT-ORTASINDA çift katmanlı 2-köşeli Tarih & Oyun Hızı sistemi
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            })
            .with_children(|bot_center| {
                spawn_date_and_speed_hud(
                    bot_center,
                    ui,
                    skins,
                    3,
                    "11 Kasım 1444",
                    Action::SpeedDown,
                    Action::SpeedUp,
                );
            });

            // Sol-altta bilgi/ipucu çubuğu
            root.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(64.0),
                bottom: Val::Px(8.0),
                width: Val::Px(390.0),
                height: Val::Px(36.0),
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(16.0)),
                ..default()
            })
            .with_children(|b| {
                frame(b, &ui.tooltip, TIP_BORDER);
                b.spawn((
                    TipText,
                    Text::new(TIP_DEFAULT),
                    TextFont {
                        font: ui.font.clone().into(),
                        font_size: FontSize::Px(13.5),
                        ..default()
                    },
                    TextColor(DIM_TEXT),
                ));
            });

            root.spawn((
                EventPopup,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    margin: UiRect {
                        left: Val::Px(-260.0),
                        top: Val::Px(-150.0),
                        ..default()
                    },
                    width: Val::Px(520.0),
                    height: Val::Px(300.0),
                    ..default()
                },
            ))
            .with_children(|wrap| {
                headed_panel(
                    wrap,
                    ui,
                    &ui.headed_alert,
                    "KRALIN KARARI",
                    (Val::Px(520.0), Val::Px(300.0)),
                    |p| {
                        p.spawn(Node {
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            ..default()
                        })
                        .with_children(|b| {
                            text(
                                b,
                                ui,
                                "Komşu krallık sınırımızda asker topluyor. Danışmanlar bir ittifak teklif etmeyi öneriyor, ancak bu hazineye ağır bir yük getirecek.",
                                18.0,
                                BODY_TEXT,
                            );
                        });
                        divider(p, ui);
                        p.spawn(Node {
                            column_gap: Val::Px(16.0),
                            ..default()
                        })
                        .with_children(|row| {
                            button(
                                row,
                                ui,
                                &skins.success,
                                BUTTON_BORDER,
                                "İttifak Kur",
                                Vec2::new(180.0, 40.0),
                                Action::CloseEvent,
                                Some("Hazine -200, prestij +10"),
                            );
                            button(
                                row,
                                ui,
                                &skins.danger,
                                BUTTON_BORDER,
                                "Reddet",
                                Vec2::new(180.0, 40.0),
                                Action::CloseEvent,
                                Some("Komşu ile ilişkiler -25"),
                            );
                        });
                    },
                );
            });
        });
}

fn handle_clicks(
    q: Query<(&Interaction, &Action), Changed<Interaction>>,
    mut current: ResMut<Current>,
    mut s: ResMut<Settings>,
) {
    for (interaction, action) in &q {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match *action {
            Action::Goto(screen) => current.0 = screen,
            Action::Quit => std::process::exit(0),
            Action::ToggleFullscreen => s.fullscreen = !s.fullscreen,
            Action::ToggleMusic => s.music = !s.music,
            Action::VolUp => s.volume = (s.volume + 0.1).min(1.0),
            Action::VolDown => s.volume = (s.volume - 0.1).max(0.0),
            Action::SpeedUp => s.speed = (s.speed + 1).min(5),
            Action::SpeedDown => s.speed = s.speed.saturating_sub(1),
            Action::Tab(i) => s.tab = i,
            Action::ShowEvent => s.event_open = true,
            Action::CloseEvent => s.event_open = false,
            Action::Noop => {}
        }
    }
}

fn apply_screen(current: Res<Current>, mut q: Query<(&Screen, &mut Node)>) {
    if !current.is_changed() {
        return;
    }
    for (screen, mut node) in &mut q {
        node.display = if *screen == current.0 {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn apply_event(s: Res<Settings>, mut q: Query<&mut Node, With<EventPopup>>) {
    if !s.is_changed() {
        return;
    }
    for mut node in &mut q {
        node.display = if s.event_open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn checkbox_visuals(
    s: Res<Settings>,
    ui: Option<Res<Ui>>,
    mut q: Query<(&CheckBox, &mut ImageNode)>,
) {
    let Some(ui) = ui else { return };
    if !s.is_changed() {
        return;
    }
    for (kind, mut img) in &mut q {
        let on = match kind {
            CheckBox::Fullscreen => s.fullscreen,
            CheckBox::Music => s.music,
        };
        img.image = if on {
            ui.check_on.clone()
        } else {
            ui.check_off.clone()
        };
    }
}

fn tab_visuals(
    s: Res<Settings>,
    ui: Option<Res<Ui>>,
    mut tabs: Query<(&TabButton, &mut ImageNode)>,
    mut label: Query<&mut Text, With<TabText>>,
) {
    let Some(ui) = ui else { return };
    if !s.is_changed() {
        return;
    }
    for (t, mut img) in &mut tabs {
        img.image = if t.0 == s.tab {
            ui.tab_on.clone()
        } else {
            ui.tab_off.clone()
        };
    }
    let desc = [
        "Ses: ana ses ve müzik düzeyini ayarla.",
        "Grafik: ekran modu ve çözünürlük seçenekleri.",
        "Oyun: zorluk ve otomatik kayıt tercihleri.",
    ];
    for mut t in &mut label {
        t.0 = desc[s.tab.min(2)].to_string();
    }
}

fn volume_visuals(
    s: Res<Settings>,
    mut fill: Query<(&VolumeFill, &mut Node), Without<VolumeHandle>>,
    mut handle: Query<&mut Node, (With<VolumeHandle>, Without<VolumeFill>)>,
    mut label: Query<&mut Text, With<VolumeLabel>>,
) {
    if !s.is_changed() {
        return;
    }
    let pct = (s.volume * 100.0).clamp(0.0, 100.0);
    for (vf, mut n) in &mut fill {
        n.width = Val::Px((vf.0 * (pct / 100.0)).round());
    }
    for mut n in &mut handle {
        n.left = Val::Percent(pct);
    }
    for mut t in &mut label {
        t.0 = format!("{}%", pct.round() as i32);
    }
}

fn speed_visuals(
    s: Res<Settings>,
    ui: Option<Res<Ui>>,
    mut q: Query<(&SpeedPip, &mut ImageNode)>,
) {
    let Some(ui) = ui else { return };
    if !s.is_changed() {
        return;
    }
    for (pip, mut img) in &mut q {
        img.image = if pip.0 <= s.speed {
            ui.pip_on.clone()
        } else {
            ui.pip_off.clone()
        };
    }
}

fn scale_and_nav_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut scale: ResMut<UiScale>,
    mut current: ResMut<Current>,
) {
    if keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::NumpadAdd) {
        scale.0 = (scale.0 + 0.25).min(3.0);
    }
    if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::NumpadSubtract) {
        scale.0 = (scale.0 - 0.25).max(0.5);
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        if let Screen::Showcase(p) = current.0 {
            current.0 = Screen::Showcase((p + 1) % 5);
        }
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        if let Screen::Showcase(p) = current.0 {
            current.0 = Screen::Showcase((p + 4) % 5);
        }
    }
}

fn debug_capture(mut commands: Commands, mut current: ResMut<Current>, mut frame_idx: Local<u32>) {
    let Ok(path) = std::env::var("SCREENSHOT") else { return };
    *frame_idx += 1;
    if *frame_idx == 2 {
        current.0 = match std::env::var("SCREEN").as_deref() {
            Ok("s1") | Ok("showcase1") => Screen::Showcase(0),
            Ok("s2") | Ok("showcase2") => Screen::Showcase(1),
            Ok("s3") | Ok("showcase3") => Screen::Showcase(2),
            Ok("s4") | Ok("showcase4") => Screen::Showcase(3),
            Ok("s5") | Ok("showcase5") => Screen::Showcase(4),
            Ok("settings") => Screen::Settings,
            Ok("hud") => Screen::Hud,
            Ok("main") => Screen::Main,
            _ => Screen::Showcase(0),
        };
    }
    if *frame_idx == 40 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
    if *frame_idx == 80 {
        std::process::exit(0);
    }
}
