use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::components::text::{text_bundle, TextTone, TextVariant};
use crate::ui::theme::Theme;
use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::ui::gradients::{BackgroundGradient, BorderGradient, LinearGradient};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Royal,
    Secondary,
    Ghost,
    Danger,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonStyle {
    #[default]
    Solid,
    Card,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonState {
    #[default]
    Normal,
    Hovered,
    Pressed,
}

#[derive(Component, Clone, Debug)]
pub struct UiButton {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub style: ButtonStyle,
    pub disabled: bool,
    pub focused: bool,
}

impl Default for UiButton {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            style: ButtonStyle::Solid,
            disabled: false,
            focused: false,
        }
    }
}

/// Butonun iç gölge ve 3D kabartma derinlik katmanı (Inner Shadow)
#[derive(Component)]
pub struct UiButtonInnerShadow;

/// Butonun ana başlık / metin işaretleyicisi
#[derive(Component)]
pub struct UiButtonText;

/// Butonun alt açıklama metni işaretleyicisi
#[derive(Component)]
pub struct UiButtonSubtitle;

/// Butonun solundaki ikon rozet kutusu
#[derive(Component)]
pub struct UiButtonBadge;

/// Butonun rozet içi sembol metni
#[derive(Component)]
pub struct UiButtonBadgeSymbol;

/// Butonun rozet içi görsel imajı (Örn: Altın 3D Play Mücevheri)
#[derive(Component)]
pub struct UiButtonBadgeImage {
    pub normal: Handle<Image>,
    pub hover: Handle<Image>,
    pub pressed: Handle<Image>,
}

/// Buton varyantlarına ve durumlarına göre Yukardan Aşağıya Gradyanlar ve Kenarlık Gradyanları
/// Not: Pressed durumunda ASLA ters (reverse) gradyan yapılmaz; fiziksel olarak basılmış hissi vermek için
/// aynı yönlü (üstten alta) daha koyu/doygun tonlar ve üstten hafif gölge/kararma kullanılır.
pub fn button_gradients(variant: ButtonVariant, state: ButtonState) -> (BackgroundGradient, BorderGradient) {
    match variant {
        ButtonVariant::Primary => match state {
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("F4C96B").unwrap().into(),
                    Srgba::hex("B07A26").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("FFF1C9").unwrap().into(),
                    Srgba::hex("8E6221").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("FFDB88").unwrap().into(),
                    Srgba::hex("CF9533").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("FFFAE6").unwrap().into(),
                    Srgba::hex("A87428").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("DDAE55").unwrap().into(),
                    Srgba::hex("9A6620").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("EBCF90").unwrap().into(),
                    Srgba::hex("7A5219").unwrap().into(),
                ])),
            ),
        },
        ButtonVariant::Royal => match state {
            // Gece Mavisi / Asil Çelik Gövde + Zarif Altın Kenarlık
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("1B263B").unwrap().into(),
                    Srgba::hex("0D131F").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("D4AF37").unwrap().into(),
                    Srgba::hex("7B5919").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("273754").unwrap().into(),
                    Srgba::hex("141D2E").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("F6E27A").unwrap().into(),
                    Srgba::hex("9C7123").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("141D2D").unwrap().into(),
                    Srgba::hex("090D15").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("BA9726").unwrap().into(),
                    Srgba::hex("5C4212").unwrap().into(),
                ])),
            ),
        },
        ButtonVariant::Secondary => match state {
            // Slate / Antrasit Çelik Gövde + Gümüş/Çelik Kenarlık
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("27303E").unwrap().into(),
                    Srgba::hex("161B24").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("566B88").unwrap().into(),
                    Srgba::hex("263242").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("344256").unwrap().into(),
                    Srgba::hex("1F2734").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("7B93B5").unwrap().into(),
                    Srgba::hex("38485C").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("1F2633").unwrap().into(),
                    Srgba::hex("10141C").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("43546B").unwrap().into(),
                    Srgba::hex("1C2532").unwrap().into(),
                ])),
            ),
        },
        ButtonVariant::Danger => match state {
            // Yakut / Bordo Kızılı Gövde + Parlak Kırmızı Kenarlık
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("8B1E22").unwrap().into(),
                    Srgba::hex("540D10").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("E55353").unwrap().into(),
                    Srgba::hex("781518").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("A8272C").unwrap().into(),
                    Srgba::hex("6B1216").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("FF7373").unwrap().into(),
                    Srgba::hex("961B1F").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("72161A").unwrap().into(),
                    Srgba::hex("41070A").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("B83A3A").unwrap().into(),
                    Srgba::hex("540E10").unwrap().into(),
                ])),
            ),
        },
        ButtonVariant::Ghost => match state {
            // Ghost butonda hafif yarı saydam derinlik gradyanı
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::new(0.0, 0.0, 0.0, 0.0).into(),
                    Srgba::new(0.0, 0.0, 0.0, 0.0).into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::new(1.0, 1.0, 1.0, 0.0).into(),
                    Srgba::new(1.0, 1.0, 1.0, 0.0).into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::new(1.0, 1.0, 1.0, 0.10).into(),
                    Srgba::new(1.0, 1.0, 1.0, 0.04).into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::new(1.0, 1.0, 1.0, 0.28).into(),
                    Srgba::new(1.0, 1.0, 1.0, 0.12).into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::new(1.0, 1.0, 1.0, 0.05).into(),
                    Srgba::new(1.0, 1.0, 1.0, 0.02).into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::new(1.0, 1.0, 1.0, 0.18).into(),
                    Srgba::new(1.0, 1.0, 1.0, 0.08).into(),
                ])),
            ),
        },
    }
}

