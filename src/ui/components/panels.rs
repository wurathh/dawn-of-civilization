use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::theme::Theme;
use bevy::prelude::*;
use bevy::ui::gradients::{BackgroundGradient, BorderGradient, LinearGradient};

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

/// Panel için yukarıdan aşağıya Bevy gradyanları (Obsidyen / Arduvaz derinliği ve metalik kenarlık)
pub fn panel_gradients(elevation: PanelElevation) -> (BackgroundGradient, BorderGradient) {
    match elevation {
        PanelElevation::Raised => (
            // Lüks Koyu Obsidyen Derinlik Gradyanı
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("121722").unwrap().into(),
                Srgba::hex("070A0F").unwrap().into(),
            ])),
            // Parlak Altın Metal Kenarlık Gradyanı
            BorderGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("E5C068").unwrap().into(),
                Srgba::hex("7B5919").unwrap().into(),
            ])),
        ),
        PanelElevation::Flat => (
            // Mat Koyu Çelik / Slate Gradyanı
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("141B26").unwrap().into(),
                Srgba::hex("0A0E15").unwrap().into(),
            ])),
            // İnce Çelik Kenarlık Gradyanı
            BorderGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("485A75").unwrap().into(),
                Srgba::hex("202938").unwrap().into(),
            ])),
        ),
        PanelElevation::Overlay => (
            // Yarı Saydam Koyu Menü / Modal Gradyanı
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("1A2230").unwrap().into(),
                Srgba::hex("0D131C").unwrap().into(),
            ])),
            // İnce Işıltılı Kenarlık Gradyanı
            BorderGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("6B82A6").unwrap().into(),
                Srgba::hex("2A3648").unwrap().into(),
            ])),
        ),
    }
}

/// Standart panel node bundle'ı (Gradyan Arka Plan ve Kenarlık ile).
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

/// Panel tepesine vuran zarif üst ışık çizgisi efekti (Sadece üstte, yanlarda veya altta rahatsız edici çizgi yok)
pub fn spawn_panel_inner_glow(parent: &mut ChildSpawnerCommands, radius: f32) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            border: UiRect {
                top: Val::Px(1.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
            },
            border_radius: BorderRadius::px(radius, radius, 0.0, 0.0),
            ..default()
        },
        BorderColor {
            top: Color::srgba(1.0, 1.0, 1.0, 0.15),
            bottom: Color::NONE,
            left: Color::NONE,
            right: Color::NONE,
        },
        Pickable::IGNORE,
    ));
}

/// Merkezden kenarlara doğru sönen, altın parıltılı lüks ayraç çizgisi
pub fn spawn_gold_divider(
    parent: &mut ChildSpawnerCommands,
    _theme: &Theme,
    width: Val,
) {
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

/// Panel üstü için zengin, gradyanlı, alt gölgeli ve parıltılı Header Başlık Bloğu
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
                Srgba::hex("241C10").unwrap().into(),
                Srgba::hex("120E08").unwrap().into(),
            ])),
            Color::srgb(0.78, 0.62, 0.26),
            Color::srgb(0.96, 0.86, 0.52),
        )
    } else {
        (
            BackgroundGradient::from(LinearGradient::to_bottom(vec![
                Srgba::hex("1C2535").unwrap().into(),
                Srgba::hex("0E131C").unwrap().into(),
            ])),
            Color::srgb(0.35, 0.45, 0.60),
            Color::srgb(0.92, 0.95, 0.98),
        )
    };

    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::axes(Space::MD.val(&theme.spacing), Space::SM.val(&theme.spacing)),
            border: UiRect::bottom(Val::Px(1.5)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(3.0),
            position_type: PositionType::Relative,
            border_radius: BorderRadius::px(theme.radius.lg, theme.radius.lg, 0.0, 0.0),
            ..default()
        },
        hdr_bg_grad,
        BorderColor::all(border_bot_color),
    ))
    .with_children(|hdr| {
        // Başlık
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

        // Varsa alt açıklama
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

        // Alt tarafa vuran yumuşak ayırıcı gölge çizgisi
        hdr.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(-2.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                height: Val::Px(2.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.40)),
            Pickable::IGNORE,
        ));
    });
}

/// Başlıklı kart/panel oluşturucu (ChildSpawnerCommands üzerinden).
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
        // 1. Panel Inner Glow & 3D Işık Yansıması
        spawn_panel_inner_glow(c, theme.radius.lg);

        // 2. Gradyanlı & Alt Gölgeli Header
        spawn_panel_header(c, theme, fonts, title_str, None::<String>, elevation == PanelElevation::Raised);

        // 3. İçerik Alanı (Padding ile sarılı)
        c.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Space::MD.val(&theme.spacing)),
                flex_direction: FlexDirection::Column,
                row_gap: Space::SM.val(&theme.spacing),
                ..default()
            },
        ))
        .with_children(|content_box| {
            content_builder(content_box);
        });
    });

    card
}

/// Saf Rust & Bevy primitifleriyle %100 OPAK lüks altın çerçeveli kart/panel.
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
        UiPanel { elevation: PanelElevation::Raised },
    ));

    card.with_children(|c| {
        // 1. Panel Inner Glow
        spawn_panel_inner_glow(c, theme.radius.lg);

        // 2. Özel Lüks Altın Header
        spawn_panel_header(c, theme, fonts, title_str, sub_str, true);

        // 3. İçerik Bloğu
        c.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Space::MD.val(&theme.spacing)),
                flex_direction: FlexDirection::Column,
                row_gap: Space::MD.val(&theme.spacing),
                ..default()
            },
        ))
        .with_children(|content_box| {
            content_builder(content_box);
        });
    });

    card
}

/// Saf Rust & Bevy primitifleriyle %100 OPAK koyu çelik kart/panel.
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
        UiPanel { elevation: PanelElevation::Flat },
    ));

    card.with_children(|c| {
        spawn_panel_inner_glow(c, theme.radius.lg);
        content_builder(c);
    });

    card
}
