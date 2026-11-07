#![allow(deprecated)]

use crate::map::labels::{get_country_label_color, CountryMapLabel};
use crate::map::rendering::{
    generate_random_civ_color, recolor_country_on_map, recolor_province_on_map,
};
use crate::map::types::WorldMapData;
use crate::scenario::{format_number, ActiveScenario};
use crate::state::{AppState, GameSession, PreviousMenuState};
use crate::ui::{
    bottom_slicer, close_button, date_slicer, divider, headed_panel, inset_panel, progress_width,
    ribbon_slicer, sliced, text, ui_button, vertical_slicer, vsep, HudDateText, HudHoverItem,
    Skins, SpeedPip, Tip, TipText, Ui, BODY_TEXT, BUTTON_BORDER, DIM_TEXT, GOLD_TEXT, TIP_BORDER,
};
use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

#[derive(Component)]
pub struct InGameHudRoot;

#[derive(Component)]
pub struct HudPlayerTitleText;

#[derive(Component)]
pub struct HudPlayerColorBadge;

#[derive(Component)]
pub struct HudPlayerFlagNode;

#[derive(Component)]
pub struct HudHoverText;

#[derive(Component)]
pub struct ProvincePanelRoot;

#[derive(Component)]
pub struct ProvincePanelTitle;

#[derive(Component)]
pub struct ProvincePanelDetails;

#[derive(Component)]
pub struct CountryStatusPanelRoot;

#[derive(Component)]
pub struct BackToMenuButton;

pub fn refresh_province_panel_texts(
    selected_province: Option<u32>,
    map_data: &WorldMapData,
    scenario: &ActiveScenario,
    prov_panel_q: &mut Query<&mut Node, With<ProvincePanelRoot>>,
    prov_title_q: &mut Query<
        (&mut Text, &mut TextColor),
        (
            With<ProvincePanelTitle>,
            Without<HudHoverText>,
            Without<ProvincePanelDetails>,
        ),
    >,
    prov_details_q: &mut Query<
        &mut Text,
        (
            With<ProvincePanelDetails>,
            Without<HudHoverText>,
            Without<ProvincePanelTitle>,
        ),
    >,
) {
    let Some(mut panel_node) = prov_panel_q.iter_mut().next() else {
        return;
    };
    if let Some(sel_id) = selected_province {
        if let Some(p) = map_data.provinces.get(&sel_id) {
            panel_node.display = Display::Flex;
            let civ_opt = scenario
                .province_owners
                .get(&sel_id)
                .and_then(|cid| scenario.civs.get(cid));

            if let Some((mut title_txt, mut title_col)) = prov_title_q.iter_mut().next() {
                if let Some(civ) = civ_opt {
                    title_txt.0 = format!("{} (ID: #{})  —  {}", p.name, p.id, civ.name);
                    title_col.0 = Color::srgb_u8(
                        civ.color[0].saturating_add(45),
                        civ.color[1].saturating_add(45),
                        civ.color[2].saturating_add(45),
                    );
                } else {
                    title_txt.0 = format!("{} (ID: #{}) — Tarafsız", p.name, p.id);
                    title_col.0 = GOLD_TEXT;
                }
            }

            if let Some(mut det_txt) = prov_details_q.iter_mut().next() {
                let (cap_str, pop_str, eco_str) = if let Some(civ) = civ_opt {
                    (
                        civ.capital_name.as_str(),
                        format_number(civ.population),
                        format_number(civ.economy as u64),
                    )
                } else {
                    ("-", "-".to_string(), "-".to_string())
                };
                det_txt.0 = format!(
                    "Kıta: {}   •   Kalkınma: %{}   •   Koordinat: [X: {:.0}, Y: {:.0}]\nBaşkent: {}   •   Ülke Nüfusu: {}   •   Ekonomi: {}",
                    p.continent, p.growth_rate, p.city_px[0], p.city_px[1], cap_str, pop_str, eco_str
                );
            }
        }
    } else {
        panel_node.display = Display::None;
    }
}