pub fn primary_button_gradients(state: ButtonState) -> (BackgroundGradient, BorderGradient) {
    button_gradients(ButtonVariant::Primary, state)
}

/// Varyanta ve duruma göre Inner Shadow BorderColor (Üst ışık çizgisi, alt gölge çizgisi)
pub fn inner_shadow_colors_for_variant(variant: ButtonVariant, state: ButtonState) -> BorderColor {
    match variant {
        ButtonVariant::Primary => inner_shadow_colors(state),
        ButtonVariant::Royal => match state {
            ButtonState::Normal => BorderColor {
                top: Color::srgba(0.70, 0.82, 1.0, 0.35),
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.45),
                left: Color::srgba(0.70, 0.82, 1.0, 0.15),
                right: Color::srgba(0.0, 0.0, 0.0, 0.25),
            },
            ButtonState::Hovered => BorderColor {
                top: Color::srgba(0.85, 0.92, 1.0, 0.50),
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.45),
                left: Color::srgba(0.85, 0.92, 1.0, 0.20),
                right: Color::srgba(0.0, 0.0, 0.0, 0.25),
            },
            ButtonState::Pressed => BorderColor {
                top: Color::srgba(0.45, 0.65, 0.95, 0.28), // Işık çizgisi kaybolmaz, gövdeye uygun koyulaşmış mavi-çelik tona geçer
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.35),
                left: Color::srgba(0.45, 0.65, 0.95, 0.12),
                right: Color::srgba(0.0, 0.0, 0.0, 0.20),
            },
        },
        ButtonVariant::Secondary => match state {
            ButtonState::Normal => BorderColor {
                top: Color::srgba(1.0, 1.0, 1.0, 0.20),
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.40),
                left: Color::srgba(1.0, 1.0, 1.0, 0.08),
                right: Color::srgba(0.0, 0.0, 0.0, 0.20),
            },
            ButtonState::Hovered => BorderColor {
                top: Color::srgba(1.0, 1.0, 1.0, 0.35),
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.40),
                left: Color::srgba(1.0, 1.0, 1.0, 0.15),
                right: Color::srgba(0.0, 0.0, 0.0, 0.20),
            },
            ButtonState::Pressed => BorderColor {
                top: Color::srgba(0.70, 0.80, 0.95, 0.18), // Işık çizgisi kaybolmaz, mat çelik/gümüş tonda parlar
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.30),
                left: Color::srgba(0.70, 0.80, 0.95, 0.08),
                right: Color::srgba(0.0, 0.0, 0.0, 0.18),
            },
        },
        ButtonVariant::Danger => match state {
            ButtonState::Normal => BorderColor {
                top: Color::srgba(1.0, 0.65, 0.65, 0.35),
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.45),
                left: Color::srgba(1.0, 0.65, 0.65, 0.15),
                right: Color::srgba(0.0, 0.0, 0.0, 0.25),
            },
            ButtonState::Hovered => BorderColor {
                top: Color::srgba(1.0, 0.80, 0.80, 0.50),
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.45),
                left: Color::srgba(1.0, 0.80, 0.80, 0.20),
                right: Color::srgba(0.0, 0.0, 0.0, 0.25),
            },
            ButtonState::Pressed => BorderColor {
                top: Color::srgba(1.0, 0.45, 0.45, 0.30), // Işık çizgisi kaybolmaz, koyu mercan/yakut tonda kalır
                bottom: Color::srgba(0.0, 0.0, 0.0, 0.35),
                left: Color::srgba(1.0, 0.45, 0.45, 0.12),
                right: Color::srgba(0.0, 0.0, 0.0, 0.20),
            },
        },
        ButtonVariant::Ghost => match state {
            ButtonState::Normal => BorderColor {
                top: Color::NONE,
                bottom: Color::NONE,
                left: Color::NONE,
                right: Color::NONE,
            },
            ButtonState::Hovered => BorderColor {
                top: Color::srgba(1.0, 1.0, 1.0, 0.25),
                bottom: Color::NONE,
                left: Color::srgba(1.0, 1.0, 1.0, 0.08),
                right: Color::NONE,
            },
            ButtonState::Pressed => BorderColor {
                top: Color::srgba(1.0, 1.0, 1.0, 0.14),
                bottom: Color::NONE,
                left: Color::srgba(1.0, 1.0, 1.0, 0.05),
                right: Color::NONE,
            },
        },
    }
}

