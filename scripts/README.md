# DAWN OF CIVILIZATION — UI DOKU ÜRETİM MOTORU (`scripts/`)

Bu klasör, oyundaki 9-Slice (dokusu bozulmadan her boyutta uzayıp kısaltılabilen) buton, panel, kart ve süsleme dokularını matematiksel olarak prosedürel üreten profesyonel Python araçlarını içerir.

---

## 🚀 Hızlı Başlangıç

Tüm dokuları tek bir komutla yeniden oluşturmak ve önizleme sayfasını güncellemek için:

```bash
python3 scripts/generate_ui_textures.py
```

Bu komut:
1. `assets/ui_textures/buttons/` klasörüne **12 adet 64x64 piksel (16px köşe korumalı)** buton master dokusu üretir.
2. `assets/ui_textures/panels/` klasörüne **%100 OPAK 96x96 piksel (24px köşe korumalı)** panel ve kart master dokusu üretir.
3. `assets/ui_textures/PREVIEW_TEXTURES.png` önizleme sayfasını otomatik oluşturur.

---

## 📁 Modül Mimarisi (`scripts/ui_generator/`)

- **`config.py`:** Çözünürlükler, 9-slice kenar mesafeleri (corner insets), renkler ve dosya yolları.
- **`noise.py`:** Deterministik mikro-gren (noise) ve fırçalanmış metal (brushed metal sheen) matematik fonksiyonları. Dikiş izi bırakmaz.
- **`buttons.py`:** Primary (Altın), Royal (Perçinli Çift Çerçeve), Secondary (Arduvaz Çelik), Danger (Yakut Kırmızı) butonlarının Normal, Hover ve Pressed durumlarını üretir.
- **`panels.py`:** %100 Opak (Alpha=255) zeminli Ornate Gold ve Slate Card çerçeve dokularını üretir.
- **`slicer.py`:** Bevy'nin `TextureSlicer` mantığını Python tarafında birebir simüle eden 9-slice gerdirme motoru.
- **`preview.py`:** Tüm buton varyantlarını, durumlarını ve panelleri gerilmiş halde tek bir görselde toplayan sunum sayfası oluşturucu.

---

## 🛠️ İleride Yeni Bileşen veya Doku Eklerken

1. **Yeni bir panel tipi (örn. `panel_stone`):**
   - `scripts/ui_generator/panels.py` içine yeni variant parametresi eklenir.
   - Kenarların ve zeminin %100 opak kalması için `final_a = 255` korunur.
2. **Yeni bir buton tipi (örn. `button_emerald`):**
   - `scripts/ui_generator/buttons.py` içine renk ve rim gradyanları eklenir.
3. `python3 scripts/generate_ui_textures.py` çalıştırılarak hem oyun asset'leri hem de önizleme sayfası anında güncellenir.
