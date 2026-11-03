"""
Önizleme ve Test Sayfası Oluşturucu
"""

from PIL import Image, ImageDraw, ImageFont
from .config import (
    BTN_CORNER, PANEL_CORNER, PREVIEW_PATH,
    FONT_TITLE, FONT_HEADING, FONT_BODY, FONT_CAPTION
)
from .slicer import render_9slice

def get_font(path, size):
    try:
        return ImageFont.truetype(path, size)
    except Exception:
        return ImageFont.load_default()

def generate_preview_sheet(btn_textures, pnl_textures):
    preview_w = 1440
    preview_h = 1050
    preview = Image.new("RGBA", (preview_w, preview_h), (9, 13, 20, 255))
    draw = ImageDraw.Draw(preview)

    font_title = get_font(FONT_TITLE, 26)
    font_sub = get_font(FONT_CAPTION, 14)
    font_btn = get_font(FONT_HEADING, 15)
    font_label = get_font(FONT_BODY, 13)

    # Arka plan taktik ızgara çizgileri
    for y in range(0, preview_h, 36):
        draw.line([(0, y), (preview_w, y)], fill=(18, 24, 36, 150), width=1)
    for x in range(0, preview_w, 36):
        draw.line([(x, 0), (x, preview_h)], fill=(18, 24, 36, 150), width=1)

    # Başlık Alanı
    draw.text((60, 36), "DAWN OF CIVILIZATION — 9-SLICE UI DOKU SİSTEMİ", fill=(217, 165, 74, 255), font=font_title)
    draw.text((60, 72), "Piksel-Kusursuz 9-Slice: Butonlar ve %100 OPAK paneller her çözünürlükte bozulmadan uzayıp kısalır.", fill=(160, 175, 195, 255), font=font_sub)
    draw.line([(60, 100), (preview_w - 60, 100)], fill=(217, 165, 74, 180), width=2)

    # Sütun Başlıkları
    col_x1 = 250
    col_x2 = 560
    col_x3 = 870
    col_x4 = 1170

    draw.text((col_x1 + 45, 115), "NORMAL DURUM", fill=(217, 165, 74, 255), font=font_label)
    draw.text((col_x2 + 25, 115), "HOVER DURUMU (PARILTI)", fill=(245, 215, 130, 255), font=font_label)
    draw.text((col_x3 + 20, 115), "PRESSED DURUMU (ÇÖKÜK)", fill=(180, 135, 60, 255), font=font_label)
    draw.text((col_x4 - 5, 115), "9-SLICE GERİLMİŞ (GENİŞ)", fill=(217, 165, 74, 255), font=font_label)

    row_y = 150
    button_rows = [
        ("primary", "1. PRIMARY (İmparatorluk Altını)", "OYNA", (10, 8, 3, 255)),
        ("royal", "2. ROYAL (Perçinli Süslü Altın)", "YENİ OYUN KUR", (245, 235, 210, 255)),
        ("secondary", "3. SECONDARY (Koyu Arduvaz)", "SENARYO SEÇİMİ", (230, 235, 245, 255)),
        ("danger", "4. DANGER (Savaş / Çıkış)", "OYUNDAN ÇIK", (255, 255, 255, 255)),
    ]

    for var, title, btn_text, text_col in button_rows:
        draw.text((60, row_y + 15), title, fill=(235, 238, 245, 255), font=font_label)

        # Normal (260x48)
        btn_norm = render_9slice(btn_textures[(var, "normal")], 260, 48, corner=BTN_CORNER)
        preview.paste(btn_norm, (col_x1, row_y), btn_norm)

        # Hover (260x48)
        btn_hov = render_9slice(btn_textures[(var, "hover")], 260, 48, corner=BTN_CORNER)
        preview.paste(btn_hov, (col_x2, row_y), btn_hov)

        # Pressed (260x48)
        btn_press = render_9slice(btn_textures[(var, "pressed")], 260, 48, corner=BTN_CORNER)
        preview.paste(btn_press, (col_x3, row_y), btn_press)

        # Geniş / Stretch Testi (180x48)
        btn_wide = render_9slice(btn_textures[(var, "hover" if var=="primary" else "normal")], 180, 48, corner=BTN_CORNER)
        preview.paste(btn_wide, (col_x4 - 30, row_y), btn_wide)

        # Yazıları Ortalayarak Yaz
        for bx in [col_x1, col_x2, col_x3]:
            t_col = (5, 4, 1, 255) if (var == "primary" and bx == col_x2) else text_col
            bbox = font_btn.getbbox(btn_text)
            tw = bbox[2] - bbox[0]
            tx = bx + (260 - tw) // 2
            draw.text((tx, row_y + 14), btn_text, fill=t_col, font=font_btn)

        bbox_w = font_btn.getbbox("KÜÇÜK BOYUT")
        draw.text((col_x4 - 30 + (180 - (bbox_w[2] - bbox_w[0])) // 2, row_y + 14), "KÜÇÜK BOYUT", fill=text_col, font=font_btn)

        draw.line([(60, row_y + 68), (preview_w - 60, row_y + 68)], fill=(28, 36, 50, 160), width=1)
        row_y += 82

    # -------------------------------------------------------------
    # 5. GERİLMİŞ %100 OPAK PANELLER (Alt Bölüm)
    # -------------------------------------------------------------
    draw.text((60, row_y + 10), "%100 OPAK 9-SLICE PANELLER (Arka Plan Asla Geçirmez, Alpha=255):", fill=(217, 165, 74, 255), font=font_label)

    pnl_y = row_y + 36
    # Panel 1: Ornate Gold Frame (620x180)
    pnl_gold = render_9slice(pnl_textures["ornate_gold"], 620, 180, corner=PANEL_CORNER)
    preview.paste(pnl_gold, (60, pnl_y), pnl_gold)
    draw.text((85, pnl_y + 20), "ORNATE GOLD PANEL (%100 Opak Obsidyen)", fill=(217, 165, 74, 255), font=font_label)
    draw.text((85, pnl_y + 44), "Antik çentikli altın köşeler, döküm bronz gren ve masif obsidyen zemin.", fill=(160, 175, 195, 255), font=font_sub)

    mini_btn = render_9slice(btn_textures[("primary", "normal")], 200, 42, corner=BTN_CORNER)
    preview.paste(mini_btn, (85, pnl_y + 110), mini_btn)
    draw.text((130, pnl_y + 120), "SEFERİ BAŞLAT", fill=(10, 8, 3, 255), font=font_btn)

    # Panel 2: Slate Card (660x180)
    pnl_slate = render_9slice(pnl_textures["slate_card"], 660, 180, corner=PANEL_CORNER)
    preview.paste(pnl_slate, (720, pnl_y), pnl_slate)
    draw.text((745, pnl_y + 20), "SLATE HUD & DİPLOMASİ KARTI (%100 Opak Arduvaz)", fill=(230, 235, 245, 255), font=font_label)
    draw.text((745, pnl_y + 44), "İnce çelik kenarlık, kristal netliğinde okunabilir masif taş zemin.", fill=(160, 175, 195, 255), font=font_sub)

    mini_btn2 = render_9slice(btn_textures[("secondary", "hover")], 200, 42, corner=BTN_CORNER)
    preview.paste(mini_btn2, (745, pnl_y + 110), mini_btn2)
    draw.text((780, pnl_y + 120), "DETAYLI BİLGİ", fill=(245, 215, 130, 255), font=font_btn)

    # -------------------------------------------------------------
    # 6. HAM MASTER DOKULAR
    # -------------------------------------------------------------
    master_y = pnl_y + 205
    draw.text((60, master_y), "HAM MASTER 9-SLICE DOKULAR (assets/ui_textures/ altındaki kaynak dosyalar):", fill=(217, 165, 74, 255), font=font_label)

    mx = 60
    my = master_y + 25
    for var, _, _, _ in button_rows:
        for st in ["normal", "hover", "pressed"]:
            raw_t = btn_textures[(var, st)]
            draw.rectangle([mx - 1, my - 1, mx + 64, my + 64], outline=(40, 55, 75, 255))
            preview.paste(raw_t, (mx, my), raw_t)
            mx += 76

    for p_var in ["ornate_gold", "slate_card"]:
        raw_p = pnl_textures[p_var].resize((64, 64), Image.Resampling.BILINEAR)
        draw.rectangle([mx - 1, my - 1, mx + 64, my + 64], outline=(60, 80, 110, 255))
        preview.paste(raw_p, (mx, my), raw_p)
        mx += 76

    preview.save(PREVIEW_PATH)
    print(f"\n[★] Master görsel önizleme oluşturuldu: {PREVIEW_PATH}")
