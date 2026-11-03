# DAWN OF CIVILIZATION — UI GELİŞTİRİCİ & AGENT KURALLARI (UI_RULES.md)

Bu kılavuz, Dawn of Civilization projesinde arayüz (UI) geliştiren, değiştiren veya refactor eden tüm agent'lar ve geliştiriciler için **kesin ve bağlayıcı kuralları** içerir.

---

## 1. Altın Kurallar (Strict Commandments)

1. **Hardcoded Değer Kesinlikle Yasaktır:**
   - Kod içine doğrudan `Color::srgb(...)`, `Color::srgba(...)`, `Color::srgb_u8(...)`, `Val::Px(17.5)` gibi keyfi ham değerler yazılamaz.
   - Tüm renkler, boyutlar, boşluklar, kenarlık kalınlıkları, köşe yuvarlaklıkları ve tipografi font boyutları **sadece ve sadece** `crate::ui::theme::Theme` resource'u veya token'larından (`theme.colors.*`, `theme.spacing.*`, `theme.radius.*`, `theme.borders.*`, `theme.typography.*`) alınacaktır.

2. **Yalnızca Merkezi UI Kütüphanesini Kullan:**
   - Ekranlarda sıfırdan dağınık widget ağaçları kurmak yerine `crate::ui::components::*` içerisindeki widget'lar ve layout yardımcıları (`row`, `column`, `spacer_v`, `spacer_h`, `divider_h`, `spawn_button`, `panel_node`, `spawn_checkbox`, vb.) tercih edilmelidir.

3. **Yeni Component Gerekiyorsa Önce Kütüphaneye Ekle:**
   - Bir ekranda yeni bir UI deseni gerekiyorsa (örn. yeni bir Slider, Dropdown veya Kart tipi), bu desen doğrudan o ekranda hardcode edilmez.
   - Önce `src/ui/components/<yeni_bilesen>.rs` dosyası oluşturulup kütüphaneye dahil edilir.
   - Ardından `examples/ui_showcase.rs` dosyasına eklenip görsel olarak doğrulanır.
   - Son olarak asıl ekranda çağrılır.

4. **Görsel Doğrulama (Screenshot) Zorunludur:**
   - Herhangi bir UI değişikliği yapıldığında sadece `cargo check` ile yetinilemez. Oyun veya showcase çalıştırılarak ekran görüntüsü alınmalı ve eleştirel bir gözle görsel hiyerarşi, hizalama ve taşmalar kontrol edilmelidir.

---

## 2. Bevy 0.20 API Standartları (Kaçınılması Gerekenler)

Bevy 0.20 sürümünde UI mimarisi ciddi şekilde modernize edilmiştir. Aşağıdaki eski API'lerin kullanılması yasaktır:

| Eski / Yasak API | Modern Bevy 0.20 Karşılığı | Neden? |
| :--- | :--- | :--- |
| `NodeBundle { .. }` | `(Node { .. }, BackgroundColor(..), BorderColor(..))` | Bundle'lar Bevy 0.20'de deprecated edildi, bileşen tuple'ları kullanılmalıdır. |
| `ButtonBundle { .. }` | `(Node { .. }, BackgroundColor(..), UiButton { .. })` | Deprecated bundle. |
| `TextBundle { .. }` | `(Text::new(..), TextFont { .. }, TextColor(..))` | Modern Bevy 0.20 Text yapısı. |
| `Interaction` (Hovered, Pressed) | Pointer Observers (`On<PointerOver>`, `On<PointerOut>`, vb.) | `Interaction` enum'ı ve bundle'ı Bevy 0.20'de deprecated edilmiştir. |
| `ChildBuilder` | `ChildSpawnerCommands` | Closure parametresi Bevy 0.20'de `&mut ChildSpawnerCommands` olmalıdır. |
| `EntityCommands<'w, 's, 'a>` | `EntityCommands<'a>` | Bevy 0.20'de `EntityCommands` yalnızca 1 lifetime parametresi alır. |

---

## 3. Token Referans Kılavuzu (`Theme`)

### 3.1 Renk Paleti (`theme.colors.*`) — "Obsidyen & Bronz"
- **`bg_base`**: `#080C14` — Derin gece mavisi uzay/boşluk arka planı.
- **`surface_flat`** / **`surface_panel()`**: `#0E1420` (%85 Alpha) — Düz zemin kartları, liste satırları, form kutuları.
- **`surface_raised`**: `#151D2C` (%92 Alpha) — Yükseltilmiş ana modallar, popuplar, floating paneller.
- **`surface_overlay`**: `#06090F` (%96 Alpha) — Tooltip ve modal karartma katmanları.
- **`border_subtle`**: `rgba(38, 50, 70, 0.55)` — Düşük kontrastlı cam kenarlıkları.
- **`border_default`**: `rgba(50, 66, 92, 0.80)` — Normal panel ve buton kenarlıkları.
- **`border_accent`**: `rgba(217, 165, 74, 0.85)` — Bronz altın vurgu kenarlıkları (Hover/Focus).
- **`text_primary`**: `#ECE8DF` — Yüksek kontrastlı ana metin rengi (fildişi beyazı).
- **`text_secondary`**: `#9BA3B0` — İkincil metin, alt başlık ve etiketler.
- **`text_disabled`**: `#525B6C` — Pasif elemanlar ve sayaçlar.
- **`accent`**: `#D9A54A` — Tekil canlı bronz/altın vurgu rengi.
- **`accent_hover`**: `#EDB85E` — Hover durumunda bronz aydınlanma.
- **`accent_pressed`**: `#B88532` — Pressed durumunda bronz derinleşme.
- **`success`**: `#38A169` — Pozitif diplomasi, müttefikler, onay.
- **`warning`**: `#DD6B20` — İkazlar, dikkat durumları.
- **`danger`**: `#E53E3E` — Düşmanlar, savaş ilanı, fetih, çıkış butonları.

