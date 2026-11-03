"""
9-Slice Buton Doku Üreticisi
"""

import math
from PIL import Image, ImageDraw
from .config import BTN_SIZE
from .noise import blend_color, apply_metallic_noise

def create_button_texture(variant="primary", state="normal"):
    img = Image.new("RGBA", (BTN_SIZE, BTN_SIZE), (0, 0, 0, 0))
    pixels = img.load()

    # Renk ve Gradyan Profilleri
    if variant == "primary":
        if state == "normal":
            rim_top = (248, 208, 130, 255)
            rim_mid = (217, 165, 74, 255)
            rim_bot = (145, 102, 35, 255)
            body_top = (195, 142, 52, 255)
            body_mid = (170, 118, 38, 255)
            body_bot = (130, 85, 24, 255)
            glow_col = (237, 184, 94, 60)
            inner_shadow = False
        elif state == "hover":
            rim_top = (255, 245, 195, 255)
            rim_mid = (252, 212, 125, 255)
            rim_bot = (190, 140, 55, 255)
            body_top = (240, 190, 100, 255)
            body_mid = (220, 165, 75, 255)
            body_bot = (180, 125, 45, 255)
            glow_col = (255, 220, 125, 150)
            inner_shadow = False
        else: # pressed
            rim_top = (120, 80, 25, 255)
            rim_mid = (160, 110, 40, 255)
            rim_bot = (220, 170, 80, 255)
            body_top = (115, 75, 22, 255)
            body_mid = (135, 90, 28, 255)
            body_bot = (155, 105, 34, 255)
            glow_col = (0, 0, 0, 0)
            inner_shadow = True

    elif variant == "secondary":
        if state == "normal":
            rim_top = (85, 105, 135, 255)
            rim_mid = (50, 68, 92, 255)
            rim_bot = (28, 38, 54, 255)
            body_top = (32, 45, 64, 255)
            body_mid = (24, 32, 48, 255)
            body_bot = (18, 24, 36, 255)
            glow_col = (50, 70, 100, 40)
            inner_shadow = False
        elif state == "hover":
            rim_top = (245, 200, 120, 255)
            rim_mid = (210, 158, 75, 255)
            rim_bot = (125, 90, 35, 255)
            body_top = (48, 64, 88, 255)
            body_mid = (36, 48, 68, 255)
            body_bot = (26, 36, 52, 255)
            glow_col = (217, 165, 74, 90)
            inner_shadow = False
        else: # pressed
            rim_top = (20, 28, 40, 255)
            rim_mid = (35, 48, 68, 255)
            rim_bot = (75, 95, 125, 255)
            body_top = (14, 18, 26, 255)
            body_mid = (18, 24, 34, 255)
            body_bot = (26, 34, 48, 255)
            glow_col = (0, 0, 0, 0)
            inner_shadow = True

    elif variant == "danger":
        if state == "normal":
            rim_top = (250, 130, 130, 255)
            rim_mid = (215, 55, 55, 255)
            rim_bot = (140, 30, 30, 255)
            body_top = (180, 42, 42, 255)
            body_mid = (150, 30, 30, 255)
            body_bot = (110, 20, 20, 255)
            glow_col = (230, 45, 45, 50)
            inner_shadow = False
        elif state == "hover":
            rim_top = (255, 180, 180, 255)
            rim_mid = (245, 85, 85, 255)
            rim_bot = (175, 40, 40, 255)
            body_top = (215, 60, 60, 255)
            body_mid = (180, 42, 42, 255)
            body_bot = (140, 26, 26, 255)
            glow_col = (255, 70, 70, 120)
            inner_shadow = False
        else: # pressed
            rim_top = (90, 15, 15, 255)
            rim_mid = (125, 22, 22, 255)
            rim_bot = (205, 50, 50, 255)
            body_top = (95, 16, 16, 255)
            body_mid = (115, 20, 20, 255)
            body_bot = (140, 28, 28, 255)
            glow_col = (0, 0, 0, 0)
            inner_shadow = True

    else: # royal (ornate multi-layered gold with corner brackets)
        if state == "normal":
            rim_top = (255, 228, 155, 255)
            rim_mid = (228, 178, 85, 255)
            rim_bot = (155, 110, 40, 255)
            body_top = (52, 42, 30, 255)
            body_mid = (36, 29, 21, 255)
            body_bot = (26, 20, 15, 255)
            glow_col = (237, 184, 94, 80)
            inner_shadow = False
        elif state == "hover":
            rim_top = (255, 250, 205, 255)
            rim_mid = (255, 218, 120, 255)
            rim_bot = (190, 140, 55, 255)
            body_top = (78, 60, 40, 255)
            body_mid = (54, 42, 28, 255)
            body_bot = (40, 30, 20, 255)
            glow_col = (255, 225, 115, 150)
            inner_shadow = False
        else: # pressed
            rim_top = (120, 85, 30, 255)
            rim_mid = (165, 118, 45, 255)
            rim_bot = (235, 185, 95, 255)
            body_top = (22, 16, 11, 255)
            body_mid = (30, 22, 15, 255)
            body_bot = (42, 32, 22, 255)
            glow_col = (0, 0, 0, 0)
            inner_shadow = True

    radius = 7.0
    pad = 3.0

    for y in range(BTN_SIZE):
        ty = y / (BTN_SIZE - 1.0)
        if ty < 0.45:
            body_c = blend_color(body_top, body_mid, ty / 0.45)
        else:
            body_c = blend_color(body_mid, body_bot, (ty - 0.45) / 0.55)

        if ty < 0.5:
            rim_c = blend_color(rim_top, rim_mid, ty / 0.5)
        else:
            rim_c = blend_color(rim_mid, rim_bot, (ty - 0.5) / 0.5)

        for x in range(BTN_SIZE):
            cx = (pad + radius) if x < (pad + radius) else ((BTN_SIZE - 1 - pad - radius) if x > (BTN_SIZE - 1 - pad - radius) else x)
            cy = (pad + radius) if y < (pad + radius) else ((BTN_SIZE - 1 - pad - radius) if y > (BTN_SIZE - 1 - pad - radius) else y)

            dist = math.hypot(x - cx, y - cy)
            outer_dist = dist - radius
            if outer_dist > 1.2:
                if outer_dist < 3.2 and glow_col[3] > 0:
                    glow_t = 1.0 - ((outer_dist - 1.2) / 2.0)
                    glow_alpha = int(glow_col[3] * glow_t * glow_t)
                    pixels[x, y] = (glow_col[0], glow_col[1], glow_col[2], glow_alpha)
                continue

            alpha_edge = min(1.0, max(0.0, 0.5 - outer_dist))
            is_rim = outer_dist > -2.2
            is_groove = outer_dist > -3.2 and not is_rim

            if is_rim:
                if y <= pad + 2:
                    c = blend_color(rim_c, (255, 255, 255, 255), 0.25)
                elif y >= BTN_SIZE - 1 - pad - 2 and not inner_shadow:
                    c = blend_color(rim_c, (0, 0, 0, 255), 0.25)
                else:
                    c = rim_c
            elif is_groove:
                c = (0, 0, 0, 200)
            else:
                c = body_c
                if y == int(pad + 3.2) and not inner_shadow:
                    c = blend_color(c, (255, 255, 255, 255), 0.35)
                elif y == int(pad + 4.2) and not inner_shadow:
                    c = blend_color(c, (255, 255, 255, 255), 0.15)
                elif y == BTN_SIZE - 1 - int(pad + 3.2) and inner_shadow:
                    c = blend_color(c, (255, 255, 255, 255), 0.25)

                if inner_shadow and y < pad + 8:
                    shadow_t = 1.0 - (y - pad) / 8.0
                    c = blend_color(c, (0, 0, 0, 255), shadow_t * 0.45)

            # Doku & Metalik Noise / Gren Efekti:
            if not is_groove:
                c = apply_metallic_noise(c, x, y, is_rim=is_rim, seed=77)

            final_a = int(c[3] * alpha_edge)
            pixels[x, y] = (c[0], c[1], c[2], final_a)

    draw = ImageDraw.Draw(img)
    if variant == "royal":
        stud_coords = [
            (pad + 6, pad + 6),
            (BTN_SIZE - 1 - pad - 6, pad + 6),
            (pad + 6, BTN_SIZE - 1 - pad - 6),
            (BTN_SIZE - 1 - pad - 6, BTN_SIZE - 1 - pad - 6)
        ]
        stud_c = (255, 235, 170, 255) if state != "pressed" else (160, 120, 50, 255)
        for sx, sy in stud_coords:
            draw.polygon([
                (sx, sy - 2),
                (sx + 2, sy),
                (sx, sy + 2),
                (sx - 2, sy)
            ], fill=stud_c, outline=(60, 45, 15, 200))

    return img
