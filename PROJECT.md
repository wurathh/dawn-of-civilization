# Dawn of Civilization — Proje Kılavuzu ve Bilgi Hafızası

> **Proje Adı:** Dawn of Civilization  
> **Tür:** Grand Strategy (Büyük Strateji / AoH3 & HoI4 Benzeri)  
> **Programlama Dili & Sürüm:** Rust (2024 Edition)  
> **Oyun Motoru:** Bevy Engine `0.20.1`  
> **Son Güncelleme:** Kasım 2026  

---

## 1. Projenin Amacı ve Vizyonu

**Dawn of Civilization**, *Age of History 3 (AoH3)* ve *Hearts of Iron IV (HoI4)* tarzında; derinlikli jeopolitik, dinamik siyasi harita boyama, gerçekçi eyalet sınırları, akıcı kamera kontrolleri ve yüksek performanslı simülasyon sunan modern bir **Büyük Strateji (Grand Strategy)** oyunudur.

### Temel Hedefler
- **Devasa Dünya Haritası:** 11.600 x 4.218 piksel çözünürlüğünde, binlerce eyalet içeren gerçek dünya projeksiyonu.
- **Yüksek Performans & Düşük Bellek:** RLE (Run-Length Encoding) ile sıkıştırılmış eyalet verisi ve optimize edilmiş çok katmanlı render altyapısı (60+ FPS stabil).
- **Sonsuz Yatay Döngü (Infinite Wrapping):** Pasifik Okyanusu'nda sınır hissi olmadan kesintisiz dünya turu.
- **Kartografik Tipografi ve Estetik:** Ülkelerin coğrafi sınırlarına tam oturan, eğik, dikey veya yatay dinamik ülke isimleri ve yakınlaşınca beliren şık başkent rozetleri.
- **Dinamik Siyasi Simülasyon:** 200'den fazla ülke, eyalet mülkiyetleri, nüfus, ekonomi ve gerçek zamanlı siyasi harita renklendirmesi.

---

## 2. Teknoloji Yığını ve Derleme Altyapısı

| Katman | Teknoloji / Kütüphane | Açıklama |
| :--- | :--- | :--- |
| **Dil** | Rust 2024 Edition | Güvenli bellek yönetimi ve maksimum hız |
| **Oyun Motoru** | Bevy `0.20.1` | ECS mimarisi, 2D render, font & UI sistemi |
| **Serileştirme** | `serde`, `serde_json` | Senaryo ve eyalet veri dosyalarının okunması |
| **Profil & Optimizasyon** | `bevy/dynamic_linking` | Geliştirme sürecinde saniyeler içinde hızlı linkleme |
| **Bağımlılık Opt.** | `opt-level = 3` (dev paketler) | Bevy motor bileşenlerinin geliştirme modunda da akıcı çalışması |

### Çalıştırma ve Derleme Komutları
```bash
# Geliştirme modunda hızlı çalıştırma:
cargo run

# Hızlı derleme kontrolü:
cargo check

# Yayın (Release) derlemesi:
cargo build --release
```

---

## 3. Proje Mimarisi ve Kaynak Kod Dizini

```
Dawn of Civilization/
├── assets/                       # Oyun varlıkları (Dokular, fontlar, sesler, senaryolar)
│   ├── backgrounds/              # Menü için rastgele yüklenen tarihi/tematik görseller
│   ├── flags/                    # Ülke bayrakları hiyerarşisi ({tag}/0.png)
│   ├── fonts/                    # EBGaramond-ExtraBold, Inter-Bold, Inter-SemiBold vb.
│   ├── map/                      # Harita dokuları, sınır çizgileri, eyalet JSON ve RLE verisi
│   └── scenarios/modernworld/    # Modern Dünya senaryosu (201 ülke, eyalet dağılımları)
├── src/                          # Kaynak kodlar
│   ├── main.rs                   # Bevy App başlatıcı, pencere ayarları, sistem pluginleri
│   ├── state.rs                  # Uygulama durumları (AppState), UI fontları ve oturum verisi
│   ├── scenario.rs               # Senaryo okuyucu, ülke modelleri, mülkiyet tabloları, bayrak yolu
│   ├── map.rs                    # Harita motoru, RLE ikili arama, kamera, etiketler ve rozetler
│   ├── menu.rs                   # Ana menü, senaryo & ülke seçim arayüzü, HUD ve renk paleti
│   └── fps.rs                    # Canlı FPS ve kare süresi (ms) tanılama sayacı
├── Cargo.toml                    # Bağımlılıklar ve profil yapılandırmaları
└── PROJECT.md                    # Bu doküman (Proje Hafızası)
```

