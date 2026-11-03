# Dawn of Civilization — UI Tasarım Dili (Obsidyen & Bronz)

Bu doküman, projenin tüm kullanıcı arayüzü (UI) bileşenlerinin tek referans kaynağıdır. Tüm ekranlar, menüler ve HUD parçaları burada tanımlanan kurallara ve tasarım token'larına uymak zorundadır.

---

## 1. Tasarım Felsefesi ve Ruh Hali
- **Hedef:** Modern, ağırbaşlı, minimal ve lüks bir grand strategy arayüzü.
- **Referans:** Paradox (Crusader Kings III, Victoria 3) ve modern lüks dashboard estetiği.
- **İlke:** Canlı renkli siyasi dünya haritasının üzerinde duran yarı saydam koyu cam yüzeyler (frosted obsidian), sıcak bronz/kehribar aksanlar, net hiyerarşi ve nefes alan 4px ızgara sistemi.

---

## 2. Renk Paleti (Obsidyen & Bronz)

| Token Adı | HEX / RGBA Değeri | Açıklama & Gerekçe |
|---|---|---|
| `bg_base` | `#080C14` / `rgba(8, 12, 20, 1.0)` | En derin arka plan; harita dışı menü zeminleri için koyu obsidyen. |
| `surface_flat` | `rgba(14, 20, 32, 0.85)` | Cam hissi veren yarı saydam standart panel yüzeyi; alttaki haritayı sezdirir ama metni net korur. |
| `surface_raised` | `rgba(21, 29, 44, 0.92)` | Modallar, vurgulu kartlar veya iç kutucuklar için bir kat yükseltilmiş yüzey. |
| `surface_overlay` | `rgba(6, 9, 15, 0.96)` | Tooltip ve dropdown menüler gibi en üst katman zeminleri. |
| `border_subtle` | `rgba(38, 50, 70, 0.55)` | İkincil elemanlar ve iç ayrımlar için düşük kontrastlı soğuk çelik kenarlık. |
| `border_default` | `rgba(50, 66, 92, 0.80)` | Standart kart ve panel çerçevesi; panele net form kazandırır. |
| `border_accent` | `rgba(217, 165, 74, 0.85)` | Bronz/altın vurgulu odak ve birincil buton kenarlığı. |
| `text_primary` | `#ECE8DF` / `rgba(236, 232, 223, 1.0)` | Yumuşak fildişi metin; saf beyazın göz yormasını engeller, yüksek kontrast sunar. |
| `text_secondary` | `#9BA3B0` / `rgba(155, 163, 176, 1.0)` | İkincil açıklamalar, meta veriler ve etiketler için soğuk arduvaz grisi. |
| `text_disabled` | `#525B6C` / `rgba(82, 91, 108, 1.0)` | Devre dışı kalmış buton ve pasif metinler. |
| `accent` | `#D9A54A` / `rgba(217, 165, 74, 1.0)` | **Tek ana vurgu rengi**: Tarihî ve asil sıcak bronz/kehribar. |
| `accent_hover` | `#EDB85E` / `rgba(237, 184, 94, 1.0)` | Vurgu elemanlarının hover durumu. |
| `accent_pressed` | `#B88532` / `rgba(184, 133, 50, 1.0)` | Vurgu elemanlarının tıklandığı an. |
| `success` | `#38A169` / `rgba(56, 161, 105, 1.0)` | Olumlu durumlar, kalkınma artışı, müttefik statüsü. |
| `warning` | `#DD6B20` / `rgba(221, 107, 32, 1.0)` | Dikkat gerektiren uyarılar, düşük istikrar. |
| `danger` | `#E53E3E` / `rgba(229, 62, 62, 1.0)` | Savaş ilanı, fetih, silme ve tehlike butonları. |

---

## 3. Tipografi Sistemi

Projede yalnızca iki font ailesi kullanılır:
1. **Display & Headline:** `EB Garamond ExtraBold` (İmparatorluk, tarih ve heybet hissi veren klasik serif).
2. **UI & Gövde:** `Inter` (Bold, SemiBold, Medium) (Ekran netliği yüksek, geometrik modern sans-serif).

### Tipografi Ölçeği

