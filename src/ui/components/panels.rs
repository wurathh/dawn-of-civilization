#![allow(deprecated)]

use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::components::text::text;
use crate::ui::theme::{
    GOLD_TEXT, HEADER_BORDER, HEAD_BOTTOM, HEAD_H, HEAD_SIDE, HEAD_TOP_SLICE, HEAD_2X_H,
    HEAD_2X_TOP_SLICE, INSET_BORDER, PANEL_BORDER, TEXTURE_INSET, Theme, Ui,
};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::ui::gradients::{BackgroundGradient, BorderGradient, LinearGradient};
use bevy::ui::widget::NodeImageMode;

pub const PANEL_ALPHA: f32 = 0.90;
pub const TEXTURE_ALPHA: f32 = 0.85;

pub fn slicer(border: f32) -> NodeImageMode {
    NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect::all(border),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    })
}

pub fn sliced(img: &Handle<Image>, border: f32) -> ImageNode {
    ImageNode::new(img.clone())
        .with_mode(slicer(border))
        .with_color(Color::srgba(1.0, 1.0, 1.0, PANEL_ALPHA))
}

/// 14px yüksekliğindeki yatay çubuklar (slider/progress) için:
/// Üst ve alt sınır 3px (toplam 6px < 14px), sol ve sağ 6px; böylece dikeyde doku hiç ezilmez.
pub fn sliced_bar(img: &Handle<Image>) -> ImageNode {
    ImageNode::new(img.clone()).with_mode(NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect {
            min_inset: Vec2::new(6.0, 3.0),
            max_inset: Vec2::new(6.0, 3.0),
        },
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    }))
}

pub fn headed_slicer() -> NodeImageMode {
    NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect {
            min_inset: Vec2::new(HEAD_SIDE, HEAD_TOP_SLICE),
            max_inset: Vec2::new(HEAD_SIDE, HEAD_BOTTOM),
        },
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    })
}

pub fn headed_2x_slicer() -> NodeImageMode {
    NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect {
            min_inset: Vec2::new(HEAD_SIDE, HEAD_2X_TOP_SLICE),
            max_inset: Vec2::new(HEAD_SIDE, HEAD_BOTTOM),
        },
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    })
}

pub fn ribbon_slicer() -> NodeImageMode {
    NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect {
            min_inset: Vec2::new(8.0, 0.0),
            max_inset: Vec2::new(22.0, 10.0),
        },
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    })
}

pub fn vertical_slicer() -> NodeImageMode {
    NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect {
            min_inset: Vec2::new(6.0, 6.0),
            max_inset: Vec2::new(10.0, 16.0),
        },
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    })
}

pub fn bottom_slicer() -> NodeImageMode {
    NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect {
            min_inset: Vec2::new(24.0, 10.0),
            max_inset: Vec2::new(24.0, 0.0),
        },
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    })
}

pub fn date_slicer() -> NodeImageMode {
    NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect {
            min_inset: Vec2::new(20.0, 9.0),
            max_inset: Vec2::new(20.0, 0.0),
        },
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    })
}

pub fn frame(p: &mut ChildSpawnerCommands, img: &Handle<Image>, border: f32) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        sliced(img, border),
        FocusPolicy::Pass,
        Pickable::IGNORE,
    ));
}

pub fn texture_overlay(p: &mut ChildSpawnerCommands, texture: &Handle<Image>) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(TEXTURE_INSET),
            right: Val::Px(TEXTURE_INSET),
            top: Val::Px(TEXTURE_INSET),
            bottom: Val::Px(TEXTURE_INSET),
            overflow: Overflow::clip(),
            ..default()
        },
        FocusPolicy::Pass,
        Pickable::IGNORE,
    ))
    .with_child((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(texture.clone())
            .with_mode(NodeImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            })
            .with_color(Color::srgba(1.0, 1.0, 1.0, TEXTURE_ALPHA)),
        FocusPolicy::Pass,
        Pickable::IGNORE,
    ));
}

