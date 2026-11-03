use crate::state::UiFonts;
use crate::ui::theme::Theme;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextVariant {
    Display,
    Headline,
    Title,
    Label,
    Body,
    Caption,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextTone {
    Primary,
    Secondary,
    Accent,
    Disabled,
    Danger,
    Success,
    Warning,
    Inverse,
    InverseSecondary,
}

/// Standart tipografi bundle'ı üretir.
pub fn text_bundle(
    theme: &Theme,
    fonts: &UiFonts,
    content: impl Into<String>,
    variant: TextVariant,
    tone: TextTone,
) -> (Text, TextFont, TextColor) {
    let (font, font_size) = match variant {
        TextVariant::Display => (fonts.display.clone(), FontSize::Px(theme.typography.display)),
        TextVariant::Headline => (fonts.display.clone(), FontSize::Px(theme.typography.headline)),
        TextVariant::Title => (fonts.heading.clone(), FontSize::Px(theme.typography.title)),
        TextVariant::Label => (fonts.heading.clone(), FontSize::Px(theme.typography.label)),
        TextVariant::Body => (fonts.body.clone(), FontSize::Px(theme.typography.body)),
        TextVariant::Caption => (fonts.medium.clone(), FontSize::Px(theme.typography.caption)),
    };

    let color = match tone {
        TextTone::Primary => theme.colors.text_primary,
        TextTone::Secondary => theme.colors.text_secondary,
        TextTone::Accent => theme.colors.accent,
        TextTone::Disabled => theme.colors.text_disabled,
        TextTone::Danger => theme.colors.danger,
        TextTone::Success => theme.colors.success,
        TextTone::Warning => theme.colors.warning,
        TextTone::Inverse => theme.colors.text_inverse(),
        TextTone::InverseSecondary => theme.colors.text_inverse_secondary(),
    };

    (
        Text::new(content.into()),
        TextFont {
            font,
            font_size,
            ..default()
        },
        TextColor(color),
    )
}

pub fn display(theme: &Theme, fonts: &UiFonts, text: impl Into<String>) -> (Text, TextFont, TextColor) {
    text_bundle(theme, fonts, text, TextVariant::Display, TextTone::Primary)
}

pub fn headline(theme: &Theme, fonts: &UiFonts, text: impl Into<String>) -> (Text, TextFont, TextColor) {
    text_bundle(theme, fonts, text, TextVariant::Headline, TextTone::Primary)
}

pub fn title(theme: &Theme, fonts: &UiFonts, text: impl Into<String>) -> (Text, TextFont, TextColor) {
    text_bundle(theme, fonts, text, TextVariant::Title, TextTone::Primary)
}

pub fn label(theme: &Theme, fonts: &UiFonts, text: impl Into<String>) -> (Text, TextFont, TextColor) {
    text_bundle(theme, fonts, text, TextVariant::Label, TextTone::Primary)
}

pub fn body(theme: &Theme, fonts: &UiFonts, text: impl Into<String>) -> (Text, TextFont, TextColor) {
    text_bundle(theme, fonts, text, TextVariant::Body, TextTone::Secondary)
}

pub fn caption(theme: &Theme, fonts: &UiFonts, text: impl Into<String>) -> (Text, TextFont, TextColor) {
    text_bundle(theme, fonts, text, TextVariant::Caption, TextTone::Secondary)
}
