# Leiðbeiningar fyrir Myndbandsupptöku

Þetta skjal inniheldur leiðbeiningar um hvernig á að sýna virkni forritsins í myndbandi.

## Undirbúningur

1. Keyra forritið:
```bash
cargo run --release
```

2. Hafa gluggann í fullri stærð fyrir bestu myndgæði

## Sýnikennsla - Röð aðgerða

### 1. Opnun og Yfirlit (10 sek)
- Sýna aðalviðmót með 4 flipum
- Benda á titil "🏫 Búnaðarlisti Tækniskólans"
- Sýna navigation (📝 Skrá, 🔄 Uppfæra, 🗑️ Eyða, 📋 Birta)

### 2. Skrá Búnað - Borð (30 sek)
1. Velja "📝 Skrá" flipann
2. Velja "🪑 Borð" með radio button
3. Sýna **dropdown valmynd** fyrir hús:
   - Velja "Háteigsvegur"
4. Sýna **sleða** fyrir hæð:
   - Setja á 2
5. Sýna **sleða** fyrir herbergi:
   - Setja á 5
6. Sýna preview: "Staðsetning: H-25"
7. Slá inn verðmæti: 25000
8. Stilla fjölda sæta með **sleða**: 6
9. Smella á "✅ Skrá búnað"
10. Sýna grænt success message með ID

### 3. Skrá Búnað - Stóll (20 sek)
1. Velja "💺 Stóll"
2. Velja staðsetningu með dropdown + sleðum:
   - Hafnarfjörður, hæð 1, herbergi 10 (HA-110)
3. Verðmæti: 15000
4. Tegund stóls: Velja "Skólastóll" með **dropdown**
5. Skrá búnað
6. Sýna success message

### 4. Skrá Búnað - Skjávarpi (20 sek)
1. Velja "📽️ Skjávarpi"
2. Staðsetning: Skólavörðuholt, hæð 3, herbergi 15 (S-315)
3. Verðmæti: 150000
4. Lumens: 3500
5. Skrá búnað

### 5. Skrá Fleiri Hluti (30 sek)
Skrá fljótt nokkra hluti til viðbótar:
- Borð í H-202 fyrir 4, 20000 kr
- Stóll í H-202 Hægindastóll, 25000 kr
- Skjávarpi í HA-110, 2500 lumens, 120000 kr

### 6. Birta - Allur Búnaður (30 sek)
1. Fara í "📋 Birta" flipann
2. Velja "Allur búnaður"
3. Smella "🔍 Birta"
4. **Sýna töfluna** með öllum búnaði
5. Benda á **formatted úttak** með fulltum lýsingum:
   - "Borð með ID: 1, kostar 25000 kr., fyrir 6 manns og er staðsett í H-25"
6. Sýna að ID er lesanlegt (ekki brotið)

### 7. Röðun (Column Sorting) (40 sek)
1. **Smella á "Tegund ▲▼"**
   - Sýna að búnaður raðast eftir tegund
2. **Smella aftur á "Tegund ▲▼"**
   - Sýna reverse röðun (descending)
3. **Smella á "Verðmæti ▲▼"**
   - Sýna röðun eftir verði
4. **Smella á "Staðsetning ▲▼"**
   - Sýna röðun eftir staðsetningu
5. **Smella á "🔄 Endurstilla röðun"**
   - Sýna að fara aftur í default röðun

### 8. Síur (Filters) (60 sek)

**Eftir húsi:**
1. Velja "Eftir húsi" radio button
2. Velja "Hafnarfjörður" með dropdown
3. Birta - sýna bara HA búnað

**Eftir tegund:**
1. Velja "Eftir tegund"
2. Velja "Stóll"
3. Birta - sýna bara stóla

**Eftir stofu:**
1. Velja "Eftir stofu"
2. Velja hús: Háteigsvegur
3. Hæð: 2 (með sleða)
4. Herbergi: 2 (með sleða)
5. Birta - sýna búnað í H-202

