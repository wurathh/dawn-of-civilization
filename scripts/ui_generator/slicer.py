"""
9-Slice Gerdirme ve Render Motoru
"""

from PIL import Image

def render_9slice(texture, width, height, corner=16):
    """
    Verilen 9-slice dokuyu, 4 köşeyi sabit tutup kenar ve orta bölgeleri uzatarak
    istenen hedef (width, height) boyutuna gerer.
    """
    dest = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    tw, th = texture.size

    # 4 Köşe (Hiç bozulmadan kopyalanır)
    c_tl = texture.crop((0, 0, corner, corner))
    c_tr = texture.crop((tw - corner, 0, tw, corner))
    c_bl = texture.crop((0, th - corner, corner, th))
    c_br = texture.crop((tw - corner, th - corner, tw, th))

    dest.paste(c_tl, (0, 0), c_tl)
    dest.paste(c_tr, (width - corner, 0), c_tr)
    dest.paste(c_bl, (0, height - corner), c_bl)
    dest.paste(c_br, (width - corner, height - corner), c_br)

    # 4 Kenar (Bilinear interpolasyonla uzatılır)
    edge_t = texture.crop((corner, 0, tw - corner, corner)).resize((width - 2 * corner, corner), Image.Resampling.BILINEAR)
    edge_b = texture.crop((corner, th - corner, tw - corner, th)).resize((width - 2 * corner, corner), Image.Resampling.BILINEAR)
    edge_l = texture.crop((0, corner, corner, th - corner)).resize((corner, height - 2 * corner), Image.Resampling.BILINEAR)
    edge_r = texture.crop((tw - corner, corner, tw, th - corner)).resize((corner, height - 2 * corner), Image.Resampling.BILINEAR)

    dest.paste(edge_t, (corner, 0), edge_t)
    dest.paste(edge_b, (corner, height - corner), edge_b)
    dest.paste(edge_l, (0, corner), edge_l)
    dest.paste(edge_r, (width - corner, corner), edge_r)

    # Orta Zemin (Genişletilir)
    center = texture.crop((corner, corner, tw - corner, th - corner)).resize((width - 2 * corner, height - 2 * corner), Image.Resampling.BILINEAR)
    dest.paste(center, (corner, corner), center)

    return dest