pub fn setup_ingame_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario: Res<ActiveScenario>,
    session: Res<GameSession>,
    ui: Res<Ui>,
    skins: Res<Skins>,
) {
    let player_civ = scenario.civs.get(&session.player_civ_id);
    let (civ_name, civ_color, flag_path, eco_val, pop_val, prov_count) =
        if let Some(c) = player_civ {
            (
                c.name.clone(),
                Color::srgb_u8(c.color[0], c.color[1], c.color[2]),
                ActiveScenario::get_flag_path(&c.tag, None),
                format_number(c.economy as u64),
                format_number(c.population),
                c.provinces.len(),
            )
        } else {
            (
                "Bilinmiyor".to_string(),
                Color::srgb(0.8, 0.2, 0.2),
                "flags/unkn/0.png".to_string(),
                "0".to_string(),
                "0".to_string(),
                0,
            )
        };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            Pickable::IGNORE,
            InGameHudRoot,
        ))
        .with_children(|root| {
            // ================================================================
            // 1. SOL ÜST L-ŞEKİLLİ EU4 HUD (Flag Crest + Ribbon Bar + Vertical Bar)
            // Ekranın tam sol-üst köşesine sıfırlanmış L-hud montajı
            // ================================================================
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            })
            .with_children(|hud_root| {
                // Üst satır: Bayrak Kutusu (ZIndex 3) + Sağdaki Yatay Kaynak Barı (ZIndex 2)
                hud_root
                    .spawn((
                        ZIndex(2),
                        Node {
                            align_items: AlignItems::FlexStart,
                            ..default()
                        },
                    ))
                    .with_children(|top_row| {
                        // A) Flag Crest (118x74, ZIndex(3))
                        top_row
                            .spawn((
                                Interaction::default(),
                                ImageNode::new(ui.hud_crest.clone()),
                                Node {
                                    width: Val::Px(118.0),
                                    height: Val::Px(74.0),
                                    flex_shrink: 0.0,
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                                ZIndex(3),
                                HudHoverItem,
                                Tip("Devlet yönetim panelini aç/kapat"),
                            ))
                            .observe(
                                |_: On<PointerClick>,
                                 mut session: ResMut<GameSession>,
                                 mut panel_q: Query<&mut Node, With<CountryStatusPanelRoot>>| {
                                    session.country_panel_open = !session.country_panel_open;
                                    for mut node in &mut panel_q {
                                        node.display = if session.country_panel_open {
                                            Display::Flex
                                        } else {
                                            Display::None
                                        };
                                    }
                                },
                            )
                            .with_children(|crest| {
                                crest
                                    .spawn(Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(12.0),
                                        top: Val::Px(10.0),
                                        width: Val::Px(86.0),
                                        height: Val::Px(52.0),
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
                                            Pickable::IGNORE,
                                            HudPlayerFlagNode,
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
                            });

                        // B) Yatay Kaynak Şeridi (hud_ribbon_bar.png, ZIndex(2))
                        // 44px yükseklik, -18px sol marj ile bayrak kutusunun tam arkasına kenetlenir
                        top_row
                            .spawn((
                                ZIndex(2),
                                Node {
                                    height: Val::Px(44.0),
                                    margin: UiRect::left(Val::Px(-18.0)),
                                    padding: UiRect {
                                        left: Val::Px(24.0),
                                        right: Val::Px(22.0),
                                        top: Val::Px(3.0),
                                        bottom: Val::Px(6.0),
                                    },
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(8.0),
                                    flex_shrink: 0.0,
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                            ))
                            .with_children(|bar| {
                                bar.spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ImageNode::new(ui.hud_ribbon.clone()).with_mode(ribbon_slicer()),
                                    FocusPolicy::Pass,
                                    Pickable::IGNORE,
                                ));

                                // Ülke renk rozeti ve adı
                                bar.spawn((
                                    Node {
                                        width: Val::Px(12.0),
                                        height: Val::Px(12.0),
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    BackgroundColor(civ_color),
                                    BorderColor::all(GOLD_TEXT),
                                    HudPlayerColorBadge,
                                ));

                                bar.spawn((
                                    Text::new(civ_name.clone()),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(14.5),
                                        ..default()
                                    },
                                    TextColor(GOLD_TEXT),
                                    HudPlayerTitleText,
                                ));

                                vsep(bar, &ui, Val::Px(22.0));

                                let stats: [(&Handle<Image>, String, &'static str); 4] = [
                                    (&ui.icon_coin, eco_val.clone(), "Ekonomik Güç ve Hazine"),
                                    (&ui.icon_population, pop_val.clone(), "Toplam Ülke Nüfusu"),
                                    (
                                        &ui.icon_flag,
                                        format!("{} Şehir", prov_count),
                                        "Kontrol Edilen Eyalet Sayısı",
                                    ),
                                    (
                                        &ui.icon_prestige,
                                        format!("Tur {}", session.turn),
                                        "Mevcut Sefer Turu",
                                    ),
                                ];

                                for (i, (icon, val, tip)) in stats.iter().enumerate() {
                                    if i > 0 {
                                        vsep(bar, &ui, Val::Px(22.0));
                                    }
                                    bar.spawn((
                                        Interaction::default(),
                                        BackgroundColor(Color::NONE),
                                        Node {
                                            align_items: AlignItems::Center,
                                            column_gap: Val::Px(6.0),
                                            padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
                                            border_radius: BorderRadius::all(Val::Px(4.0)),
                                            ..default()
                                        },
                                        HudHoverItem,
                                        Tip(tip),
                                    ))
                                    .with_children(|chip| {
                                        chip.spawn((
                                            ImageNode::new((*icon).clone()),
                                            Node {
                                                width: Val::Px(22.0),
                                                height: Val::Px(22.0),
                                                ..default()
                                            },
                                            Pickable::IGNORE,
                                        ));
                                        text(chip, &ui, val, 14.0, GOLD_TEXT);
                                    });
                                }
                            });
                    });

                // C) Dikey İkon Çubuğu (hud_vertical_bar.png, ZIndex(1))
                // 48px genişlik, -10px üst marj ile bayrak kutusunun alt-arkasına kenetlenir, sol kenara tam oturur
                hud_root
                    .spawn((
                        ZIndex(1),
                        Node {
                            width: Val::Px(48.0),
                            margin: UiRect::top(Val::Px(-10.0)),
                            padding: UiRect {
                                left: Val::Px(3.0),
                                right: Val::Px(7.0),
                                top: Val::Px(14.0),
                                bottom: Val::Px(10.0),
                            },
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(2.0),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                    ))
                    .with_children(|vbar| {
                        vbar.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ImageNode::new(ui.hud_vertical.clone()).with_mode(vertical_slicer()),
                            FocusPolicy::Pass,
                            Pickable::IGNORE,
                        ));

                        let v_icons: [(&Handle<Image>, &'static str, bool); 9] = [
                            (&ui.icon_crown, "Devlet Yönetimi ve Saray", true),
                            (&ui.icon_coin, "Hazine ve Vergi Politikası", false),
                            (&ui.icon_production, "Üretim ve İmalathaneler", false),
                            (&ui.icon_science, "Teknoloji ve Fikirler", false),
                            (&ui.icon_manpower, "Ordu ve Donanma Yönetimi", false),
                            (&ui.icon_shield, "Diplomasi ve İttifaklar", false),
                            (&ui.icon_population, "Eyaletler ve Nüfus", false),
                            (&ui.icon_flag, "Milli Kararlar ve Görevler", false),
                            (&ui.icon_prestige, "Prestij ve Büyük Güçler", false),
                        ];
                        for (idx, (icon, tip, toggles_country)) in v_icons.iter().enumerate() {
                            if idx > 0 {
                                vbar.spawn((
                                    Node {
                                        width: Val::Px(30.0),
                                        height: Val::Px(1.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)),
                                    FocusPolicy::Pass,
                                    Pickable::IGNORE,
                                ));
                            }
                            let mut btn = vbar.spawn((
                                Interaction::default(),
                                BackgroundColor(Color::NONE),
                                Node {
                                    width: Val::Px(38.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border_radius: BorderRadius::all(Val::Px(4.0)),
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                                HudHoverItem,
                                Tip(*tip),
                            ));
                            btn.with_children(|slot| {
                                slot.spawn((
                                    ImageNode::new((*icon).clone()),
                                    Node {
                                        width: Val::Px(24.0),
                                        height: Val::Px(24.0),
                                        ..default()
                                    },
                                    Pickable::IGNORE,
                                ));
                            });
                            if *toggles_country {
                                btn.observe(
                                    |_: On<PointerClick>,
                                     mut session: ResMut<GameSession>,
                                     mut panel_q: Query<
                                        &mut Node,
                                        With<CountryStatusPanelRoot>,
                                     >| {
                                        session.country_panel_open = !session.country_panel_open;
                                        for mut node in &mut panel_q {
                                            node.display = if session.country_panel_open {
                                                Display::Flex
                                            } else {
                                                Display::None
                                            };
                                        }
                                    },
                                );
                            }
                        }
                    });
            });

            // ================================================================
            // 2. SAĞ ÜST: AYARLAR VE ANA MENÜ BUTONLARI (Sağ üst köşe)
            // ================================================================
            root.spawn(Node {
                position_type: PositionType::Absolute,
                top: Val::Px(4.0),
                right: Val::Px(6.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(6.0),
                ..default()
            })
            .with_children(|top_right| {
                let mut settings_btn = ui_button(
                    top_right,
                    &ui,
                    &skins.secondary,
                    BUTTON_BORDER,
                    "Ayarlar",
                    Vec2::new(100.0, 36.0),
                    Some("Oyun ayarlarını aç"),
                );
                settings_btn.observe(
                    |_: On<PointerClick>,
                     mut prev: ResMut<PreviousMenuState>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        prev.0 = AppState::InGame;
                        next_state.set(AppState::Settings);
                    },
                );

                let mut menu_btn = ui_button(
                    top_right,
                    &ui,
                    &skins.secondary,
                    BUTTON_BORDER,
                    "Ana Menü",
                    Vec2::new(120.0, 36.0),
                    Some("Ana menüye dön"),
                );
                menu_btn.insert(BackToMenuButton);
                menu_btn.observe(
                    |_: On<PointerClick>,
                     mut prev: ResMut<PreviousMenuState>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        prev.0 = AppState::InGame;
                        next_state.set(AppState::MainMenu);
                    },
                );
            });

            // ================================================================
            // 3. SOL PANEL: ÜLKE DURUMU PANELİ (CountryStatusPanelRoot)
            // ================================================================
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(68.0),
                    top: Val::Px(90.0),
                    display: if session.country_panel_open {
                        Display::Flex
                    } else {
                        Display::None
                    },
                    ..default()
                },
                CountryStatusPanelRoot,
            ))
            .with_children(|wrap| {
                headed_panel(
                    wrap,
                    &ui,
                    &ui.headed,
                    "ÜLKE DURUMU",
                    (Val::Px(350.0), Val::Px(310.0)),
                    |p| {
                        p.spawn(Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(10.0),
                            top: Val::Px(-34.0),
                            ..default()
                        })
                        .with_children(|cb| {
                            close_button(cb, &ui, Some("Paneli kapat")).observe(
                                |_: On<PointerClick>,
                                 mut session: ResMut<GameSession>,
                                 mut panel_q: Query<
                                    &mut Node,
                                    With<CountryStatusPanelRoot>,
                                >| {
                                    session.country_panel_open = false;
                                    for mut node in &mut panel_q {
                                        node.display = Display::None;
                                    }
                                },
                            );
                        });

                        text(p, &ui, &civ_name, 16.0, GOLD_TEXT);
                        divider(p, &ui);
                        progress_width(
                            p,
                            &ui,
                            "İstikrar: +2 (%82)",
                            &ui.bar_green,
                            290.0,
                            0.82,
                        );
                        progress_width(
                            p,
                            &ui,
                            "Meşruiyet: 95 / 100",
                            &ui.bar_blue,
                            290.0,
                            0.95,
                        );
                        progress_width(
                            p,
                            &ui,
                            "Savaş Yorgunluğu: %12",
                            &ui.bar_red,
                            290.0,
                            0.12,
                        );
                    },
                );
            });

            // ================================================================
            // 4. ALT SOL: SEÇİLİ EYALET DETAY PANELİ (ProvincePanelRoot)
            // ================================================================
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(68.0),
                    bottom: Val::Px(54.0),
                    display: Display::None,
                    ..default()
                },
                ProvincePanelRoot,
            ))
            .with_children(|wrap| {
                headed_panel(
                    wrap,
                    &ui,
                    &ui.headed,
                    "EYALET DETAYI",
                    (Val::Px(495.0), Val::Px(225.0)),
                    |panel| {
                        panel
                            .spawn(Node {
                                position_type: PositionType::Absolute,
                                right: Val::Px(10.0),
                                top: Val::Px(-34.0),
                                ..default()
                            })
                            .with_children(|cb| {
                                close_button(cb, &ui, Some("Eyalet panelini kapat")).observe(
                                    |_: On<PointerClick>,
                                     mut map_data: ResMut<WorldMapData>,
                                     mut prov_panel_q: Query<
                                        &mut Node,
                                        With<ProvincePanelRoot>,
                                    >| {
                                        map_data.selected_province = None;
                                        for mut node in &mut prov_panel_q {
                                            node.display = Display::None;
                                        }
                                    },
                                );
                            });

                        panel.spawn((
                            Text::new("Seçili Şehir"),
                            TextFont {
                                font: ui.font.clone().into(),
                                font_size: FontSize::Px(17.0),
                                ..default()
                            },
                            TextColor(GOLD_TEXT),
                            ProvincePanelTitle,
                        ));

                        inset_panel(
                            panel,
                            &ui,
                            (Val::Percent(100.0), Val::Auto),
                            UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            |det_box| {
                                det_box.spawn((
                                    Text::new("Detaylar"),
                                    TextFont {
                                        font: ui.font.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(BODY_TEXT),
                                    ProvincePanelDetails,
                                ));
                            },
                        );

                        // Aksiyon Butonları: Ülke Rengini Değiştir & Şehri Fethet
                        panel
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(10.0),
                                margin: UiRect::top(Val::Px(4.0)),
                                ..default()
                            })
                            .with_children(|actions| {
                                // 1. Ülke Rengini Değiştir Butonu (EU4 Secondary Skin)
                                ui_button(
                                    actions,
                                    &ui,
                                    &skins.secondary,
                                    BUTTON_BORDER,
                                    "Ülke Rengini Değiştir",
                                    Vec2::new(215.0, 36.0),
                                    Some("Seçili eyaletin bağlı olduğu ülkenin harita rengini değiştir"),
                                )
                                .observe(
                                    |_: On<PointerClick>,
                                     map_data: Res<WorldMapData>,
                                     session: Res<GameSession>,
                                     mut scenario: ResMut<ActiveScenario>,
                                     mut images: ResMut<Assets<Image>>,
                                     mut badge_q: Query<
                                        &mut BackgroundColor,
                                        With<HudPlayerColorBadge>,
                                     >,
                                     mut prov_panel_q: Query<&mut Node, With<ProvincePanelRoot>>,
                                     mut prov_title_q: Query<
                                        (&mut Text, &mut TextColor),
                                        (
                                            With<ProvincePanelTitle>,
                                            Without<HudHoverText>,
                                            Without<ProvincePanelDetails>,
                                        ),
                                     >,
                                     mut prov_details_q: Query<
                                        &mut Text,
                                        (
                                            With<ProvincePanelDetails>,
                                            Without<HudHoverText>,
                                            Without<ProvincePanelTitle>,
                                        ),
                                     >,
                                     label_parent_q: Query<(&CountryMapLabel, &Children)>,
                                     mut text_col_q: Query<
                                        &mut TextColor,
                                        (
                                            Without<ProvincePanelTitle>,
                                            Without<ProvincePanelDetails>,
                                            Without<HudHoverText>,
                                        ),
                                     >| {
                                        let Some(sel_pid) = map_data.selected_province else {
                                            return;
                                        };
                                        let Some(&cid) = scenario.province_owners.get(&sel_pid)
                                        else {
                                            return;
                                        };
                                        let new_col =
                                            generate_random_civ_color(cid.wrapping_add(sel_pid));
                                        if let Some(civ) = scenario.civs.get_mut(&cid) {
                                            civ.color = new_col;
                                        }
                                        recolor_country_on_map(
                                            &map_data, &scenario, &mut images, cid,
                                        );
                                        let new_label_col = get_country_label_color(new_col);
                                        for (lbl, children) in &label_parent_q {
                                            if lbl.civ_id == cid {
                                                for child in children.iter() {
                                                    if let Ok(mut tc) = text_col_q.get_mut(child) {
                                                        tc.0 = new_label_col;
                                                    }
                                                }
                                            }
                                        }
                                        if cid == session.player_civ_id {
                                            for mut badge_bg in &mut badge_q {
                                                badge_bg.0 = Color::srgb_u8(
                                                    new_col[0], new_col[1], new_col[2],
                                                );
                                            }
                                        }
                                        refresh_province_panel_texts(
                                            map_data.selected_province,
                                            &map_data,
                                            &scenario,
                                            &mut prov_panel_q,
                                            &mut prov_title_q,
                                            &mut prov_details_q,
                                        );
                                    },
                                );

                                // 2. Şehri Fethet (Oyuncu Ülkesine Kat) Butonu (EU4 Danger Skin)
                                ui_button(
                                    actions,
                                    &ui,
                                    &skins.danger,
                                    BUTTON_BORDER,
                                    "Şehri Ülkeme Kat",
                                    Vec2::new(200.0, 36.0),
                                    Some("Seçili eyaleti devlet topraklarına kat"),
                                )
                                .observe(
                                    |_: On<PointerClick>,
                                     map_data: Res<WorldMapData>,
                                     session: Res<GameSession>,
                                     mut scenario: ResMut<ActiveScenario>,
                                     mut images: ResMut<Assets<Image>>,
                                     mut prov_panel_q: Query<&mut Node, With<ProvincePanelRoot>>,
                                     mut prov_title_q: Query<
                                        (&mut Text, &mut TextColor),
                                        (
                                            With<ProvincePanelTitle>,
                                            Without<HudHoverText>,
                                            Without<ProvincePanelDetails>,
                                        ),
                                     >,
                                     mut prov_details_q: Query<
                                        &mut Text,
                                        (
                                            With<ProvincePanelDetails>,
                                            Without<HudHoverText>,
                                            Without<ProvincePanelTitle>,
                                        ),
                                     >| {
                                        let Some(sel_pid) = map_data.selected_province else {
                                            return;
                                        };
                                        let player_cid = session.player_civ_id;
                                        let old_cid =
                                            scenario.province_owners.get(&sel_pid).copied();
                                        if old_cid == Some(player_cid) {
                                            return;
                                        }
                                        if let Some(oc) = old_cid {
                                            if let Some(old_civ) = scenario.civs.get_mut(&oc) {
                                                old_civ.provinces.retain(|&p| p != sel_pid);
                                            }
                                        }
                                        if let Some(player_civ) =
                                            scenario.civs.get_mut(&player_cid)
                                        {
                                            if !player_civ.provinces.contains(&sel_pid) {
                                                player_civ.provinces.push(sel_pid);
                                            }
                                        }
                                        scenario.province_owners.insert(sel_pid, player_cid);
                                        recolor_province_on_map(
                                            &map_data,
                                            &scenario,
                                            &mut images,
                                            sel_pid,
                                        );
                                        refresh_province_panel_texts(
                                            map_data.selected_province,
                                            &map_data,
                                            &scenario,
                                            &mut prov_panel_q,
                                            &mut prov_title_q,
                                            &mut prov_details_q,
                                        );
                                    },
                                );
                            });
                    },
                );
            });

            // ================================================================
            // 5. ALT ORTA: ÇİFT KATMANLI TARİH & HIZ HUD (Date Bar + Bottom Bar)
            // Ekranın tam alt ortasına sıfırlanmış simetrik hız & tarih sistemi
            // ================================================================
            root.spawn(Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-142.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|stack| {
                // Üst Katman: Tarih Plakası (hud_date_bar.png, ZIndex(1))
                stack
                    .spawn((
                        ImageNode::new(ui.hud_date.clone()).with_mode(date_slicer()),
                        Node {
                            width: Val::Px(196.0),
                            height: Val::Px(32.0),
                            margin: UiRect::bottom(Val::Px(-6.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::bottom(Val::Px(3.0)),
                            ..default()
                        },
                        ZIndex(1),
                        Tip("Mevcut oyun tarihi"),
                    ))
                    .with_children(|date_box| {
                        date_box.spawn((
                            Text::new(format!(
                                "{:02}.{:02}.{}",
                                session.day, session.month, session.year
                            )),
                            TextFont {
                                font: ui.font.clone().into(),
                                font_size: FontSize::Px(13.5),
                                ..default()
                            },
                            TextColor(GOLD_TEXT),
                            HudDateText,
                        ));
                    });

                // Alt Katman: Hız Kontrol Barı (hud_bottom_bar.png, ZIndex(2))
                stack
                    .spawn((
                        ImageNode::new(ui.hud_bottom.clone()).with_mode(bottom_slicer()),
                        Node {
                            width: Val::Px(284.0),
                            height: Val::Px(42.0),
                            padding: UiRect::axes(Val::Px(20.0), Val::Px(6.0)),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        ZIndex(2),
                    ))
                    .with_children(|bbar| {
                        ui_button(
                            bbar,
                            &ui,
                            &skins.secondary,
                            BUTTON_BORDER,
                            "-",
                            Vec2::new(28.0, 26.0),
                            Some("Oyun hızını azalt"),
                        )
                        .observe(
                            |_: On<PointerClick>,
                             mut session: ResMut<GameSession>,
                             ui: Res<Ui>,
                             mut pips: Query<(&SpeedPip, &mut ImageNode)>| {
                                session.speed = session.speed.saturating_sub(1).max(1);
                                for (pip, mut img) in &mut pips {
                                    img.image = if pip.0 <= session.speed {
                                        ui.pip_on.clone()
                                    } else {
                                        ui.pip_off.clone()
                                    };
                                }
                            },
                        );

                        bbar.spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(6.0),
                            ..default()
                        })
                        .with_children(|pips| {
                            for s in 1..=5usize {
                                let img = if s <= session.speed {
                                    &ui.pip_on
                                } else {
                                    &ui.pip_off
                                };
                                pips.spawn((
                                    Interaction::default(),
                                    ImageNode::new(img.clone()),
                                    Node {
                                        width: Val::Px(22.0),
                                        height: Val::Px(5.0),
                                        ..default()
                                    },
                                    SpeedPip(s),
                                    Tip("Oyun hızı kademesi"),
                                ))
                                .observe(
                                    move |_: On<PointerClick>,
                                          mut session: ResMut<GameSession>,
                                          ui: Res<Ui>,
                                          mut pip_q: Query<(&SpeedPip, &mut ImageNode)>| {
                                        session.speed = s;
                                        for (pip, mut img) in &mut pip_q {
                                            img.image = if pip.0 <= session.speed {
                                                ui.pip_on.clone()
                                            } else {
                                                ui.pip_off.clone()
                                            };
                                        }
                                    },
                                );
                            }
                        });

                        ui_button(
                            bbar,
                            &ui,
                            &skins.secondary,
                            BUTTON_BORDER,
                            "+",
                            Vec2::new(28.0, 26.0),
                            Some("Oyun hızını artır"),
                        )
                        .observe(
                            |_: On<PointerClick>,
                             mut session: ResMut<GameSession>,
                             ui: Res<Ui>,
                             mut pips: Query<(&SpeedPip, &mut ImageNode)>| {
                                session.speed = (session.speed + 1).min(5);
                                for (pip, mut img) in &mut pips {
                                    img.image = if pip.0 <= session.speed {
                                        ui.pip_on.clone()
                                    } else {
                                        ui.pip_off.clone()
                                    };
                                }
                            },
                        );
                    });
            });

            // ================================================================
            // 6. ALT SOL: TOOLTIP / HOVER BİLGİ ÇUBUĞU (tooltip.png)
            // ================================================================
            root.spawn((
                sliced(&ui.tooltip, TIP_BORDER),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(56.0),
                    bottom: Val::Px(6.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(7.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(14.0),
                    ..default()
                },
            ))
            .with_children(|tip_bar| {
                tip_bar.spawn((
                    Text::new("Haritada bir şehrin üzerine gelin veya tıklayın"),
                    TextFont {
                        font: ui.font.clone().into(),
                        font_size: FontSize::Px(12.5),
                        ..default()
                    },
                    TextColor(BODY_TEXT),
                    HudHoverText,
                ));

                tip_bar.spawn((
                    Text::new(""),
                    TextFont {
                        font: ui.font.clone().into(),
                        font_size: FontSize::Px(12.0),
                        ..default()
                    },
                    TextColor(DIM_TEXT),
                    TipText,
                ));
            });
        });
}

pub fn cleanup_ingame_hud(mut commands: Commands, q: Query<Entity, With<InGameHudRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

pub fn handle_ingame_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut prev: ResMut<PreviousMenuState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        prev.0 = AppState::InGame;
        next_state.set(AppState::Settings);
    }
}