/// Inner Shadow için duruma göre BorderColor (Üst ışıltı, alt derin gölge)
pub fn inner_shadow_colors(state: ButtonState) -> BorderColor {
    match state {
        ButtonState::Normal => BorderColor {
            top: Color::srgba(1.0, 0.973, 0.863, 0.55),  // rgba(255, 248, 220, 0.55)
            bottom: Color::srgba(0.0, 0.0, 0.0, 0.35),   // rgba(0, 0, 0, 0.35)
            left: Color::srgba(1.0, 0.973, 0.863, 0.20),
            right: Color::srgba(0.0, 0.0, 0.0, 0.20),
        },
        ButtonState::Hovered => BorderColor {
            top: Color::srgba(1.0, 1.0, 1.0, 0.65),      // rgba(255, 255, 255, 0.65)
            bottom: Color::srgba(0.0, 0.0, 0.0, 0.35),   // rgba(0, 0, 0, 0.35)
            left: Color::srgba(1.0, 1.0, 1.0, 0.25),
            right: Color::srgba(0.0, 0.0, 0.0, 0.25),
        },
        ButtonState::Pressed => BorderColor {
            top: Color::srgba(1.0, 0.85, 0.50, 0.35),   // Işık çizgisi kaybolmak yerine sıcak kehribar/altın parıltı tonuna geçer
            bottom: Color::srgba(0.0, 0.0, 0.0, 0.30),
            left: Color::srgba(1.0, 0.85, 0.50, 0.15),
            right: Color::srgba(0.0, 0.0, 0.0, 0.20),
        },
    }
}

/// Saf Rust & Bevy primitifleriyle ultra-keskin, vektörel buton oluşturan yardımcı fonksiyon.
pub fn spawn_button<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    text: impl Into<String>,
    variant: ButtonVariant,
    size: ButtonSize,
) -> EntityCommands<'a> {
    let content = text.into();
    let text_variant = match size {
        ButtonSize::Small => TextVariant::Caption,
        ButtonSize::Medium => TextVariant::Label,
        ButtonSize::Large => TextVariant::Title,
    };

    let (pad_h, pad_v) = match size {
        ButtonSize::Small => (Space::SM.val(&theme.spacing), Space::XXS.val(&theme.spacing)),
        ButtonSize::Medium => (Space::MD.val(&theme.spacing), Space::XS.val(&theme.spacing)),
        ButtonSize::Large => (Space::LG.val(&theme.spacing), Space::SM.val(&theme.spacing)),
    };

    let btn_comp = UiButton {
        variant,
        size,
        style: ButtonStyle::Solid,
        disabled: false,
        focused: false,
    };

    // Solid Buton: Primary, Royal, Secondary, Danger ve Ghost varyantları
    // Bevy 0.15'in BackgroundGradient ve BorderGradient özellikleri ile ultra-keskin 3D derinlikli butonlar üretir.
    let (bg_grad, border_grad) = button_gradients(variant, ButtonState::Normal);
    let shadow_border = inner_shadow_colors_for_variant(variant, ButtonState::Normal);

    let (text_color, text_tone) = match variant {
        ButtonVariant::Primary => (Color::from(Srgba::hex("2A1704").unwrap()), TextTone::Inverse),
        ButtonVariant::Royal => (Color::from(Srgba::hex("F6E27A").unwrap()), TextTone::Accent),
        ButtonVariant::Secondary => (Color::from(Srgba::hex("E2E8F0").unwrap()), TextTone::Primary),
        ButtonVariant::Danger => (Color::from(Srgba::hex("FFFFFF").unwrap()), TextTone::Primary),
        ButtonVariant::Ghost => (Color::from(Srgba::hex("CBD5E1").unwrap()), TextTone::Secondary),
    };

    let mut entity = parent.spawn((
        Node {
            padding: UiRect::axes(pad_h, pad_v),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Space::XS.val(&theme.spacing),
            border: UiRect::all(Val::Px(1.5)), // Gradyan kenarlık
            border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
            position_type: PositionType::Relative,
            overflow: Overflow::clip(),
            ..default()
        },
        bg_grad,     // Gövde gradyanı
        border_grad, // Border gradyanı
        btn_comp,
        ButtonState::Normal,
    ));

    entity.with_children(|btn| {
        // 1. Inner Shadow & 3D Bevel Katmanı (Absolute overlay)
        btn.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                border: UiRect {
                    top: Val::Px(1.0),
                    bottom: Val::Px(2.0),
                    left: Val::Px(1.0),
                    right: Val::Px(1.0),
                },
                border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
                ..default()
            },
            shadow_border,
            UiButtonInnerShadow,
            Pickable::IGNORE,
        ));

        // 2. Metin
        let text_bundle_val = text_bundle(theme, fonts, content, text_variant, text_tone);
        btn.spawn((
            text_bundle_val.0,
            text_bundle_val.1,
            TextColor(text_color),
            UiButtonText,
            Pickable::IGNORE,
        ));
    });

    entity
}