| Düzey | Font Ailesi | Ağırlık | Boyut (px) | Kullanım Alanı & Gerekçe |
|---|---|---|---|---|
| `display` | EB Garamond | ExtraBold | `36.0` | Ana menü oyun başlığı, büyük zafer ekranları. |
| `headline` | EB Garamond | ExtraBold | `24.0` | Panel ana başlıkları, modal başlıkları. |
| `title` | Inter | Bold | `18.0` | Kart başlıkları, seçili ülke adı. |
| `label` | Inter | Bold | `14.0` | Buton metinleri, sekme başlıkları, vurgulu istatistikler. |
| `body` | Inter | SemiBold | `13.0` | Standart liste öğeleri, açıklamalar, tablo satırları. |
| `caption` | Inter | Medium | `11.0` | Tooltip alt metinleri, dipnotlar, sayaç rozetleri. |

---

## 4. Boşluk Sistemi (4px Tabanlı Katı Izgara)

UI genelinde keyfi piksel değerleri yasaktır. Sadece aşağıdaki 4px katları kullanılır:

| Token | Değer | Kullanım |
|---|---|---|
| `space_2xs` | `4.0px` | İkon ile metin arası, mikro rozet iç boşluğu. |
| `space_xs`  | `8.0px` | Buton dikey padding'i, kompakt eleman aralıkları. |
| `space_sm`  | `12.0px`| Buton yatay padding'i, form girdi alanları. |
| `space_md`  | `16.0px`| Standart kart içi padding, satır boşlukları. |
| `space_lg`  | `24.0px`| Bölüm araları, büyük kart içi boşluklar. |
| `space_xl`  | `32.0px`| Ekran kenar boşlukları, modal iç padding'i. |
| `space_2xl` | `48.0px`| Ana menü kart aralıkları, ekran blokları arası mesafe. |

---

## 5. Köşe Yuvarlaklığı (Border Radius)

Aşırı yuvarlak "oyuncak" hissi engellenip modern lüks cam köşeler sağlanır:

| Token | Değer | Kullanım |
|---|---|---|
| `radius_sm` | `4.0px` | Bayraklar, mikro rozetler, küçük etiketler. |
| `radius_md` | `8.0px` | Butonlar, girdi kutuları, liste satırları, tooltip. |
| `radius_lg` | `14.0px`| Paneller, modallar, ana menü kartları. |

---

## 6. Kenarlık ve Katmanlama Mantığı

- **Kenarlık Kalınlığı:**
  - `border_thin`: `1.0px` (İç kutular, ayrım çizgileri, ikincil elemanlar).
  - `border_default`: `1.5px` (Butonlar, ana kart ve modal çerçeveleri).
- **Z-Index Katman Hiyerarşisi:**
  - `Z = 0`: Arka plan / Harita.
  - `Z = 10`: Standart HUD ve sabit paneller.
  - `Z = 20`: Yüzen pencereler ve yan çekmeceler.
  - `Z = 50`: Modallar ve onay pencereleri.
  - `Z = 100`: Tooltip'ler ve bildirimler (Toast).

---

## 7. Etkileşim Durumları (Interaction States)

Her etkileşimli bileşen (buton, toggle, sekme vb.) şu 6 duruma görsel yanıt üretir:
1. **Normal:** Standart zemin ve ince kenarlık.
2. **Hover:** Zemin hafif aydınlanır (+%10 parlaklık), kenarlık bronz vurguya döner, imleç el şeklini alır.
3. **Pressed:** Zemin hafif koyulaşır, kenarlık parlaklığı artar.
4. **Focused:** Bronz dış çerçeve halkası (`border_accent` 1.5px) belirir (gamepad & klavye odak noktası).
5. **Selected / Active:** Vurgu rengi zemin veya parlayan bronz şerit.
6. **Disabled:** Opaklık %40'a iner, metin `text_disabled`, tıklama yoksayılır (`Pickable::IGNORE`).

---

## 8. Hareket ve Geçişler (Motion)

- Hover geçişleri: `100ms - 150ms` (gecikmesiz, hızlı ve keskin geri bildirim).
- Modal / Panel açılış-kapanış: `180ms - 220ms` yumuşak ease-out.
- Asla abartılı veya dikkat dağıtıcı animasyon yapılmaz; strateji oyunlarında hız esastır.

---

## 9. Gamepad ve Klavye Navigasyonu

- Her odaklanabilir öğe `Focusable` bileşeni taşır.
- Klavye (Tab / Ok tuşları) ve Gamepad D-Pad ile odakta olan elemanın etrafında belirgin bronz odak çerçevesi çizilir.
- `Enter` veya Gamepad `A / Cross` tuşu seçimi tetikler.
