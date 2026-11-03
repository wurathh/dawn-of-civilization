#!/usr/bin/env python3
"""
Dawn of Civilization — UI Doku Üreteci CLI Giriş Noktası

Kullanım:
    python3 scripts/generate_ui_textures.py

Bu komut:
1. `assets/ui_textures/buttons/` altına 12 adet 9-slice buton dokusunu üretir.
2. `assets/ui_textures/panels/` altına %100 OPAK 9-slice panel/kart dokularını üretir.
3. `assets/ui_textures/PREVIEW_TEXTURES.png` önizleme sayfasını günceller.
"""

import os
import sys

# Proje kök dizinini Python path'e ekle
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from scripts.ui_generator.config import BUTTONS_DIR, PANELS_DIR, PREVIEW_PATH
from scripts.ui_generator.buttons import create_button_texture
from scripts.ui_generator.panels import create_panel_texture
from scripts.ui_generator.preview import generate_preview_sheet

def main():
    os.makedirs(BUTTONS_DIR, exist_ok=True)
    os.makedirs(PANELS_DIR, exist_ok=True)

    print("=======================================================")
    print("  Dawn of Civilization — Profesyonel UI Doku Üreteci")
    print("=======================================================")

    variants = ["primary", "royal", "secondary", "danger"]
    states = ["normal", "hover", "pressed"]

    # 1. Buton Dokuları
    btn_textures = {}
    print("\n[1/3] 9-Slice Buton Dokuları Üretiliyor...")
    for var in variants:
        for st in states:
            filename = f"button_{var}_{st}.png"
            path = os.path.join(BUTTONS_DIR, filename)
            tex = create_button_texture(var, st)
            tex.save(path)
            btn_textures[(var, st)] = tex
            print(f"  ✓ {filename} (64x64, 16px corner slice)")

    # 2. Panel Dokuları (%100 Opak)
    pnl_textures = {}
    print("\n[2/3] 9-Slice %100 Opak Panel Dokuları Üretiliyor...")
    for p_var in ["ornate_gold", "slate_card"]:
        filename = f"panel_{p_var}.png"
        path = os.path.join(PANELS_DIR, filename)
        tex = create_panel_texture(p_var)
        tex.save(path)
        pnl_textures[p_var] = tex
        print(f"  ✓ {filename} (96x96, 24px corner slice, %100 Opak)")

    # 3. Önizleme Sayfası
    print("\n[3/3] Yüksek Çözünürlüklü Önizleme Sayfası Birleştiriliyor...")
    generate_preview_sheet(btn_textures, pnl_textures)

    print("\n[✓] İŞLEM TAMAMLANDI!")
    print(f"    Önizleme Sayfası: {PREVIEW_PATH}")
    print(f"    Buton Dokuları:   {BUTTONS_DIR}")
    print(f"    Panel Dokuları:   {PANELS_DIR}\n")

if __name__ == "__main__":
    main()