/// Zengin menü ve aksiyon kart butonu (Saf Rust UI + İkon kutusu + Başlık + Alt açıklama)
pub fn spawn_action_card_button<'a>(
    parent: &'a mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    theme: &Theme,
    fonts: &UiFonts,
    badge_symbol: &str,
    title: impl Into<String>,
    subtitle: impl Into<String>,
    variant: ButtonVariant,
) -> EntityCommands<'a> {
    let btn_comp = UiButton {
        variant,
        size: ButtonSize::Large,
        style: ButtonStyle::Card,
        disabled: false,
        focused: false,
    };

    let (bg_grad, border_grad) = action_card_gradients(variant, ButtonState::Normal);
    let shadow_border = inner_shadow_colors_for_variant(variant, ButtonState::Normal);
    let (_badge_bg, badge_sym_color, title_color, sub_color) =
        action_card_inner_colors(variant, ButtonState::Normal, false);

    let mut entity = parent.spawn((
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::axes(Space::MD.val(&theme.spacing), Space::SM.val(&theme.spacing)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Space::MD.val(&theme.spacing),
            border: UiRect::all(Val::Px(1.5)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
            position_type: PositionType::Relative,
            overflow: Overflow::clip(),
            ..default()
        },
        bg_grad,
        border_grad,
        btn_comp,
        ButtonState::Normal,
    ));

    entity.with_children(|btn| {
        // 1. Inner Shadow & Üst Işık Çizgisi
        btn.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                border: UiRect {
                    top: Val::Px(1.0),
                    bottom: Val::Px(2.0),
                    left: Val::Px(1.0),
                    right: Val::Px(1.0),
                },
                border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
                ..default()
            },
            shadow_border,
            UiButtonInnerShadow,
            Pickable::IGNORE,
        ));

        // 2. İkon Kutusu (Mühür / Rozet)
        let is_custom_icon = badge_symbol == "▶" || badge_symbol == "✕";
        let (badge_bg_grad, badge_border_grad) = action_card_badge_gradients(variant, ButtonState::Normal);
        
        btn.spawn((
            Node {
                width: Val::Px(46.0),
                height: Val::Px(46.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
                border: if is_custom_icon { UiRect::all(Val::Px(0.0)) } else { UiRect::all(Val::Px(1.5)) },
                position_type: PositionType::Relative,
                overflow: Overflow::clip(),
                ..default()
            },
            badge_bg_grad,
            badge_border_grad,
            UiButtonBadge,
            Pickable::IGNORE,
        ))
        .with_children(|icon_box| {
            if is_custom_icon {
                // Yüksek çözünürlüklü 3D Vektör İkon Görseli
                let (norm_p, hov_p, prs_p) = if badge_symbol == "▶" {
                    (
                        "ui_textures/icons/play_gem_normal.png",
                        "ui_textures/icons/play_gem_hover.png",
                        "ui_textures/icons/play_gem_pressed.png",
                    )
                } else {
                    (
                        "ui_textures/icons/exit_gem_normal.png",
                        "ui_textures/icons/exit_gem_hover.png",
                        "ui_textures/icons/exit_gem_pressed.png",
                    )
                };

                let norm_h = asset_server.load(norm_p);
                let hov_h = asset_server.load(hov_p);
                let prs_h = asset_server.load(prs_p);

                icon_box.spawn((
                    ImageNode::new(norm_h.clone()),
                    Node {
                        width: Val::Px(44.0),
                        height: Val::Px(44.0),
                        ..default()
                    },
                    UiButtonBadgeImage {
                        normal: norm_h,
                        hover: hov_h,
                        pressed: prs_h,
                    },
                    Pickable::IGNORE,
                ));
            } else {
                // Standart sembol ve üst ışık çizgisi
                icon_box.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        border: UiRect {
                            top: Val::Px(1.0),
                            bottom: Val::Px(1.5),
                            left: Val::Px(0.5),
                            right: Val::Px(0.5),
                        },
                        border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
                        ..default()
                    },
                    shadow_border,
                    UiButtonInnerShadow,
                    Pickable::IGNORE,
                ));

                icon_box.spawn((
                    Text::new(badge_symbol),
                    TextFont {
                        font: fonts.heading.clone(),
                        font_size: FontSize::Px(theme.typography.title_lg()),
                        ..default()
                    },
                    TextColor(badge_sym_color),
                    UiButtonBadgeSymbol,
                    Pickable::IGNORE,
                ));
            }
        });

        // 3. Metin Kolonu
        btn.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|text_col| {
            text_col.spawn((
                Text::new(title.into()),
                TextFont {
                    font: fonts.heading.clone(),
                    font_size: FontSize::Px(theme.typography.title_lg()),
                    ..default()
                },
                TextColor(title_color),
                UiButtonText,
                Pickable::IGNORE,
            ));
            text_col.spawn((
                Text::new(subtitle.into()),
                TextFont {
                    font: fonts.medium.clone(),
                    font_size: FontSize::Px(theme.typography.body_sm()),
                    ..default()
                },
                TextColor(sub_color),
                UiButtonSubtitle,
                Pickable::IGNORE,
            ));
        });
    });

    entity
}