pub fn tall_headed_panel<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    header_h: f32,
    size: (Val, Val),
    body_padding: UiRect,
    row_gap: Val,
    header: impl FnOnce(&mut ChildSpawnerCommands),
    body: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut cmd = p.spawn((
        Node {
            width: size.0,
            height: size.1,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            ..default()
        },
        FocusPolicy::Block,
    ));
    cmd.with_children(|panel| {
        // 1. Bağımsız Üst Başlık Çubuğu: İstenen yükseklikte (örneğin 60-64px) yukarı doğru serbestçe genişleyebilir
        panel
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(header_h),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    flex_shrink: 0.0,
                    margin: UiRect::bottom(Val::Px(-6.0)),
                    ..default()
                },
                sliced(&ui.header, HEADER_BORDER),
                ZIndex(2),
                Pickable::IGNORE,
            ))
            .with_children(header);

        // 2. Panel Gövdesi
        panel
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    position_type: PositionType::Relative,
                    overflow: Overflow::clip(),
                    ..default()
                },
                sliced(&ui.panel, PANEL_BORDER),
                FocusPolicy::Pass,
            ))
            .with_children(|body_panel| {
                // Dokulu arka plan katmanı
                body_panel
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(TEXTURE_INSET),
                            right: Val::Px(TEXTURE_INSET),
                            top: Val::Px(TEXTURE_INSET),
                            bottom: Val::Px(TEXTURE_INSET),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        FocusPolicy::Pass,
                        Pickable::IGNORE,
                    ))
                    .with_child((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ImageNode::new(ui.texture.clone())
                            .with_mode(NodeImageMode::Tiled {
                                tile_x: true,
                                tile_y: true,
                                stretch_value: 1.0,
                            })
                            .with_color(Color::srgba(1.0, 1.0, 1.0, TEXTURE_ALPHA)),
                        FocusPolicy::Pass,
                        Pickable::IGNORE,
                    ));

                // Butonlar / içerik alanı
                body_panel
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: body_padding,
                        row_gap,
                        ..default()
                    })
                    .with_children(body);
            });
    });
    cmd
}

pub fn headed_2x_panel_custom<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    size: (Val, Val),
    body_padding: UiRect,
    row_gap: Val,
    header: impl FnOnce(&mut ChildSpawnerCommands),
    body: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut cmd = p.spawn((
        Node {
            width: size.0,
            height: size.1,
            flex_shrink: 0.0,
            position_type: PositionType::Relative,
            ..default()
        },
        FocusPolicy::Block,
    ));
    cmd.with_children(|panel| {
        panel.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(ui.headed_2x.clone())
                .with_mode(headed_2x_slicer())
                .with_color(Color::srgba(1.0, 1.0, 1.0, PANEL_ALPHA)),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
        panel
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(TEXTURE_INSET),
                    right: Val::Px(TEXTURE_INSET),
                    top: Val::Px(HEAD_2X_H + 2.0),
                    bottom: Val::Px(TEXTURE_INSET),
                    overflow: Overflow::clip(),
                    ..default()
                },
                FocusPolicy::Pass,
                Pickable::IGNORE,
            ))
            .with_child((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ImageNode::new(ui.texture.clone())
                    .with_mode(NodeImageMode::Tiled {
                        tile_x: true,
                        tile_y: true,
                        stretch_value: 1.0,
                    })
                    .with_color(Color::srgba(1.0, 1.0, 1.0, TEXTURE_ALPHA)),
                FocusPolicy::Pass,
                Pickable::IGNORE,
            ));
        panel
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(HEAD_2X_H),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(0.0),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .with_children(header);
        panel
            .spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(HEAD_2X_TOP_SLICE),
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: body_padding,
                row_gap,
                ..default()
            })
            .with_children(body);
    });
    cmd
}

pub fn headed_panel_custom<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    img: &Handle<Image>,
    header_h: f32,
    size: (Val, Val),
    header: impl FnOnce(&mut ChildSpawnerCommands),
    body: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    headed_panel_custom_spaced(
        p,
        ui,
        img,
        header_h,
        size,
        UiRect::new(
            Val::Px(HEAD_SIDE),
            Val::Px(HEAD_SIDE),
            Val::Px(10.0),
            Val::Px(22.0),
        ),
        Val::Px(10.0),
        header,
        body,
    )
}

