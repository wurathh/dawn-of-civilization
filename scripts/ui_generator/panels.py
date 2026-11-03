"""
9-Slice OPAK (Saydam Olmayan) Panel & Kart Doku Üreticisi
"""

import math
from PIL import Image, ImageDraw
from .config import PANEL_SIZE
from .noise import blend_color, apply_metallic_noise, pseudo_noise, clamp_byte

def create_panel_texture(variant="ornate_gold"):
    """
    96x96 piksel, 24px kenar korumalı (9-slice) %100 OPAK panel/çerçeve dokusu.
    Arka plan tamamen opaktır (Alpha=255); altındaki harita, yazı veya UI elemanları kesinlikle görünmez.
    """
    img = Image.new("RGBA", (PANEL_SIZE, PANEL_SIZE), (0, 0, 0, 0))
    pixels = img.load()

    if variant == "ornate_gold":
        rim_hi = (255, 235, 170, 255)      # Üst Işıltılı Altın
        rim_lo = (135, 95, 32, 255)        # Alt Bronz Gölge
        # %100 OPAK Obsidyen-Arduvaz Zemin (Alpha = 255)
        body_bg = (11, 15, 24, 255)
    else: # slate_card
        rim_hi = (105, 135, 175, 255)      # Çelik Mavi İnce Çerçeve
        rim_lo = (35, 48, 68, 255)
        # %100 OPAK Koyu Arduvaz Zemin (Alpha = 255)
        body_bg = (13, 18, 28, 255)

    radius = 12.0
    pad = 2.0

    for y in range(PANEL_SIZE):
        ty = y / (PANEL_SIZE - 1.0)
        rim_c = blend_color(rim_hi, rim_lo, ty)
        # Gövde içi hafif dikey ışık süzülmesi (tamamen opak)
        bg_c = blend_color(
            (body_bg[0] + int(8 * (1.0 - ty)), body_bg[1] + int(8 * (1.0 - ty)), body_bg[2] + int(10 * (1.0 - ty)), 255),
            body_bg,
            ty
        )

        for x in range(PANEL_SIZE):
            cx = (pad + radius) if x < (pad + radius) else ((PANEL_SIZE - 1 - pad - radius) if x > (PANEL_SIZE - 1 - pad - radius) else x)
            cy = (pad + radius) if y < (pad + radius) else ((PANEL_SIZE - 1 - pad - radius) if y > (PANEL_SIZE - 1 - pad - radius) else y)

            dist = math.hypot(x - cx, y - cy)
            outer_dist = dist - radius
            if outer_dist > 1.2:
                continue

            # Sadece dış kenar antialiasing'i
            alpha_edge = min(1.0, max(0.0, 0.5 - outer_dist))
            is_outer_rim = outer_dist > -1.8
            is_inner_groove = outer_dist > -3.0 and not is_outer_rim

            if is_outer_rim:
                c = rim_c
            elif is_inner_groove:
                c = (5, 7, 12, 255) # İç oyuk (100% opak)
            else:
                c = bg_c
                # Üst kenar altın ortam parıltısı (yalnızca ilk 18px)
                if y < pad + 18 and variant == "ornate_gold":
                    glow_t = (1.0 - (y - pad) / 18.0) * 0.28
                    c = blend_color(c, (217, 165, 74, 255), glow_t)

            # Panel için ince arduvaz / taş dokusu gürültüsü
            if not is_inner_groove:
                p_noise = pseudo_noise(x, y, 333) * (2.5 if is_outer_rim else 4.0)
                c = (clamp_byte(c[0] + p_noise), clamp_byte(c[1] + p_noise), clamp_byte(c[2] + p_noise), 255)

            pixels[x, y] = (c[0], c[1], c[2], int(255 * alpha_edge))

    # Ornate ise köşelere antik çentik ve altın köşebent motifi ekle
    if variant == "ornate_gold":
        draw = ImageDraw.Draw(img)
        corners = [
            (pad + 12, pad + 12),
            (PANEL_SIZE - 1 - pad - 12, pad + 12),
            (pad + 12, PANEL_SIZE - 1 - pad - 12),
            (PANEL_SIZE - 1 - pad - 12, PANEL_SIZE - 1 - pad - 12),
        ]
        for cx, cy in corners:
            draw.line([(cx - 4, cy), (cx + 4, cy)], fill=(245, 215, 130, 255), width=1)
            draw.line([(cx, cy - 4), (cx, cy + 4)], fill=(245, 215, 130, 255), width=1)

    return img