/// Pointer olaylarını ButtonState bileşenine bağlayan observer sistemleri
pub fn handle_button_pointer_over(
    trigger: On<PointerOver>,
    mut q: Query<(&UiButton, &mut ButtonState)>,
) {
    if let Ok((btn, mut state)) = q.get_mut(trigger.entity) {
        if !btn.disabled && *state != ButtonState::Pressed {
            *state = ButtonState::Hovered;
        }
    }
}

pub fn handle_button_pointer_out(
    trigger: On<PointerOut>,
    mut q: Query<(&UiButton, &mut ButtonState)>,
) {
    if let Ok((btn, mut state)) = q.get_mut(trigger.entity) {
        if !btn.disabled {
            *state = ButtonState::Normal;
        }
    }
}

pub fn handle_button_pointer_press(
    trigger: On<PointerPress>,
    mut q: Query<(&UiButton, &mut ButtonState)>,
) {
    if let Ok((btn, mut state)) = q.get_mut(trigger.entity) {
        if !btn.disabled {
            *state = ButtonState::Pressed;
        }
    }
}

pub fn handle_button_pointer_release(
    trigger: On<PointerRelease>,
    mut q: Query<(&UiButton, &mut ButtonState)>,
) {
    if let Ok((btn, mut state)) = q.get_mut(trigger.entity) {
        if !btn.disabled {
            *state = ButtonState::Hovered;
        }
    }
}

/// Solid butonlar için saf Rust renk hesaplama fonksiyonu
fn button_colors(
    variant: ButtonVariant,
    state: ButtonState,
    disabled: bool,
    focused: bool,
) -> (Color, Color) {
    if disabled {
        return (
            Color::srgba(0.08, 0.11, 0.16, 0.40),
            Color::srgba(0.20, 0.25, 0.32, 0.30),
        );
    }
    if focused {
        return (
            Color::srgb(0.96, 0.78, 0.30),
            Color::srgb(1.0, 0.95, 0.75),
        );
    }

    match (variant, state) {
        (ButtonVariant::Primary, _) => (
            Color::srgb(0.83, 0.63, 0.27),
            Color::srgb(0.95, 0.82, 0.58),
        ),
        // --- ROYAL: Gece Mavisi / Çelik + Altın Kenarlık ---
        (ButtonVariant::Royal, ButtonState::Normal) => (
            Color::srgb(0.08, 0.11, 0.18),
            Color::srgb(0.78, 0.62, 0.28),
        ),
        (ButtonVariant::Royal, ButtonState::Hovered) => (
            Color::srgb(0.13, 0.18, 0.28),
            Color::srgb(1.0, 0.88, 0.48),
        ),
        (ButtonVariant::Royal, ButtonState::Pressed) => (
            Color::srgb(0.05, 0.07, 0.12),
            Color::srgb(0.68, 0.52, 0.22),
        ),

        // --- SECONDARY: Antrasit / Çelik ---
        (ButtonVariant::Secondary, ButtonState::Normal) => (
            Color::srgb(0.10, 0.13, 0.18),
            Color::srgb(0.24, 0.30, 0.40),
        ),
        (ButtonVariant::Secondary, ButtonState::Hovered) => (
            Color::srgb(0.16, 0.21, 0.29),
            Color::srgb(0.52, 0.64, 0.80),
        ),
        (ButtonVariant::Secondary, ButtonState::Pressed) => (
            Color::srgb(0.07, 0.09, 0.13),
            Color::srgb(0.18, 0.22, 0.30),
        ),

        // --- DANGER: Derin Yakut Kırmızısı ---
        (ButtonVariant::Danger, ButtonState::Normal) => (
            Color::srgb(0.40, 0.08, 0.10),
            Color::srgb(0.72, 0.18, 0.22),
        ),
        (ButtonVariant::Danger, ButtonState::Hovered) => (
            Color::srgb(0.56, 0.12, 0.16),
            Color::srgb(0.94, 0.28, 0.32),
        ),
        (ButtonVariant::Danger, ButtonState::Pressed) => (
            Color::srgb(0.28, 0.05, 0.07),
            Color::srgb(0.50, 0.12, 0.15),
        ),

        // --- GHOST: Saydam / Hafif Çerçeveli ---
        (ButtonVariant::Ghost, ButtonState::Normal) => (
            Color::NONE,
            Color::NONE,
        ),
        (ButtonVariant::Ghost, ButtonState::Hovered) => (
            Color::srgba(1.0, 1.0, 1.0, 0.07),
            Color::srgba(1.0, 1.0, 1.0, 0.18),
        ),
        (ButtonVariant::Ghost, ButtonState::Pressed) => (
            Color::srgba(1.0, 1.0, 1.0, 0.04),
            Color::srgba(1.0, 1.0, 1.0, 0.10),
        ),
    }
}

