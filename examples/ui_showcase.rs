use bevy::prelude::*;
use dawn_of_civilization::state::UiFonts;
use dawn_of_civilization::ui::{
    caption, display, headline, row, selectable_row_node,
    spawn_badge_chip, spawn_button, spawn_card_with_header, spawn_checkbox, spawn_gold_divider,
    spawn_ornate_card, spawn_progress_bar, spawn_stat_card, spawn_tab_item, spawn_toast,
    spawn_toggle, title, ButtonSize, ButtonStyle, ButtonVariant, PanelElevation, Space, TextTone,
    Theme, UiButton, UiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dawn of Civilization — UI Component Showcase".to_string(),
                resolution: (1440, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.04, 0.06, 0.09)))
        .init_resource::<UiFonts>()
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup_showcase)
        .run();
}

fn setup_showcase(mut commands: Commands, theme: Res<Theme>, fonts: Res<UiFonts>) {
    commands.spawn(Camera2d);

    // Kapsayıcı Tam Ekran Scroll / Flex Container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Space::XL.val(&theme.spacing)),
                flex_direction: FlexDirection::Column,
                row_gap: Space::LG.val(&theme.spacing),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(theme.colors.bg_base),
        ))
        .with_children(|root| {
            // 1. Showcase Başlık ve Tipografi Bloğu
            root.spawn(display(&theme, &fonts, "UI COMPONENT SHOWCASE"));
            root.spawn(caption(
                &theme,
                &fonts,
                "Tasarım Dili: Obsidyen & Asil Altın • Saf Rust & GPU Vektörel Hiyerarşi",
            ));
            spawn_gold_divider(root, &theme, Val::Percent(100.0));

            // Ana Yatay İki Sütunlu Grid
            root.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Space::LG.val(&theme.spacing),
                ..default()
            })
            .with_children(|grid| {
                // SOL KOLON
                grid.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Space::MD.val(&theme.spacing),
                    width: Val::Percent(50.0),
                    ..default()
                })
                .with_children(|col_left| {
                    // Kart 1: Süslü Panel & Buton Varyantları
                    spawn_ornate_card(
                        col_left,
                        &theme,
                        &fonts,
                        "Buton Varyantları & Aksiyon Kartları",
                        Some("Saf Rust ile vektörel kenarlıklar ve yüksek kontrastlı hover renkleri"),
                        |card| {
                            // Primary, Royal, Secondary Butonlar
                            card.spawn(row(&theme.spacing, Space::SM)).with_children(|r| {
                                spawn_button(r, &theme, &fonts, "Primary (M)", ButtonVariant::Primary, ButtonSize::Medium);
                                spawn_button(r, &theme, &fonts, "Royal (M)", ButtonVariant::Royal, ButtonSize::Medium);
                                spawn_button(r, &theme, &fonts, "Secondary (M)", ButtonVariant::Secondary, ButtonSize::Medium);
                                spawn_button(r, &theme, &fonts, "Danger (M)", ButtonVariant::Danger, ButtonSize::Medium);
                                spawn_button(r, &theme, &fonts, "Ghost (M)", ButtonVariant::Ghost, ButtonSize::Medium);
                            });

                            // Boyut Varyantları
                            card.spawn(row(&theme.spacing, Space::SM)).with_children(|r| {
                                spawn_button(r, &theme, &fonts, "Small Buton", ButtonVariant::Secondary, ButtonSize::Small);
                                spawn_button(r, &theme, &fonts, "Medium Buton", ButtonVariant::Primary, ButtonSize::Medium);
                                spawn_button(r, &theme, &fonts, "Large Buton", ButtonVariant::Royal, ButtonSize::Large);
                            });

                            // Standart ve Vurgulu Solid Butonlar
                            card.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Space::XS.val(&theme.spacing),
                                width: Val::Percent(100.0),
                                ..default()
                            }).with_children(|btns_col| {
                                spawn_button(
                                    btns_col,
                                    &theme,
                                    &fonts,
                                    "YENİ OYUN OLUŞTUR",
                                    ButtonVariant::Primary,
                                    ButtonSize::Large,
                                );
                                spawn_button(
                                    btns_col,
                                    &theme,
                                    &fonts,
                                    "SENARYO LİSTESİ",
                                    ButtonVariant::Royal,
                                    ButtonSize::Medium,
                                );
                            });

                            // Disabled ve Focused Durumları
                            card.spawn(row(&theme.spacing, Space::SM)).with_children(|r| {
                                let mut disabled_btn = spawn_button(r, &theme, &fonts, "Disabled Buton", ButtonVariant::Secondary, ButtonSize::Medium);
                                disabled_btn.insert(UiButton {
                                    variant: ButtonVariant::Secondary,
                                    size: ButtonSize::Medium,
                                    style: ButtonStyle::Solid,
                                    disabled: true,
                                    focused: false,
                                });

                                let mut focused_btn = spawn_button(r, &theme, &fonts, "Focused Buton", ButtonVariant::Secondary, ButtonSize::Medium);
                                focused_btn.insert(UiButton {
                                    variant: ButtonVariant::Secondary,
                                    size: ButtonSize::Medium,
                                    style: ButtonStyle::Solid,
                                    disabled: false,
                                    focused: true,
                                });
                            });
                        },
                    );

                    // Kart 2: Form & Seçim Elemanları
                    spawn_card_with_header(
                        col_left,
                        &theme,
                        &fonts,
                        "Form & Ayar Elemanları",
                        PanelElevation::Flat,
                        |card| {
                            spawn_toggle(card, &theme, &fonts, "Savaş Sisi (Fog of War)", true);
                            spawn_toggle(card, &theme, &fonts, "Demir Adam Modu (Ironman)", false);
                            spawn_checkbox(card, &theme, &fonts, "Otomatik Tur Atlama", true);
                            spawn_checkbox(card, &theme, &fonts, "Ses ve Müzik Efektleri", false);
                        },
                    );
                });

                // SAĞ KOLON
                grid.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Space::MD.val(&theme.spacing),
                    width: Val::Percent(50.0),
                    ..default()
                })
                .with_children(|col_right| {
                    // Kart 3: Oyun HUD & İstatistik Parçaları
                    spawn_card_with_header(
                        col_right,
                        &theme,
                        &fonts,
                        "Strateji HUD & Göstergeleri",
                        PanelElevation::Raised,
                        |card| {
                            // İstatistik Sayaçları Satırı
                            card.spawn(row(&theme.spacing, Space::SM)).with_children(|r| {
                                spawn_stat_card(r, &theme, &fonts, "NÜFUS", "85.2 M");
                                spawn_stat_card(r, &theme, &fonts, "EKONOMİ", "₺ 1.15 T");
                                spawn_stat_card(r, &theme, &fonts, "EYALET", "81 İl");
                                spawn_stat_card(r, &theme, &fonts, "GELİR", "+12.4K / tur");
                            });

                            // Rozetler / Chip'ler
                            card.spawn(row(&theme.spacing, Space::SM)).with_children(|r| {
                                spawn_badge_chip(r, &theme, &fonts, "★ BAŞKENT ANKARA", TextTone::Accent);
                                spawn_badge_chip(r, &theme, &fonts, "MÜTTEFİK (NATO)", TextTone::Success);
                                spawn_badge_chip(r, &theme, &fonts, "SAVAŞTA (GERGİN)", TextTone::Danger);
                            });

                            // İlerleme Çubukları
                            card.spawn(headline(&theme, &fonts, "Kalkınma & Araştırma"));
                            spawn_progress_bar(card, &theme, 0.72, Val::Percent(100.0));
                            spawn_progress_bar(card, &theme, 0.35, Val::Percent(100.0));
                        },
                    );

                    // Kart 4: Bildirimler, Sekmeler & Listeler
                    spawn_card_with_header(
                        col_right,
                        &theme,
                        &fonts,
                        "Listeler, Sekmeler & Toast Bildirimleri",
                        PanelElevation::Flat,
                        |card| {
                            // Sekme Çubuğu (Tab Bar)
                            card.spawn(row(&theme.spacing, Space::XS)).with_children(|r| {
                                spawn_tab_item(r, &theme, &fonts, "Genel Bakış", 0, true);
                                spawn_tab_item(r, &theme, &fonts, "Diplomasi", 1, false);
                                spawn_tab_item(r, &theme, &fonts, "Ekonomi", 2, false);
                                spawn_tab_item(r, &theme, &fonts, "Askeriye", 3, false);
                            });

                            // Seçilebilir Liste Satırları
                            card.spawn(selectable_row_node(&theme, true)).with_children(|row| {
                                row.spawn(title(&theme, &fonts, "Modern Dünya (2026)"));
                                row.spawn(caption(&theme, &fonts, "Seçili Senaryo"));
                            });

                            card.spawn(selectable_row_node(&theme, false)).with_children(|row| {
                                row.spawn(title(&theme, &fonts, "İkinci Dünya Savaşı (1939)"));
                                row.spawn(caption(&theme, &fonts, "Tarihî Senaryo"));
                            });

                            // Toast Bildirimleri
                            spawn_toast(card, &theme, &fonts, "Oyun kaydedildi: Autosave #1", TextTone::Success);
                            spawn_toast(card, &theme, &fonts, "Uyarı: Hazine rezervleri kritik seviyede!", TextTone::Danger);
                        },
                    );
                });
            });
        });
}
