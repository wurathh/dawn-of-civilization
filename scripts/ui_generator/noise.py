"""
Deterministik Mikro Gren (Noise) ve Fırçalanmış Metal Sheen Motoru
"""

import math

def pseudo_noise(x, y, seed=1337):
    """
    (x, y) koordinatı ve seed tabanlı [-1.0, 1.0] aralığında deterministik psödo gürültü.
    Tohum sabit kaldıkça pikseller %100 tekrarlanabilir ve 9-slice dikişsizdir.
    """
    val = math.sin(x * 127.1 + y * 311.7 + seed * 43.19) * 43758.5453123
    return (val - math.floor(val)) * 2.0 - 1.0

def clamp_byte(val):
    return max(0, min(255, int(val)))

def blend_color(c1, c2, t):
    """İki RGBA rengi lineer enterpolasyon (lerp) ile karıştırır."""
    return tuple(int(a * (1.0 - t) + b * t) for a, b in zip(c1, c2))

def apply_metallic_noise(c, x, y, is_rim=False, seed=77, noise_amp=7.5, brush_amp=2.5):
    """Metalik yüzeye mikro gren ve yatay fırçalama çizgileri ekler."""
    actual_amp = (noise_amp * 0.5) if is_rim else noise_amp
    micro_grain = pseudo_noise(x, y, seed) * actual_amp
    h_brush = math.sin(y * 3.14 + pseudo_noise(x, 0, seed + 22) * 0.5) * ((brush_amp * 0.6) if is_rim else brush_amp)
    n = micro_grain + h_brush
    return (clamp_byte(c[0] + n), clamp_byte(c[1] + n), clamp_byte(c[2] + n), c[3])