/// Action Card için arka plan gradyanı ve kenarlık gradyanı
pub fn action_card_gradients(variant: ButtonVariant, state: ButtonState) -> (BackgroundGradient, BorderGradient) {
    match variant {
        ButtonVariant::Primary => match state {
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("22180D").unwrap().into(),
                    Srgba::hex("0E0A06").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("D4AF37").unwrap().into(),
                    Srgba::hex("7B5919").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("322312").unwrap().into(),
                    Srgba::hex("160F08").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("FFDF78").unwrap().into(),
                    Srgba::hex("B38328").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("1A1208").unwrap().into(),
                    Srgba::hex("0A0703").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("AA8622").unwrap().into(),
                    Srgba::hex("563C0F").unwrap().into(),
                ])),
            ),
        },
        ButtonVariant::Royal => match state {
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("121A29").unwrap().into(),
                    Srgba::hex("090D15").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("C9A234").unwrap().into(),
                    Srgba::hex("6B4E15").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("1B263C").unwrap().into(),
                    Srgba::hex("0D1421").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("FBDD6C").unwrap().into(),
                    Srgba::hex("9C711D").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("0D131E").unwrap().into(),
                    Srgba::hex("06090E").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("9C771C").unwrap().into(),
                    Srgba::hex("4E360B").unwrap().into(),
                ])),
            ),
        },
        _ => match state {
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("151C28").unwrap().into(),
                    Srgba::hex("0C1017").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("3F4F66").unwrap().into(),
                    Srgba::hex("202937").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("1F293A").unwrap().into(),
                    Srgba::hex("111620").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("5D7495").unwrap().into(),
                    Srgba::hex("2D3A4D").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("0F141D").unwrap().into(),
                    Srgba::hex("080A0F").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("323E50").unwrap().into(),
                    Srgba::hex("171E28").unwrap().into(),
                ])),
            ),
        },
    }
}

/// Action Card içindeki ikon rozet kutusu (Badge) için Bevy gradyanları
pub fn action_card_badge_gradients(variant: ButtonVariant, state: ButtonState) -> (BackgroundGradient, BorderGradient) {
    let is_gold = variant == ButtonVariant::Primary || variant == ButtonVariant::Royal;
    if is_gold {
        match state {
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("2E200F").unwrap().into(),
                    Srgba::hex("150D05").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("E8C466").unwrap().into(),
                    Srgba::hex("8E631B").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("FBD67A").unwrap().into(),
                    Srgba::hex("B37F28").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("FFF4D0").unwrap().into(),
                    Srgba::hex("D4A43A").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("C99B35").unwrap().into(),
                    Srgba::hex("7B5115").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("E4BE64").unwrap().into(),
                    Srgba::hex("64410F").unwrap().into(),
                ])),
            ),
        }
    } else {
        match state {
            ButtonState::Normal => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("1B2332").unwrap().into(),
                    Srgba::hex("0E131C").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("4C5E78").unwrap().into(),
                    Srgba::hex("232C3A").unwrap().into(),
                ])),
            ),
            ButtonState::Hovered => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("354660").unwrap().into(),
                    Srgba::hex("1C2533").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("738CAE").unwrap().into(),
                    Srgba::hex("38475C").unwrap().into(),
                ])),
            ),
            ButtonState::Pressed => (
                BackgroundGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("161D2A").unwrap().into(),
                    Srgba::hex("0B0F16").unwrap().into(),
                ])),
                BorderGradient::from(LinearGradient::to_bottom(vec![
                    Srgba::hex("3C4B60").unwrap().into(),
                    Srgba::hex("18202A").unwrap().into(),
                ])),
            ),
        }
    }
}