pub fn headed_panel_custom_spaced<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    img: &Handle<Image>,
    header_h: f32,
    size: (Val, Val),
    body_padding: UiRect,
    row_gap: Val,
    header: impl FnOnce(&mut ChildSpawnerCommands),
    body: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut cmd = p.spawn((
        Node {
            width: size.0,
            height: size.1,
            flex_shrink: 0.0,
            position_type: PositionType::Relative,
            ..default()
        },
        FocusPolicy::Block,
    ));
    cmd.with_children(|panel| {
        panel.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(img.clone())
                .with_mode(headed_slicer())
                .with_color(Color::srgba(1.0, 1.0, 1.0, PANEL_ALPHA)),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
        panel
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(TEXTURE_INSET),
                    right: Val::Px(TEXTURE_INSET),
                    top: Val::Px(header_h + 2.0),
                    bottom: Val::Px(TEXTURE_INSET),
                    overflow: Overflow::clip(),
                    ..default()
                },
                FocusPolicy::Pass,
                Pickable::IGNORE,
            ))
            .with_child((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ImageNode::new(ui.texture.clone())
                    .with_mode(NodeImageMode::Tiled {
                        tile_x: true,
                        tile_y: true,
                        stretch_value: 1.0,
                    })
                    .with_color(Color::srgba(1.0, 1.0, 1.0, TEXTURE_ALPHA)),
                FocusPolicy::Pass,
                Pickable::IGNORE,
            ));
        panel
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(header_h),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(0.0),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .with_children(header);
        panel
            .spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(header_h.max(HEAD_TOP_SLICE)),
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: body_padding,
                row_gap,
                ..default()
            })
            .with_children(body);
    });
    cmd
}

pub fn headed_panel<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    img: &Handle<Image>,
    title: &str,
    size: (Val, Val),
    body: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    headed_panel_custom(
        p,
        ui,
        img,
        HEAD_H,
        size,
        |h| text(h, ui, title, 20.0, GOLD_TEXT),
        body,
    )
}

pub fn plain_panel<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    img: &Handle<Image>,
    border: f32,
    size: (Val, Val),
    body: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut cmd = p.spawn((
        Node {
            width: size.0,
            height: size.1,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(border + 4.0)),
            row_gap: Val::Px(10.0),
            flex_shrink: 0.0,
            position_type: PositionType::Relative,
            ..default()
        },
        FocusPolicy::Block,
    ));
    cmd.with_children(|panel| {
        frame(panel, img, border);
        texture_overlay(panel, &ui.texture);
        body(panel);
    });
    cmd
}

pub fn inset_panel<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    size: (Val, Val),
    padding: UiRect,
    body: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut cmd = p.spawn(Node {
        width: size.0,
        height: size.1,
        flex_direction: FlexDirection::Column,
        padding,
        row_gap: Val::Px(6.0),
        position_type: PositionType::Relative,
        ..default()
    });
    cmd.with_children(|ins| {
        frame(ins, &ui.inset, INSET_BORDER);
        body(ins);
    });
    cmd
}

pub fn header_bar_with(p: &mut ChildSpawnerCommands, ui: &Ui, img: &Handle<Image>, title: &str) {
    p.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_shrink: 0.0,
            ..default()
        },
        sliced(img, HEADER_BORDER),
        Pickable::IGNORE,
    ))
    .with_children(|h| text(h, ui, title, 19.0, GOLD_TEXT));
}

pub fn header_bar(p: &mut ChildSpawnerCommands, ui: &Ui, title: &str) {
    header_bar_with(p, ui, &ui.header, title);
}

pub fn divider(p: &mut ChildSpawnerCommands, ui: &Ui) {
    p.spawn((
        Node {
            width: Val::Px(220.0),
            height: Val::Px(12.0),
            flex_shrink: 0.0,
            ..default()
        },
        ImageNode::new(ui.divider.clone()),
        Pickable::IGNORE,
    ));
}

pub fn divider_width(p: &mut ChildSpawnerCommands, ui: &Ui, width: Val) {
    p.spawn((
        Node {
            width,
            height: Val::Px(12.0),
            flex_shrink: 0.0,
            ..default()
        },
        ImageNode::new(ui.divider.clone()),
        Pickable::IGNORE,
    ));
}

