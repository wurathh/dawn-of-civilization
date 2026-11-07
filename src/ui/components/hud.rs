#![allow(deprecated, clippy::too_many_arguments)]

use crate::state::UiFonts;
use crate::ui::components::buttons::button;
use crate::ui::components::feedback::Tip;
use crate::ui::components::layout::Space;
use crate::ui::components::panels::{bottom_slicer, date_slicer, frame, ribbon_slicer, vertical_slicer};
use crate::ui::components::text::{text, text_bundle, TextTone, TextVariant};
use crate::ui::theme::{BODY_TEXT, BUTTON_BORDER, DIM_TEXT, GOLD_TEXT, INSET_BORDER, Skins, Theme, Ui};
use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

#[derive(Component)]
pub struct HudHoverItem;

#[derive(Component, Clone, Copy)]
pub struct SpeedPip(pub usize);

#[derive(Component)]
pub struct HudDateText;

#[derive(Component)]
pub struct UiChip;

/// Altın çerçeveli ikon (`icon_frame.png` + ikon görseli).
pub fn spawn_icon_frame(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    icon: Handle<Image>,
    frame_size: Vec2,
    icon_size: Vec2,
) {
    p.spawn((
        Node {
            width: Val::Px(frame_size.x),
            height: Val::Px(frame_size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            flex_shrink: 0.0,
            ..default()
        },
        Pickable::IGNORE,
    ))
    .with_children(|fr| {
        frame(fr, &ui.icon_frame, INSET_BORDER);
        fr.spawn((
            Node {
                width: Val::Px(icon_size.x),
                height: Val::Px(icon_size.y),
                ..default()
            },
            ImageNode::new(icon),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
    });
}

/// Altın çerçeveli bayrak (`flag_frame.png` + bayrak görseli).
pub fn spawn_flag_frame(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    flag: Handle<Image>,
    frame_size: Vec2,
    flag_size: Vec2,
) {
    p.spawn((
        Node {
            width: Val::Px(frame_size.x),
            height: Val::Px(frame_size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            flex_shrink: 0.0,
            ..default()
        },
        Pickable::IGNORE,
    ))
    .with_children(|ff| {
        frame(ff, &ui.flag_frame, INSET_BORDER);
        ff.spawn((
            Node {
                width: Val::Px(flag_size.x),
                height: Val::Px(flag_size.y),
                ..default()
            },
            ImageNode::new(flag),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
    });
}

/// Sol üstte L-biçimli HUD (`bevy_menu_example` birebir yapısı):
///  1) `hud_flag_crest.png` (`ZIndex(3)` ile en üstte, 118×74)
///  2) Sağında bayrak kutusunun arkasına giren yatay kaynak barı `hud_ribbon_bar.png` (`ZIndex(2)`, `-18px` sol marj)
///  3) Altında bayrak kutusunun arkasına giren dikey sekme/aksiyon barı `hud_vertical_bar.png` (`ZIndex(1)`, `-10px` üst marj, 48px genişlik)
pub fn spawn_flag_and_ribbon_hud<A: Bundle + Clone>(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    skins: &Skins,
    server: &AssetServer,
    noop_action: A,
    catalog_btn: Option<(&str, A)>,
) {
    let include_catalog_btn = catalog_btn.is_some();

    p.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        margin: if include_catalog_btn {
            UiRect {
                left: Val::Px(-10.0),
                top: Val::Px(-10.0),
                ..default()
            }
        } else {
            UiRect::default()
        },
        ..default()
    })
    .with_children(|hud_col| {
        // ÜST SATIR: Bayrak Kutusu (ZIndex 3) + Sağdaki Yatay Kaynak Barı (ZIndex 2)
        hud_col
            .spawn((
                ZIndex(2),
                Node {
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
            ))
            .with_children(|row| {
                // 1) Sol üst bayrak kutusu (118x74) — ZIndex(3) ile hem yatay hem dikey barın ÜSTÜNDE durur
                row.spawn((
                    ZIndex(3),
                    Node {
                        width: Val::Px(118.0),
                        height: Val::Px(74.0),
                        flex_shrink: 0.0,
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    ImageNode::new(ui.hud_crest.clone()),
                ))
                .with_children(|crest| {
                    crest.spawn((
                        Interaction::default(),
                        Tip("Osmanlı Devleti — Ülke ve yönetim ekranını aç"),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(12.0),
                            top: Val::Px(10.0),
                            width: Val::Px(86.0),
                            height: Val::Px(52.0),
                            ..default()
                        },
                        ImageNode::new(ui.flag_sample.clone()),
                    ));
                });

                // 2) Sağdaki yatay kaynak alanı (44px yükseklik) — ZIndex(2) ve -18px sol marj ile bayrak kutusunun ARKASINA girer
                row.spawn((
                    ZIndex(2),
                    Node {
                        height: Val::Px(44.0),
                        margin: UiRect::left(Val::Px(-18.0)),
                        padding: UiRect {
                            left: Val::Px(22.0),
                            right: Val::Px(18.0),
                            top: Val::Px(3.0),
                            bottom: Val::Px(6.0),
                        },
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(3.0),
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

                    let resource_items = [
                        (
                            "coin",
                            "59.817",
                            "+5.155",
                            "Hazine: 59.817 (+5.155/ay) — Ekonomi sekmesini aç",
                        ),
                        ("manpower", "2.6M", "+52.3B", "İnsan gücü ve ordu mevcudu"),
                        ("production", "%37", "+102", "Üretim verimliliği ve endüstri gelişimi"),
                        ("science", "8.259", "+268", "Araştırma puanı ve teknoloji ilerlemesi"),
                        ("prestige", "246", "+3.36", "Ulusal prestij ve diplomatik ağırlık"),
                        ("crown", "84", "%76", "Yönetim kapasitesi ve meşruiyet"),
                    ];

                    for (ic, main_val, sub_val, tip) in resource_items {
                        bar.spawn((
                            Interaction::default(),
                            FocusPolicy::Block,
                            HudHoverItem,
                            noop_action.clone(),
                            Tip(tip),
                            Node {
                                height: Val::Px(34.0),
                                padding: UiRect::axes(Val::Px(7.0), Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(6.0),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Node {
                                    width: Val::Px(22.0),
                                    height: Val::Px(22.0),
                                    ..default()
                                },
                                ImageNode::new(server.load(format!("ui/icon_{ic}.png"))),
                                FocusPolicy::Pass,
                                Pickable::IGNORE,
                            ));
                            item.spawn((
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                FocusPolicy::Pass,
                                Pickable::IGNORE,
                            ))
                            .with_children(|col| {
                                text(col, ui, main_val, 14.0, BODY_TEXT);
                                text(col, ui, sub_val, 11.5, DIM_TEXT);
                            });
                        });
                    }

                    if let Some((lbl, act)) = catalog_btn {
                        button(
                            bar,
                            ui,
                            &skins.primary,
                            BUTTON_BORDER,
                            lbl,
                            Vec2::new(88.0, 30.0),
                            act,
                            None,
                        );
                    }
                });
            });

        // 3) ALTTAKİ DİKEY BAR (48px genişlik) — ZIndex(1) ve -10px üst marj ile bayrak kutusunun ALT-ARKASINA girer, sol kenara sıfır durur
        hud_col
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

                let v_icons: [(&str, Option<&str>, &'static str); 9] = [
                    ("crown", None, "Devlet Yönetimi & Saray"),
                    ("coin", None, "Hazine & Vergilendirme"),
                    ("production", None, "Üretim, İnşaat & Altyapı"),
                    ("science", None, "Teknoloji & Kurumlar"),
                    ("manpower", None, "Ordu & Askerî Birlikler"),
                    ("shield", None, "Kaleler & Garnizon Savunması"),
                    ("population", Some("57"), "Nüfus, Kültür & Eyaletler (57)"),
                    ("flag", None, "Diplomasi & Antlaşmalar"),
                    ("prestige", None, "Din, Prestij & Gelenekler"),
                ];

                for (idx, (ic, badge, tip)) in v_icons.iter().enumerate() {
                    if idx > 0 {
                        vbar.spawn((
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(1.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.04)),
                            FocusPolicy::Pass,
                            Pickable::IGNORE,
                        ));
                    }
                    vbar.spawn((
                        Interaction::default(),
                        FocusPolicy::Block,
                        HudHoverItem,
                        noop_action.clone(),
                        Tip(tip),
                        Node {
                            width: Val::Px(38.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|slot| {
                        slot.spawn((
                            Node {
                                width: Val::Px(24.0),
                                height: Val::Px(24.0),
                                margin: if badge.is_some() {
                                    UiRect::bottom(Val::Px(6.0))
                                } else {
                                    UiRect::default()
                                },
                                ..default()
                            },
                            ImageNode::new(server.load(format!("ui/icon_{ic}.png"))),
                            FocusPolicy::Pass,
                            Pickable::IGNORE,
                        ));
                        if let Some(num) = badge {
                            slot.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(1.0),
                                    bottom: Val::Px(0.0),
                                    ..default()
                                },
                                Text::new(*num),
                                TextFont {
                                    font: ui.font.clone().into(),
                                    font_size: FontSize::Px(11.0),
                                    ..default()
                                },
                                TextColor(DIM_TEXT),
                                FocusPolicy::Pass,
                                Pickable::IGNORE,
                            ));
                        }
                    });
                }
            });
    });
}

/// Ekranın alt-ortasında çift katmanlı (üstte daha kısa tarih barı, altta daha geniş oyun hızı barı)
/// iki köşesi eğimli/yuvarlatılmış simetrik Tarih & Oyun Hızı bileşeni (`bevy_menu_example` birebir yapısı).
pub fn spawn_date_and_speed_hud<A: Bundle + Clone>(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    skins: &Skins,
    init_speed: usize,
    date_str: &str,
    down_action: A,
    up_action: A,
) {
    p.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    })
    .with_children(|stack| {
        // 1) ÜSTTEKİ KISA TARİH BARI (196x32) — ZIndex(1) ve -6px alt marj ile alttaki hız barının ARKASINA/ÜSTÜNE oturur
        stack
            .spawn((
                ZIndex(1),
                Node {
                    width: Val::Px(196.0),
                    height: Val::Px(32.0),
                    margin: UiRect::bottom(Val::Px(-6.0)),
                    padding: UiRect {
                        left: Val::Px(16.0),
                        right: Val::Px(16.0),
                        top: Val::Px(3.0),
                        bottom: Val::Px(5.0),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    ..default()
                },
            ))
            .with_children(|dbar| {
                dbar.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ImageNode::new(ui.hud_date.clone()).with_mode(date_slicer()),
                    FocusPolicy::Pass,
                    Pickable::IGNORE,
                ));
                dbar.spawn((
                    HudDateText,
                    Text::new(date_str),
                    TextFont {
                        font: ui.font.clone().into(),
                        font_size: FontSize::Px(15.0),
                        ..default()
                    },
                    TextColor(GOLD_TEXT),
                    FocusPolicy::Pass,
                    Pickable::IGNORE,
                ));
            });

        // 2) ALTTAKİ GENİŞ HIZ BARI (284x42) — ZIndex(2) ile üstteki tarih barının alt kenarını örterek birleşik durur
        stack
            .spawn((
                ZIndex(2),
                Node {
                    width: Val::Px(284.0),
                    height: Val::Px(42.0),
                    padding: UiRect {
                        left: Val::Px(22.0),
                        right: Val::Px(22.0),
                        top: Val::Px(6.0),
                        bottom: Val::Px(4.0),
                    },
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    position_type: PositionType::Relative,
                    ..default()
                },
            ))
            .with_children(|sbar| {
                sbar.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ImageNode::new(ui.hud_bottom.clone()).with_mode(bottom_slicer()),
                    FocusPolicy::Pass,
                    Pickable::IGNORE,
                ));

                // Solda [-] butonu
                button(
                    sbar,
                    ui,
                    &skins.secondary,
                    BUTTON_BORDER,
                    "-",
                    Vec2::new(28.0, 26.0),
                    down_action,
                    Some("Oyun hızını azalt"),
                );

                // Ortada 5 adet '-' şeklinde yatay hız çizgisi (aktif olanlar Primary sarı/altın)
                sbar.spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    ..default()
                })
                .with_children(|pips| {
                    for idx in 1..=5 {
                        let img = if idx <= init_speed {
                            &ui.pip_on
                        } else {
                            &ui.pip_off
                        };
                        pips.spawn((
                            SpeedPip(idx),
                            Node {
                                width: Val::Px(22.0),
                                height: Val::Px(5.0),
                                ..default()
                            },
                            ImageNode::new(img.clone()),
                            Pickable::IGNORE,
                        ));
                    }
                });

                // Sağda [+] butonu
                button(
                    sbar,
                    ui,
                    &skins.secondary,
                    BUTTON_BORDER,
                    "+",
                    Vec2::new(28.0, 26.0),
                    up_action,
                    Some("Oyun hızını artır"),
                );
            });
    });
}