/// Action Card'ın sol rozeti, sembolü ve metin renkleri
fn action_card_inner_colors(
    variant: ButtonVariant,
    state: ButtonState,
    disabled: bool,
) -> (Color, Color, Color, Color) {
    if disabled {
        return (
            Color::srgb(0.12, 0.14, 0.18),
            Color::srgb(0.40, 0.45, 0.52),
            Color::srgb(0.40, 0.45, 0.52),
            Color::srgb(0.30, 0.35, 0.40),
        );
    }

    let is_gold = variant == ButtonVariant::Primary || variant == ButtonVariant::Royal;
    match state {
        ButtonState::Normal => {
            if is_gold {
                (
                    Color::srgba(0.85, 0.65, 0.20, 0.18),
                    Color::srgb(0.96, 0.84, 0.48), // Altın sembol
                    Color::srgb(0.96, 0.86, 0.52), // Başlık
                    Color::srgb(0.70, 0.74, 0.82), // Alt metin
                )
            } else {
                (
                    Color::srgba(0.20, 0.26, 0.36, 0.40),
                    Color::srgb(0.82, 0.88, 0.95),
                    Color::srgb(0.88, 0.92, 0.96),
                    Color::srgb(0.60, 0.66, 0.74),
                )
            }
        }
        ButtonState::Hovered => {
            if is_gold {
                (
                    Color::srgb(0.85, 0.65, 0.20),
                    Color::from(Srgba::hex("2A1704").unwrap()), // Altın zemin üstüne net koyu sembol (Primary butondaki koyu metin gibi)
                    Color::srgb(1.0, 0.95, 0.70),
                    Color::srgb(0.90, 0.94, 0.98),
                )
            } else {
                (
                    Color::srgb(0.32, 0.42, 0.55),
                    Color::WHITE,
                    Color::WHITE,
                    Color::srgb(0.85, 0.90, 0.96),
                )
            }
        }
        ButtonState::Pressed => {
            if is_gold {
                (
                    Color::srgb(0.68, 0.50, 0.14),
                    Color::from(Srgba::hex("1F1102").unwrap()), // Basıldığında koyu sembol
                    Color::srgb(0.90, 0.78, 0.42),
                    Color::srgb(0.65, 0.70, 0.76),
                )
            } else {
                (
                    Color::srgb(0.18, 0.24, 0.32),
                    Color::srgb(0.70, 0.75, 0.82),
                    Color::srgb(0.75, 0.80, 0.88),
                    Color::srgb(0.50, 0.55, 0.62),
                )
            }
        }
    }
}