pub fn vsep(p: &mut ChildSpawnerCommands, ui: &Ui, height: Val) {
    p.spawn((
        Node {
            width: Val::Px(2.0),
            height,
            flex_shrink: 0.0,
            ..default()
        },
        ImageNode::new(ui.vsep.clone()),
        Pickable::IGNORE,
    ));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PanelElevation {
    #[default]
    Flat,
    Raised,
    Overlay,
}

#[derive(Component, Clone, Debug)]
pub struct UiPanel {
    pub elevation: PanelElevation,
}

pub fn panel_gradients(elevation: PanelElevation) -> (BackgroundGradient, BorderGradient) {
    match elevation {
        PanelElevation::Raised => (
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::new(0.11, 0.14, 0.19, 0.97).into(),
                Srgba::new(0.06, 0.08, 0.12, 0.95).into(),
            ])),
            BorderGradient::from(LinearGradient::to_bottom(vec![
                Srgba::new(0.92, 0.78, 0.45, 0.95).into(),
                Srgba::new(0.55, 0.42, 0.20, 0.80).into(),
            ])),
        ),
        PanelElevation::Flat => (
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::new(0.08, 0.11, 0.15, 0.95).into(),
                Srgba::new(0.05, 0.07, 0.10, 0.92).into(),
            ])),
            BorderGradient::from(LinearGradient::to_bottom(vec![
                Srgba::new(0.45, 0.48, 0.55, 0.85).into(),
                Srgba::new(0.22, 0.25, 0.30, 0.70).into(),
            ])),
        ),
        PanelElevation::Overlay => (
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::new(0.06, 0.08, 0.12, 0.98).into(),
                Srgba::new(0.03, 0.04, 0.07, 0.96).into(),
            ])),
            BorderGradient::from(LinearGradient::to_bottom(vec![
                Srgba::new(0.75, 0.62, 0.35, 0.90).into(),
                Srgba::new(0.35, 0.28, 0.15, 0.75).into(),
            ])),
        ),
    }
}

pub fn panel_node(
    theme: &Theme,
    elevation: PanelElevation,
    padding: Space,
) -> (Node, BackgroundGradient, BorderGradient, UiPanel) {
    let (bg_grad, border_grad) = panel_gradients(elevation);

    (
        Node {
            padding: UiRect::all(padding.val(&theme.spacing)),
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
            flex_direction: FlexDirection::Column,
            row_gap: Space::SM.val(&theme.spacing),
            position_type: PositionType::Relative,
            overflow: Overflow::clip(),
            ..default()
        },
        bg_grad,
        border_grad,
        UiPanel { elevation },
    )
}

pub fn spawn_panel_inner_glow(_parent: &mut ChildSpawnerCommands, _radius: f32) {}

pub fn spawn_panel_frosted_backdrop(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    radius: f32,
) {
    parent.spawn((
        ImageNode {
            image: asset_server.load("ui/texture_diagonal_tile.png"),
            image_mode: NodeImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border_radius: BorderRadius::all(Val::Px(radius)),
            ..default()
        },
        Pickable::IGNORE,
    ));
}

pub fn spawn_gold_divider(parent: &mut ChildSpawnerCommands, _theme: &Theme, width: Val) {
    parent.spawn((
        Node {
            width,
            height: Val::Px(1.5),
            margin: UiRect::vertical(Val::Px(6.0)),
            ..default()
        },
        BackgroundGradient::from(LinearGradient::to_right(vec![
            Srgba::new(0.85, 0.65, 0.20, 0.05).into(),
            Srgba::new(0.98, 0.85, 0.45, 0.95).into(),
            Srgba::new(0.85, 0.65, 0.20, 0.05).into(),
        ])),
        Pickable::IGNORE,
    ));
}