pub fn hud_item_visuals(
    mut q: Query<(&Interaction, &mut BackgroundColor), (With<HudHoverItem>, Changed<Interaction>)>,
) {
    for (interaction, mut bg) in &mut q {
        *bg = BackgroundColor(match interaction {
            Interaction::None => Color::NONE,
            Interaction::Hovered => Color::srgba(0.0, 0.0, 0.0, 0.38),
            Interaction::Pressed => Color::srgba(0.0, 0.0, 0.0, 0.55),
        });
    }
}

pub fn handle_hud_item_pointer_over(
    trigger: On<PointerOver>,
    mut q: Query<&mut BackgroundColor, With<HudHoverItem>>,
) {
    if let Ok(mut bg) = q.get_mut(trigger.entity) {
        *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.38));
    }
}

pub fn handle_hud_item_pointer_out(
    trigger: On<PointerOut>,
    mut q: Query<&mut BackgroundColor, With<HudHoverItem>>,
) {
    if let Ok(mut bg) = q.get_mut(trigger.entity) {
        *bg = BackgroundColor(Color::NONE);
    }
}

pub fn handle_hud_item_pointer_press(
    trigger: On<PointerPress>,
    mut q: Query<&mut BackgroundColor, With<HudHoverItem>>,
) {
    if let Ok(mut bg) = q.get_mut(trigger.entity) {
        *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55));
    }
}