### 3.2 Boşluk Izgarası (`theme.spacing.*`) — 4px Tabanlı
Kesinlikle 4'ün katı olmayan piksel değerleri kullanılamaz!
- `theme.spacing.val_xxs()`: **4px** (Mikro boşluk, chip içi)
- `theme.spacing.val_xs()`: **8px** (Küçük element arası gap)
- `theme.spacing.val_sm()`: **12px** (Standart buton içi dikey boşluk)
- `theme.spacing.val_md()`: **16px** (Kart içi standart padding, grid gap)
- `theme.spacing.val_lg()`: **24px** (Panel arası geniş nefes payı)
- `theme.spacing.val_xl()`: **32px** (Geniş modal kenar boşluğu)
- `theme.spacing.val_xxl()`: **48px** (Ana menü ekran kenar boşlukları)

### 3.3 Köşe Yuvarlaklığı (`theme.radius.*`)
- `theme.radius.val_xs()`: **2px** (Mikro bayraklar, sayaçlar)
- `theme.radius.val_sm()`: **4px** (Badge, chip, input kutuları)
- `theme.radius.val_md()`: **8px** (Butonlar, küçük kartlar)
- `theme.radius.val_lg()`: **14px** (Ana paneller, modallar, floating menüler)

### 3.4 Kenarlık Kalınlığı (`theme.borders.*`)
- `theme.borders.thin`: **1.0px** (İç kartlar, form elemanları, ayrım çizgileri)
- `theme.borders.default`: **1.5px** (Ana paneller, butonlar, modallar)

### 3.5 Tipografi Hiyerarşisi (`theme.typography.*`)
- `display`: **36.0px** (EB Garamond ExtraBold — Oyun ana başlığı, sahne adları)
- `headline`: **24.0px** (EB Garamond ExtraBold — Kart ve modal başlıkları)
- `title_lg()`: **18.0px** (Inter Bold — Buton ana metinleri, önemli veriler)
- `label_md()`: **14.0px** (Inter Bold — Standart buton metinleri, form başlıkları)
- `body_md()` / `body_sm()`: **13.0px - 14.0px** (Inter SemiBold — Açıklama paragrafları, veri detayları)
- `caption`: **11.0px** (Inter Medium — Sürüm bilgisi, ipuçları, küçük sayaçlar)

### 3.6 Prosedürel Dokular & Altın Gradyanlar (`UiTextures`)
Görsel zenginlik ve derinlik için prosedürel olarak üretilen dokular kullanılır:
- `textures.divider_gold`: İki uca doğru sönen 2px altın filigran ayraç çizgisi (`spawn_gold_divider`).
- `textures.header_gold_glow`: Panel başlıklarının üstünden aşağıya süzülen altın ışıltı gradyanı.
- `textures.button_primary`: Lüks metalik buton dolgusu (üst kenar specular highlight).
- `textures.button_primary_hover`: Hover anında yanan altın buton gradyanı.
- `textures.panel_dark`: Kart ve panellere derinlik katan koyu arduvaz gradyanı.

### 3.7 Hover Kontrast Kuralı (Kesin Kural)
Bir buton veya kart hover edildiğinde metin rengi ile arka plan rengi ASLA aynı tona bürünemez:
- **Düz Primary Butonlar (`spawn_button`):** Normal, Hover ve Pressed durumlarında arka plan parlak altın/bronz olduğundan metin her zaman **koyu obsidyen siyahı** (`theme.colors.text_inverse()` / `#040608`) kalır.
- **Kart Butonları (`spawn_action_card_button`):** Normalde koyu zemin üzerinde altın başlık kullanır. Hover anında tüm kart altın sarısına büründüğünde başlık anında **koyu obsidyen siyaha** (`#040608`), alt açıklama **koyu arduvaza** (`#1E293B`), ikon rozeti ise **koyu obsidyen iç kutuya** döner.

---

## 4. Kodlama Örnekleri

### 4.1 Buton Spawn Etme (Standard Widget):
```rust
use crate::ui::components::{spawn_button, ButtonVariant, ButtonSize};

spawn_button(
    &mut parent,
    &fonts,
    &theme,
    "Senaryoyu Başlat ▶",
    ButtonVariant::Primary,
    ButtonSize::Large,
);
```

### 4.2 Kart & Panel Ağacı (Layout Yardımcılarıyla):
```rust
use crate::ui::components::{panel_node, PanelElevation, row, column, spacer_v, Space};

parent.spawn(panel_node(&theme, PanelElevation::Raised))
    .with_children(|card| {
        // Kart içeriği
        card.spawn(row(&theme, Space::Md))
            .with_children(|r| {
                // ...
            });
        spacer_v(card, &theme, Space::Sm);
    });
```

---

## 5. UI Doğrulama Kontrol Listesi (Checklist)

Bir ekranı tamamlamadan önce mutlaka şu kontrol listesini gözden geçirin:
- [ ] `cargo check` sıfır hata ve sıfır uyarı ile geçiyor mu?
- [ ] Koddaki tüm renkler `theme.colors`'tan mı geliyor?
- [ ] Tüm boşluklar ve paddingler `theme.spacing` 4px ızgarasına uyuyor mu?
- [ ] Buton veya interaktif elemanlarda hover ve focus durumları tanımlı mı?
- [ ] Çözünürlük değiştiğinde veya farklı en-boy oranlarında elementler üst üste biniyor mu?
- [ ] Ekran görüntüsü alınarak görsel olarak onaylandı mı?