pub fn spawn_panel_header<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    title_str: impl Into<String>,
    subtitle_opt: Option<impl Into<String>>,
    is_gold: bool,
) {
    let (hdr_bg_grad, border_bot_color, title_color) = if is_gold {
        (
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("3A2C1B").unwrap().into(),
                Srgba::hex("1E160D").unwrap().into(),
            ])),
            Color::srgba(0.82, 0.68, 0.36, 0.55),
            GOLD_TEXT,
        )
    } else {
        (
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("242C38").unwrap().into(),
                Srgba::hex("151A22").unwrap().into(),
            ])),
            Color::srgba(0.55, 0.60, 0.68, 0.35),
            GOLD_TEXT,
        )
    };

    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::axes(Space::MD.val(&theme.spacing), Space::SM.val(&theme.spacing)),
                border: UiRect::bottom(Val::Px(1.5)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(3.0),
                position_type: PositionType::Relative,
                border_radius: BorderRadius::px(theme.radius.lg, theme.radius.lg, 0.0, 0.0),
                overflow: Overflow::clip(),
                ..default()
            },
            hdr_bg_grad,
            BorderColor::all(border_bot_color),
        ))
        .with_children(|hdr| {
            hdr.spawn((
                Text::new(title_str.into()),
                TextFont {
                    font: fonts.heading.clone(),
                    font_size: FontSize::Px(theme.typography.title_lg()),
                    ..default()
                },
                TextColor(title_color),
                Pickable::IGNORE,
            ));

            if let Some(sub) = subtitle_opt {
                hdr.spawn((
                    Text::new(sub.into()),
                    TextFont {
                        font: fonts.medium.clone(),
                        font_size: FontSize::Px(theme.typography.body_sm()),
                        ..default()
                    },
                    TextColor(theme.colors.text_secondary),
                    Pickable::IGNORE,
                ));
            }
        });
}

pub fn spawn_card_with_header<'a, F>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    title: impl Into<String>,
    elevation: PanelElevation,
    content_builder: F,
) -> EntityCommands<'a>
where
    F: FnOnce(&mut ChildSpawnerCommands),
{
    let (bg_grad, border_grad) = panel_gradients(elevation);
    let title_str = title.into();

    let mut card = parent.spawn((
        Node {
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Relative,
            overflow: Overflow::clip(),
            ..default()
        },
        bg_grad,
        border_grad,
        UiPanel { elevation },
    ));

    card.with_children(|c| {
        spawn_panel_header(
            c,
            theme,
            fonts,
            title_str,
            None::<String>,
            elevation == PanelElevation::Raised,
        );
        c.spawn(Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Space::MD.val(&theme.spacing)),
            flex_direction: FlexDirection::Column,
            row_gap: Space::SM.val(&theme.spacing),
            ..default()
        })
        .with_children(content_builder);
    });

    card
}

pub fn spawn_ornate_card<'a, F>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    title: impl Into<String>,
    subtitle: Option<impl Into<String>>,
    content_builder: F,
) -> EntityCommands<'a>
where
    F: FnOnce(&mut ChildSpawnerCommands),
{
    let (bg_grad, border_grad) = panel_gradients(PanelElevation::Raised);
    let title_str = title.into();
    let sub_str = subtitle.map(|s| s.into());

    let mut card = parent.spawn((
        Node {
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Relative,
            overflow: Overflow::clip(),
            ..default()
        },
        bg_grad,
        border_grad,
        UiPanel {
            elevation: PanelElevation::Raised,
        },
    ));

    card.with_children(|c| {
        spawn_panel_header(c, theme, fonts, title_str, sub_str, true);
        c.spawn(Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Space::MD.val(&theme.spacing)),
            flex_direction: FlexDirection::Column,
            row_gap: Space::MD.val(&theme.spacing),
            ..default()
        })
        .with_children(content_builder);
    });

    card
}

pub fn spawn_slate_panel<'a, F>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    content_builder: F,
) -> EntityCommands<'a>
where
    F: FnOnce(&mut ChildSpawnerCommands),
{
    let (bg_grad, border_grad) = panel_gradients(PanelElevation::Flat);

    let mut card = parent.spawn((
        Node {
            padding: UiRect::all(Space::MD.val(&theme.spacing)),
            flex_direction: FlexDirection::Column,
            row_gap: Space::SM.val(&theme.spacing),
            position_type: PositionType::Relative,
            overflow: Overflow::clip(),
            border: UiRect::all(Val::Px(1.2)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
            ..default()
        },
        bg_grad,
        border_grad,
        UiPanel {
            elevation: PanelElevation::Flat,
        },
    ));

    card.with_children(|c| {
        content_builder(c);
    });

    card
}