pub fn handle_hud_item_pointer_release(
    trigger: On<PointerRelease>,
    mut q: Query<&mut BackgroundColor, With<HudHoverItem>>,
) {
    if let Ok(mut bg) = q.get_mut(trigger.entity) {
        *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.38));
    }
}

/// İstatistik / Metrik gösterge kutucuğu (`inset.png` 9-slice çerçeveli).
pub fn spawn_stat_card<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    label_text: impl Into<String>,
    value_text: impl Into<String>,
) -> EntityCommands<'a> {
    let lbl_str = label_text.into();
    let val_str = value_text.into();
    let is_positive = val_str.starts_with('+');
    let val_color = if is_positive {
        theme.colors.success
    } else {
        GOLD_TEXT
    };

    let mut card = parent.spawn((
        Node {
            flex_grow: 1.0,
            flex_basis: Val::Px(0.0),
            padding: UiRect::axes(Space::SM.val(&theme.spacing), Space::XS.val(&theme.spacing)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(3.0),
            position_type: PositionType::Relative,
            ..default()
        },
        DeferredInsetFrame,
    ));

    card.with_children(|c| {
        c.spawn((
            Text::new(lbl_str),
            TextFont {
                font: fonts.heading.clone(),
                font_size: FontSize::Px(theme.typography.caption),
                ..default()
            },
            TextColor(DIM_TEXT),
            Pickable::IGNORE,
        ));
        c.spawn((
            Text::new(val_str),
            TextFont {
                font: fonts.heading.clone(),
                font_size: FontSize::Px(theme.typography.title),
                ..default()
            },
            TextColor(val_color),
            Pickable::IGNORE,
        ));
    });

    card
}