**Eftir hæð:**
1. Velja "Eftir hæð"
2. Velja hús: Háteigsvegur
3. Hæð: 2 (með sleða)
4. Birta - sýna allan búnað á 2. hæð í Háteigsvegi

### 9. Leit (Search) (20 sek)
1. Velja "Allur búnaður"
2. Birta allt
3. Í **leitarreit** skrifa: "stóll"
4. Sýna að bara stólar birtast
5. Skrifa: "H-2"
6. Sýna búnað í H-2xx

### 10. Tölfræði (Statistics) - WOW! (30 sek)
1. Smella á "📊 Sýna tölfræði"
2. Sýna **tölfræðipanel** með:
   - 📦 Heildarfjöldi búnaðar
   - 💰 Heildarverðmæti
   - Sundurliðun eftir tegund með prósentum
   - Sundurliðun eftir húsi með prósentum
3. Smella "Fela tölfræði"

### 11. Uppfæra Staðsetningu (30 sek)
1. Fara í "🔄 Uppfæra" flipann
2. Slá inn ID: 1
3. Sýna **sama location input og við skráningu**:
   - Dropdown fyrir hús
   - Sleðar fyrir hæð og herbergi
4. Velja nýja staðsetningu: S-320
5. Smella "✅ Uppfæra staðsetningu"
6. Sýna success message
7. Fara í Birta og staðfesta breytinguna

### 12. JSON Export/Import (30 sek)
1. Í "📋 Birta" flipanum
2. Smella "💾 Vista í JSON"
3. Sýna success message
4. Opna `equipment.json` í text editor (optional)
5. Fara aftur í forritið
6. Smella "📂 Hlaða úr JSON"
7. Sýna imported count

### 13. Eyða Búnaði (20 sek)
1. Fara í "🗑️ Eyða" flipann
2. Slá inn ID til að eyða (t.d. 3)
3. Smella "❌ Eyða búnaði"
4. Sýna success message
5. Fara í Birta og sýna að búnaður er horfinn

### 14. Error Handling (20 sek)
1. Í Skrá: reyna að slá inn bókstafi í verðmæti
2. Sýna **rauða error message**
3. Í Uppfæra: reyna að uppfæra ID sem er ekki til
4. Sýna error message
5. Sýna að allar villur eru á íslensku

### 15. Visual Design - WOW Factor (15 sek)
Benda á:
- 🎨 Falleg layout með emojis
- ✓ Græn success messages með táknmyndum
- ✗ Rauð error messages með táknmyndum
- 📊 Tölfræðipanel með structured data
- 🔍 Search með real-time filtering
- ↕️ Sortable columns með visual feedback
- 📋 Striped table fyrir betri lesanleika

## Helstu Atriði að Leggja Áherslu Á

### Kröfur sem uppfylltar eru:
1. ✅ **Dropdown valmyndir** fyrir hús og stólategund
2. ✅ **Sleðar** fyrir hæð (0-9) og herbergi (0-99, max 99)
3. ✅ **Sama location input** í skráningu og uppfærslu
4. ✅ **Formatted úttak** - ekki "Table(id=1...)" heldur fallegar setningar
5. ✅ **Clickable columns** fyrir röðun með ascending/descending
6. ✅ **Reset röðunar** button
7. ✅ **Allar síur** í einum Display hluta með toggle options
8. ✅ **Allir print options**: allur búnaður, eftir húsi, eftir tegund, eftir stofu, eftir hæð
9. ✅ **Display og TryFrom** útfærslur fyrir allt
10. ✅ **Hvert struct/enum í sinni skrá**

### WOW Factor:
- 📊 **Tölfræðipanel** með live data
- 🔍 **Real-time search** filtering
- 🎨 **Professional visual design** með colors og emojis
- 💾 **JSON import/export** virkni
- ✅ **Excellent error handling** með íslenskum skilaboðum
- 📱 **User-friendly UI** með intuitive controls

## Tími: ~5-6 mínútur total

Ábending: Taka upp í 1080p eða hærra fyrir best quality!