/// Tüm butonların ButtonState değişimlerini saf Rust ile kristal netliğinde günceller.
pub fn update_button_visuals(
    mut btn_q: Query<(
        Entity,
        &UiButton,
        &ButtonState,
        Option<&mut BackgroundGradient>,
        Option<&mut BorderGradient>,
        Option<&mut BackgroundColor>,
        Option<&mut BorderColor>,
    ), (Changed<ButtonState>, Without<UiButtonBadge>)>,
    children_q: Query<&Children>,
    mut shadow_q: Query<&mut BorderColor, (With<UiButtonInnerShadow>, Without<UiButton>)>,
    mut badge_q: Query<(&mut BackgroundGradient, &mut BorderGradient), (With<UiButtonBadge>, Without<UiButton>)>,
    mut badge_sym_q: Query<&mut TextColor, (With<UiButtonBadgeSymbol>, Without<UiButtonText>, Without<UiButtonSubtitle>)>,
    mut badge_img_q: Query<(&UiButtonBadgeImage, &mut ImageNode)>,
    mut text_q: Query<&mut TextColor, (With<UiButtonText>, Without<UiButtonSubtitle>, Without<UiButtonBadgeSymbol>)>,
    mut sub_text_q: Query<&mut TextColor, (With<UiButtonSubtitle>, Without<UiButtonText>, Without<UiButtonBadgeSymbol>)>,
) {
    for (btn_entity, btn, state, mut opt_bg_grad, mut opt_border_grad, mut opt_bg, mut opt_border) in &mut btn_q {
        if btn.style == ButtonStyle::Solid {
            // --- TÜM SOLID BUTONLAR İÇİN GRADYAN & INNER SHADOW GÜNCELLEMESİ ---
            let (new_bg_grad, new_border_grad) = button_gradients(btn.variant, *state);
            let new_shadow_color = inner_shadow_colors_for_variant(btn.variant, *state);

            if let Some(ref mut bg_g) = opt_bg_grad {
                **bg_g = new_bg_grad;
            }
            if let Some(ref mut b_g) = opt_border_grad {
                **b_g = new_border_grad;
            }

            let target_text_color = if btn.disabled {
                Color::srgb(0.40, 0.45, 0.52)
            } else {
                match btn.variant {
                    ButtonVariant::Primary => Color::from(Srgba::hex("2A1704").unwrap()),
                    ButtonVariant::Royal => match state {
                        ButtonState::Hovered => Color::srgb(1.0, 0.96, 0.82),
                        _ => Color::from(Srgba::hex("F6E27A").unwrap()),
                    },
                    ButtonVariant::Secondary => match state {
                        ButtonState::Hovered => Color::WHITE,
                        _ => Color::from(Srgba::hex("E2E8F0").unwrap()),
                    },
                    ButtonVariant::Danger => match state {
                        ButtonState::Hovered => Color::WHITE,
                        _ => Color::from(Srgba::hex("FFFFFF").unwrap()),
                    },
                    ButtonVariant::Ghost => match state {
                        ButtonState::Hovered => Color::WHITE,
                        _ => Color::from(Srgba::hex("CBD5E1").unwrap()),
                    },
                }
            };

            if let Ok(children) = children_q.get(btn_entity) {
                for child in children.iter() {
                    if let Ok(mut sh_border) = shadow_q.get_mut(child) {
                        *sh_border = new_shadow_color;
                    }
                    if let Ok(mut t_col) = text_q.get_mut(child) {
                        t_col.0 = target_text_color;
                    }
                }
            }
        } else if btn.style == ButtonStyle::Card {
            // Action Card Görsel Güncellemesi (Gradyan + Inner Shadow + İkon ve Metinler)
            let (new_bg_grad, new_border_grad) = action_card_gradients(btn.variant, *state);
            let (new_badge_bg_grad, new_badge_border_grad) = action_card_badge_gradients(btn.variant, *state);
            let new_shadow_color = inner_shadow_colors_for_variant(btn.variant, *state);

            if let Some(ref mut bg_g) = opt_bg_grad {
                **bg_g = new_bg_grad;
            }
            if let Some(ref mut b_g) = opt_border_grad {
                **b_g = new_border_grad;
            }

            let (_badge_bg, badge_sym_col, title_col, sub_col) =
                action_card_inner_colors(btn.variant, *state, btn.disabled);

            if let Ok(children) = children_q.get(btn_entity) {
                for child in children.iter() {
                    if let Ok(mut sh_border) = shadow_q.get_mut(child) {
                        *sh_border = new_shadow_color;
                    }
                    if let Ok((mut b_bg_g, mut b_border_g)) = badge_q.get_mut(child) {
                        *b_bg_g = new_badge_bg_grad.clone();
                        *b_border_g = new_badge_border_grad.clone();
                    }
                    if let Ok((img_meta, mut img_node)) = badge_img_q.get_mut(child) {
                        img_node.image = match state {
                            ButtonState::Normal => img_meta.normal.clone(),
                            ButtonState::Hovered => img_meta.hover.clone(),
                            ButtonState::Pressed => img_meta.pressed.clone(),
                        };
                    }
                    if let Ok(mut b_sym) = badge_sym_q.get_mut(child) {
                        b_sym.0 = badge_sym_col;
                    }
                    if let Ok(mut t_col) = text_q.get_mut(child) {
                        t_col.0 = title_col;
                    }
                    if let Ok(mut s_col) = sub_text_q.get_mut(child) {
                        s_col.0 = sub_col;
                    }
                    if let Ok(grand_children) = children_q.get(child) {
                        for grand_child in grand_children.iter() {
                            if let Ok(mut sh_border) = shadow_q.get_mut(grand_child) {
                                *sh_border = new_shadow_color;
                            }
                            if let Ok((img_meta, mut img_node)) = badge_img_q.get_mut(grand_child) {
                                img_node.image = match state {
                                    ButtonState::Normal => img_meta.normal.clone(),
                                    ButtonState::Hovered => img_meta.hover.clone(),
                                    ButtonState::Pressed => img_meta.pressed.clone(),
                                };
                            }
                            if let Ok(mut b_sym) = badge_sym_q.get_mut(grand_child) {
                                b_sym.0 = badge_sym_col;
                            }
                            if let Ok(mut t_col) = text_q.get_mut(grand_child) {
                                t_col.0 = title_col;
                            }
                            if let Ok(mut s_col) = sub_text_q.get_mut(grand_child) {
                                s_col.0 = sub_col;
                            }
                        }
                    }
                }
            }
        } else {
            // Standart Solid Buton Görsel Güncellemesi (Royal, Secondary, Danger, Ghost)
            let (new_bg, new_border) = button_colors(btn.variant, *state, btn.disabled, btn.focused);
            if let Some(ref mut bg) = opt_bg {
                bg.0 = new_bg;
            }
            if let Some(ref mut border) = opt_border {
                **border = BorderColor::all(new_border);
            }

            // Metin rengi hesapla
            let target_text_color = if btn.disabled {
                Color::srgb(0.40, 0.45, 0.52)
            } else {
                match (btn.variant, state) {
                    (ButtonVariant::Royal, ButtonState::Hovered) => Color::srgb(1.0, 0.96, 0.82),
                    (ButtonVariant::Royal, _) => Color::srgb(0.92, 0.78, 0.42),
                    (ButtonVariant::Secondary, ButtonState::Hovered) => Color::WHITE,
                    (ButtonVariant::Secondary, _) => Color::srgb(0.86, 0.89, 0.94),
                    (ButtonVariant::Danger, _) => Color::WHITE,
                    (ButtonVariant::Ghost, ButtonState::Hovered) => Color::WHITE,
                    (ButtonVariant::Ghost, _) => Color::srgb(0.70, 0.75, 0.82),
                    _ => Color::WHITE,
                }
            };

            if let Ok(children) = children_q.get(btn_entity) {
                for child in children.iter() {
                    if let Ok(mut t_col) = text_q.get_mut(child) {
                        t_col.0 = target_text_color;
                    }
                }
            }
        }
    }
}