#[derive(Component)]
pub struct DeferredInsetFrame;

pub fn sync_deferred_inset_frames(
    mut commands: Commands,
    ui: Option<Res<Ui>>,
    q: Query<Entity, With<DeferredInsetFrame>>,
) {
    let Some(ui) = ui else { return };
    for entity in &q {
        commands
            .entity(entity)
            .insert(crate::ui::components::panels::sliced(&ui.inset, INSET_BORDER))
            .remove::<DeferredInsetFrame>();
    }
}

pub fn spawn_badge_chip<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    text_str: impl Into<String>,
    tone: TextTone,
) -> EntityCommands<'a> {
    let (bg, border) = match tone {
        TextTone::Accent => (
            Color::srgba(0.851, 0.647, 0.290, 0.20),
            theme.colors.border_accent,
        ),
        TextTone::Success => (
            Color::srgba(0.220, 0.631, 0.412, 0.20),
            theme.colors.success,
        ),
        TextTone::Danger => (
            Color::srgba(0.898, 0.243, 0.243, 0.20),
            theme.colors.danger,
        ),
        _ => (theme.colors.surface_raised, theme.colors.border_subtle),
    };

    let mut chip = parent.spawn((
        Node {
            padding: UiRect::axes(Space::XS.val(&theme.spacing), Space::XXS.val(&theme.spacing)),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(border),
        UiChip,
    ));

    chip.with_children(|c| {
        let t = text_bundle(theme, fonts, text_str, TextVariant::Caption, tone);
        c.spawn(t);
    });

    chip
}

pub fn flag_slot_node(
    theme: &Theme,
    width: f32,
    height: f32,
) -> (Node, BackgroundColor, BorderColor) {
    (
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(theme.colors.surface_raised),
        BorderColor::all(theme.colors.border_default),
    )
}
