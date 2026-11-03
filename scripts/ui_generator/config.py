"""
Dawn of Civilization UI Doku Üreteci — Yapılandırma ve Token Sabitleri
"""

import os

# Çözünürlük ve 9-Slice Slicing Sabitleri
BTN_SIZE = 64
BTN_CORNER = 16

PANEL_SIZE = 96
PANEL_CORNER = 24

# Dizin Yolları
ASSETS_DIR = "assets"
UI_TEXTURES_DIR = os.path.join(ASSETS_DIR, "ui_textures")
BUTTONS_DIR = os.path.join(UI_TEXTURES_DIR, "buttons")
PANELS_DIR = os.path.join(UI_TEXTURES_DIR, "panels")
FONTS_DIR = os.path.join(ASSETS_DIR, "fonts")
PREVIEW_PATH = os.path.join(UI_TEXTURES_DIR, "PREVIEW_TEXTURES.png")

# Font Yolları
FONT_TITLE = os.path.join(FONTS_DIR, "Cinzel-Bold.ttf")
FONT_HEADING = os.path.join(FONTS_DIR, "Inter-Bold.ttf")
FONT_BODY = os.path.join(FONTS_DIR, "Inter-SemiBold.ttf")
FONT_CAPTION = os.path.join(FONTS_DIR, "Inter-Medium.ttf")