---

## 4. Temel Sistemler ve Tasarım Detayları

### 4.1. Harita Render ve Sonsuz Döngü Mimarisi ([src/map.rs](file:///home/wurathh/Desktop/Dawn%20of%20Civilization/src/map.rs))
- **Harita Boyutları:** `11600 x 4218` piksel.
- **GPU Doku Bölümlemesi:** Birçok GPU tek parça 11.600px dokuyu desteklemediği için harita Batı (`west`) ve Doğu (`east`) olarak iki adet `5800 x 4218` parçaya bölünmüştür.
- **Z-Order Katmanları:**
  - `Z = 0.0`: Fiziksel Topografya Haritası (`world_west.png`, `world_east.png`)
  - `Z = 1.0`: Eyalet Sınırları Çizgisi (`borders_west.png`, `borders_east.png` — LOD ile yakınlaşınca görünür)
  - `Z = 2.0`: Dinamik Siyasi Harita Katmanı (CPU üzerinde RGBA tamponu oluşturulup GPU'ya yüklenir)
  - `Z = 3.0`: Eyalet Hover & Seçim Vurgu Katmanı
  - `Z = 4.2`: Büyük Harita Ülke İsimleri ([`CountryMapLabel`](file:///home/wurathh/Desktop/Dawn%20of%20Civilization/src/map.rs#L134-L137))
  - `Z = 4.4 - 4.5`: Başkent Rozetleri ([`CapitalCityBadge`](file:///home/wurathh/Desktop/Dawn%20of%20Civilization/src/map.rs#L139-L142))
  - `Z = 100.0`: Dinamik Eyalet Hover Tooltip Kartı ([`ProvinceHoverTooltipRoot`](file:///home/wurathh/Desktop/Dawn%20of%20Civilization/src/map.rs#L145-L163))
  - Eyalet seçildiğinde gereksiz sarı işaret kutusu kaldırılmış, temiz eyalet sınır vurgusu ve bilgi paneli korunmuştur.
- **Sonsuz Yatay Döngü (Horizontal Infinite Wrapping):**
  - Dünya 3 kopya halinde spawn edilir: Sol (`WrapCopy(-1)`), Merkez (`WrapCopy(0)`), Sağ (`WrapCopy(1)`).
  - Kamera `cam_tf.translation.x` sınırları aştığında koordinatlar `MAP_WIDTH_F32` kadar kaydırılarak kesintisiz sonsuz döngü sağlanır.

### 4.2. RLE Eyalet Tespiti ve Piksel Sorgulama
- `provinces.rle` dosyasında her piksel satırı `(start_x: u16, province_id: u16)` çiftleri halinde saklanır.
- Fare harita üzerindeyken farenin dünya koordinatı piksel koordinatına çevrilir (`world_to_map_pixel`).
- `WorldMapData::get_province_at(px, py)` fonksiyonu, ilgili satırdaki aralıklar üzerinde `binary_search_by_key` çalıştırarak **O(log N)** sürede anında eyaleti bulur.

### 4.3. Age of History 3 & Paradox Tarzı Kavisli / Dairesel Ülke İsimleri
- **Kavisli / Eğimli Harf Dizilimi (Curved & Arched Typography):**
  - Rusya (Sibirya yayı), Kanada, ABD, Brezilya, Avustralya, Türkiye (Anadolu kavisi), Cezayir (Sahra yayı), İtalya, Vietnam vb. ülkelerde metinler yay/daire ekseninde kavisli dizilir.
  - Her harf ana eksene teğet açı (`slope.atan()`) ve normal vektör doğrultusunda (`PlacedCharInfo`) matematiksel olarak yerleştirilir.
  - Çok satırlı ülkelerde (örn. ABD, Suudi Arabistan) satırlar eşmerkezli paralel kavis çizer.
- **Kesin Sınır Taşma Koruması & Dinamik Alan Büyütmesi:**
  - Ülke sınırlarının dışına (örn. Mali'nin Moritanya'ya, Demokratik Kongo'nun komşu ülkelere) taşmasını kesin olarak önlemek için kovaryans matrisinin özdeğerlerinden çekirdek yarıçapları (`sigma_major`, `sigma_minor`) hesaplanır.
  - Güvenli çekirdek alanı `safe_len = (spine_len * 0.60).min(sigma_major * 2.15)` ve `safe_thick = (spine_thick * 0.54).min(sigma_minor * 1.75)` olarak belirlenir.
  - Geniş coğrafi alana ve ferah topraklara sahip ülkelerde (Rusya: 140px, Kanada: 140px, Brezilya: 122px, Avustralya: 107px, Kazakistan: 91px, ABD: 80px, Cezayir: 69px, Türkiye: 44px, Mısır: 45px, Fransa: 41px vb.) isimler haritada heybetli ve kartografik olarak büyük görünür.
  - Sıkışık veya dar coğrafyalarda ise `safe_len` ve `safe_thick` sınırları devreye girerek fontu küçültür ve sınırların dışına taşmasını %100 engeller.
  - Mali'nin etiketi batıdaki sınır kıstırılmasından geniş doğu Sahra/Timbuktu merkezine (`5720, 2240`, +20°) alınarak Moritanya ile arasına 200+ px güvenlik mesafesi konulmuştur.
  - Batı Sahra (`BATI SAHRA`) tek satıra getirilmiş, kıyı/çöl şeridine uygun diagonal açıya (`5365, 2060`, +34.4°) oturtulmuş ve eğrilik bozulması sıfırlanmıştır.
  - Demokratik Kongo gibi uzun isimli ülkelerin font boyutu çekirdek alana tam sığacak şekilde optimize edilmiştir (`line_w` havza genişliğinin yarısını aşmaz).
- **Klasik Kartografik SİYAH Renk Sistemi:**
  - Kullanıcının kesin tercihi doğrultusunda tüm haritada istisnasız net, klasik ve asil **kartografik SİYAH** (`Color::srgba(0.08, 0.08, 0.10, 0.92)`) uygulanır; beyaz ülke adı yazımı tamamen kaldırılmıştır.
  - Başkent rozetlerinin metinleri ise gözü yormayan yumuşak parşömen tonundadır (`Color::srgb(0.80, 0.79, 0.75)`).
- **Hiyerarşik Visibility İletimi (`B0004` Çözümü):**
  - Ebeveyn etiket varlığına `Visibility::Inherited` eklenerek Bevy ECS hiyerarşi uyarıları tamamen giderilmiştir.

### 4.4. Başkent Rozetleri Sistemi (Capital City Badges)
- **Yuvarlak Köşeli ve Altın Kenarlıklı (Border & Radius) Estetik Tasarım:**
  - Başkent rozetleri düz dikdörtgen kutular yerine CPU üzerinde Signed Distance Field (SDF) ile antialiased olarak oluşturulan `5.5px` köşe yarıçapına ve `1.2px` altın/pirinç çerçeveye (`[178, 148, 92, 220]`) sahip dokularla çizilir.
  - Arka plan yarı saydam koyu lacivert/antrasit tonundadır (`[12, 16, 24, 235]`); metinler Inter-Bold tipografisiyle parşömen beyazı tonunda okunur.
- **Global Eliptik Spiral Anti-Collision Motoru (Tüm Dünya İçin Çakışma Önleyici):**
  - Başkent rozetlerinin dünya üzerindeki hiçbir noktada birbiriyle üst üste binmemesi matematiksel olarak garanti altına alınmıştır:
    1. Öncelikle adanın veya şehrin doğal koordinatı ve mikro-bölgesel yön tercihleri (KKTC/Güney Kıbrıs, İsrail/Filistin/Ürdün, Vatikan/Roma, Monako, Kongo/DRC vb.) test edilir.
    2. Eğer herhangi bir çakışma tespit edilirse, solver eliptik spiral arama halkaları (R: 15px .. 190px, 24 aşamalı 16 açısal yön) ile en yakın çakışmasız pozisyonu anında bulup yerleştirir.
    3. Normal koordinatların yanı sıra haritanın doğu ve batı sonsuz döngü kopyalarındaki (`MAP_WIDTH_F32`) çakışmalar da eşzamanlı taranır. Dünyanın hiçbir yerinde başkent rozetleri birbiriyle çakışmaz.

### 4.5. İmleç Altı Dinamik Şehir & Eyalet Hover Tooltip Sistemi ([`ProvinceHoverTooltipRoot`](file:///home/wurathh/Desktop/Dawn%20of%20Civilization/src/map.rs#L145-L152))
- **Minimalist ve Net Tasarım (Yalnızca Bayrak + İl İsmi):**
  - Kullanıcı isteği doğrultusunda alt bilgi kalabalığı (kıta, kalkınma, ülke adı vb.) tamamen kaldırılarak kart tek bir satırda sadece ait olduğu ülkenin **Bayrağı (18x12px)** ve okunaklı **Şehir / İl İsmi** olarak sadeleştirilmiştir.
  - Kart altın/çelik zarif çerçeveye (`border: 1.2px`), yumuşak yuvarlak köşelere (`border_radius: 7.0px`) ve koyu arka plana sahiptir.
- **Akıllı Gecikmeli Açılma (Anti-Flicker Delay ~0.30s):**
  - Fare harita üzerinde hızla kaydırılırken tooltip'lerin ekranda ardı ardına yanıp sönmesini engellemek için `ProvinceHoverTooltipTimer` ile 0.30 saniyelik mikro bekleme süresi uygulanır.
  - İmleç belirli bir şehir/eyalet üzerinde durduğunda kart pürüzsüzce açılır; fare açık denize, UI'a veya başka bir bölgeye kaydığı anda anında gizlenir.
- **Ekran Kenar Koruması (Boundary Clamping):**
  - Tooltip imlecin `+16px, +20px` sağ-altında konumlanır.
  - Pencerenin sağ veya alt kenarına yaklaşıldığında taşma yapmaması için otomatik olarak imlecin soluna veya üstüne ötelenir.
- **Ölçülü Dikey Kaydırma/Scroll Alanı (Vertical Camera Margins):**
  - Haritanın en kuzey ve en güney uçlarındaki şehirlere (İskandinavya, Kuzey Rusya, Grönland, Patagonya/Şili vb.) rahatça odaklanılabilmesi için dikey sınırlayıcıya gereksiz boşluk yaratmayacak ölçülü bir kaydırma payı (`margin_y = half_view_h_target * 0.12 + 80.0`) uygulanmıştır.
- **Akıllı Kıyı ve Sınır Tıklama Toleransı (Click Snapping):**
  - `province_interaction` sisteminde 3-4 piksellik arama yarıçapı ile kıyı ve sınır pikselleri akıllıca en yakın ile yakalanır (`snap`). Küçük adalar ve kıyı kasabaları tek tıkla kolayca seçilebilir.

### 4.6. Siyasi Harita Boyama & Renk Değiştirme ([src/map.rs](file:///home/wurathh/Desktop/Dawn%20of%20Civilization/src/map.rs#L1820-L1900))
- `recolor_country_on_map` fonksiyonu, bir ülkenin tüm eyalet piksellerini CPU üzerindeki Batı ve Doğu doku arabelleğinde yeni renkle günceller ve dokuyu anında yeniden GPU'ya gönderir.
- Menüdeki "Rengi Değiştir" butonu ile herhangi bir ülkenin siyasi harita rengi çalışma zamanında canlı olarak rastgele üretilen estetik renklerle güncellenebilir.

### 4.6. Menü ve Senaryo / Ülke Seçim Mimarisi ([src/menu.rs](file:///home/wurathh/Desktop/Dawn%20of%20Civilization/src/menu.rs), [src/state.rs](file:///home/wurathh/Desktop/Dawn%20of%20Civilization/src/state.rs))
- **Ekran ve Durum Akışı (State Flow):**
  1. `AppState::MainMenu` (Ana Menü):
     - "Dünya Haritasını İncele" butonu tamamen kaldırılmıştır.
     - "Yeni Oyun Oluştur" butonuna basıldığında doğrudan `AppState::ScenarioCatalog` ekranına geçilir.
  2. `AppState::ScenarioCatalog` (Senaryo Kataloğu Seçim Ekranı):
     - Arka planda tematik görsel, şık başlık ve senaryo kartları sunulur.
     - "Modern Dünya (2026)" senaryo kartı: 201 medeniyet, 4.412 eyalet, modern sınırlar, detaylı istatistik rozetleri.
     - "Senaryoyu Başlat & Haritaya Geç ▶" butonu ile senaryo yüklenir ve harita üzerinde `AppState::ScenarioSelect` ekranına geçilir.
     - "← Ana Menüye Dön" butonu ile ana menüye dönüş sağlanır.
  3. `AppState::ScenarioSelect` (Harita Üzerinde Ülke Seçim Ekranı):
     - Harita 3D/2D ortamda aktiftir; kamera sürüklenebilir, yakınlaşılabilir ve herhangi bir ülkeye veya eyalete tıklanarak ülke seçilebilir.
     - **Sol Taraf (Ülke Bilgi Dosyası / Dossier Popup):**
       - Ülke bayrağı (`flagOver.png` dalgalanma efektiyle) ve başkent bilgisi.
       - Devlet Başkanı / Cumhurbaşkanı adı ve unvanı (örn. Recep Tayyip Erdoğan, Joe Biden, Vladimir Putin, Xi Jinping vb.).
       - Yönetim Biçimi (Cumhuriyet, Monarşi, Federal Cumhuriyet vb.) ve Dini (İslam, Hristiyanlık, Ortodoks, Budizm vb.).
       - İstatistikler: Eyalet sayısı, toplam nüfus, ekonomi hacmi ve teknoloji seviyesi.
       - Diplomatik Durum: Müttefikler ve Rakipler listesi.
       - Dinamik Siyasi Renk Paleti: 6 renk tonu arasından harita rengini anında değiştirme butonları.
     - **Sağ Üst Taraf (Oyun Ayarları & Modal):**
       - "⚙ Oyun Ayarları" butonu; tıklandığında ayar modalı açılır/kapanır.
       - Savaş Sisi (Açık / Kapalı / Klasik), Zorluk Derecesi (Kolay / Normal / Zor / Efsanevi), Yapay Zeka Agresifliği (Düşük / Dengeli / Agresif) ve Demir Adam Modu (Açık / Kapalı) butonları ile canlı ayar yapılandırması.
     - **Alt Gezinme Çubuğu (Bottom Bar):**
       - Sol Altta: "← Senaryolara Dön" butonu (Kataloğa geri döner).
       - Ortada: "Önerilen / En Güçlü Ülkeler" hızlı seçim butonları (Türkiye, ABD, Rusya, Çin, Almanya, Birleşik Krallık, Fransa, Azerbaycan).
       - Sağ Altta: "OYNA ▶" butonu (Seçilen ülke ile `AppState::InGame` başlatılır).
  4. `AppState::InGame` (Oyun İçi):
     - Gerçek zamanlı oyun HUD'ı, seçilen ülke yönetimi ve simülasyon.

---

## 5. Kritik Bevy ve Rust Kuralları (Geliştirici İpuçları)

1. **ECS Query Çakışmalarına Dikkat Edin (`B0001` Hatası):**
   - Bir sistemde hem `Single<&Transform, With<Camera2d>>` hem de başka varlıkların `Query<(&mut Transform, ...)>` sorgusu varsa, mutable query'e mutlaka `Without<Camera2d>` filtresi eklenmelidir. Aksi takdirde Bevy ECS aynı bileşene çakışan erişim gerekçesiyle çalışma anında panikler (`core dumped`).
2. **Anchor Sabitleri:**
   - Bevy 0.20 sürümünde `Anchor` bileşeni sabitleri büyük harflidir: `Anchor::CENTER_LEFT`, `Anchor::CENTER`, `Anchor::BOTTOM_CENTER`.
3. **Parent-Child Ölçekleme:**
   - Rozet veya etiket gibi grup bileşenlerinde alt elemanlara ayrı ayrı `Transform` scale vermek yerine, her zaman ebeveyn varlığı ölçekleyin. Çocuklar `Transform::from_xyz(local_x, local_y, local_z)` şeklinde yerel koordinatlarda kalmalıdır.
4. **Visibility İletimi:**
   - Ebeveyn varlığa `Visibility::Hidden` veya `Visibility::Inherited` uygulandığında, tüm çocuklar otomatik olarak aynı görünürlük durumunu devralır.

---

## 6. Gelecek Yol Haritası (Roadmap)

- [ ] **Askeri & Ordu Mekaniği:**
  - Eyaletlerde tümen/tugay üretimi.
  - Birliklerin eyaletler arasında intikali ve savaş simülasyonu.
- [ ] **Diplomasi & Barış Masası:**
  - Savaş ilanı, saldırmazlık paktları, ittifaklar.
  - Savaş sonu müzakereleri, toprak ilhakı veya kukla devlet (puppet) kurma.
- [ ] **Ekonomi & Nüfus Simülasyonu:**
  - Vergi oranları, kalkınma yatırımları, eyalet altyapısı ve ordu bakım maliyetleri.
  - Tur atlama mekanizması (Turn-based / Real-time with Pause).
- [ ] **Kaydet & Yükle (Save / Load):**
  - Oyun durumu serileştirmesi (JSON veya ikili format).
- [ ] **Yeni Senaryolar & Çağlar:**
  - 1444, 1836 (Viktorya), 1936 (II. Dünya Savaşı) gibi farklı tarihi dönem senaryoları.
