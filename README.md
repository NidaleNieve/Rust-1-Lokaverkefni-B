# Rust - Lokaverkefni B
## Daníel Snær Rodríguez
# Búnaðarlisti Tækniskólans


## Hugbúnaður til að halda utan um búnað Tækniskólans í Rust með myndrænu notendaviðmóti.

## Eiginleikar (GPT Generated)

### Kjarnaaðgerðir
- ✅ **Skrá búnað**: Borð, stóla og skjávarpa með öllu viðeigandi upplýsingum
- ✅ **Breyta Búnað**: Breyta staðsetningu búnaðar eða eyða honum
- ✅ **Prenta Búnað**: Sjá listann og filtera hann. Einnig hægt að prenta hann og vista hann sem JSON
- ✅ **SQLite gagnagrunnur**: Öll gögn vistuð í gagnagrunn, hægt er að vista og hlaða frá JSON skjai

### Birting og síun
- 📋 **Allur búnaður**: Skoða allan búnað í kerfinu
- 🏢 **Eftir húsi**: Sía búnað eftir Hafnarfirði, Háteigsvegi eða Skólavörðuholti
- 📦 **Eftir tegund**: Skoða bara borð, stóla eða skjávarpa
- 🚪 **Eftir stofu**: Skoða búnað í ákveðinni stofu
- 📊 **Eftir hæð**: Skoða búnað á tiltekinni hæð í húsi

### Ítarlegir eiginleikar
- 🔍 **Leitar virkni**: Leita að búnaði með leitarreit sem leitar gegnum allt
- ↕️ **Röðunareiginleikar**: Smelltu á dálkahausa til að raða eftir ID, tegund, staðsetningu eða verðmæti (hækkandi/lækkandi)
- 💾 **JSON útflutningur**: Vista öll gögn í JSON skrá
- 📂 **JSON innflutningur**: Hlaða gögnum úr JSON skrá
- 💾 **Prenta**: Hægt er að prenta síaða listann
- 📂 **Vista sem PDF**: Hægt er að vista síaða listann sem PDF
- 🎨 **Falleg sýn**: Vel sniðin úttak með fulltum lýsingum
- 📱 **Notendavænt viðmót**: Dropdown valmyndir og sleðar fyrir staðsetningar


## Hvernig á að keyra forritið

### Forsendur

Gakktu úr skugga um að þú hafir Rust uppsett. Ef ekki, sæktu það á [https://rustup.rs/](https://rustup.rs/)

### Keyra forritið

1. Sóttu verkefnið með því að ýta á græna "Code" takkan og svo "Download Zip"

2. Opnaðu möppuna og inni í henni opnaðu terminal/command prompt

2. keyrðu þessa skipun:
```bash
cargo run
```
### Byggja forritið

Til að byggja executable:
```bash
cargo build --release
```

Executable mun vera í `target/release/CLAUDE-Lokaverkefni-B`


## Tækniupplýsingar

### Verkefnaskipulag

Verkefnið er skipulagt með hverjum struct og enum í sinni eigin skrá:

```
src/
├── main.rs           # Aðal GUI forritið
├── equipment.rs      # Equipment enum
├── location.rs       # Location struct og Building enum
├── chair_type.rs     # ChairType enum
├── table.rs          # Table struct
├── chair.rs          # Chair struct
├── projector.rs      # Projector struct
└── database.rs       # SQLite gagnagrunnsvirkni
```

### Háðir pakkar

- **eframe** (0.28): GUI ramma byggður á egui
- **rusqlite** (0.32): SQLite gagnagrunnur
- **serde** + **serde_json** (1.0): JSON serialization
- **regex** (1.10): Staðfesting á staðsetningarsniði

## Notkun

### Skrá nýjan búnað

1. Veldu flipann "📝 Skrá"
2. Veldu tegund búnaðar (Borð, Stóll eða Skjávarpi)
3. Veldu staðsetningu með dropdown og sleðum:
   - **Hús**: Hafnarfjörður (HA), Háteigsvegur (H), eða Skólavörðuholt (S)
   - **Hæð**: 0-9
   - **Herbergi**: 0-99
4. Sláðu inn verðmæti í krónum
5. Sláðu inn viðbótarupplýsingar:
   - **Borð**: Fjöldi sæta (1-20)
   - **Stóll**: Tegund (Hægindastóll, Skólastóll, Skrifstofustóll, Annað)
   - **Skjávarpi**: Lumens
6. Smelltu á "✅ Skrá búnað"

### Uppfæra staðsetningu

1. Veldu flipann "🔄 Uppfæra"
2. Sláðu inn ID búnaðarins
3. Veldu nýja staðsetningu með sama viðmóti og við skráningu
4. Smelltu á "✅ Uppfæra staðsetningu"

### Eyða búnaði

1. Veldu flipann "🗑️ Eyða"
2. Sláðu inn ID búnaðarins
3. Smelltu á "❌ Eyða búnaði"

### Birta og leita að búnaði

1. Veldu flipann "📋 Birta"
2. Notaðu leitarreitinn til að leita
3. Veldu síu:
   - **Allur búnaður**: Sýnir allan búnað
   - **Eftir húsi**: Veldu hús
   - **Eftir tegund**: Veldu tegund búnaðar
   - **Eftir stofu**: Veldu hús, hæð og herbergi
   - **Eftir hæð**: Veldu hús og hæð
4. Smelltu á "🔍 Birta"
5. Smelltu á dálkahausa til að raða
6. Smelltu á "🔄 Endurstilla röðun" til að fara aftur í sjálfgefna röðun

### JSON virkni

- **Vista í JSON**: Smelltu á "💾 Vista í JSON" til að vista alla búnaðinn í `equipment.json`
- **Hlaða úr JSON**: Smelltu á "📂 Hlaða úr JSON" til að flytja inn búnað úr `equipment.json`

## Display og TryFrom útfærslur

Allir structs útfæra `Display` trait til að birta fallegar lýsingar:

**Dæmi:**
```
Borð með ID: 1, kostar 25000 kr., fyrir 6 manns og er staðsett í H-202
Stóll með ID: 2, kostar 15000 kr., af gerðinni Skólastóll og er staðsettur í HA-101
Skjávarpi með ID: 3, kostar 150000 kr., með 3500 lúmens og er staðsettur í S-310
```

`Location` struct útfærir `TryFrom<&str>` til að umbreyta textastreng í staðsetningu með villumeðferð.

## Staðsetningarsnið

Staðsetningar eru á sniðinu: `{HÚS}-{HÆÐ}{HERBERGI}`

**Dæmi:**
- `H-202` = Háteigsvegur, 2. hæð, herbergi 2
- `HA-123` = Hafnarfjörður, 1. hæð, herbergi 23
- `S-350` = Skólavörðuholt, 3. hæð, herbergi 50

**Húsmerki:**
- `HA` - Hafnarfjörður
- `H` - Háteigsvegur
- `S` - Skólavörðuholt

## Gagnagrunnur

Gögn eru vistuð í `equipment.db` SQLite gagnagrunn sem er sjálfkrafa búinn til þegar forritið er keyrt í fyrsta skipti.

## AI Notkun

### Verkfæri notuð
- **GitHub Copilot Agent**: Til að búa til kóða, útskýra concepts, og hjálpa með debugging
- **GPT-4**: Til að hjálpa við arkitektúr ákvarðanir og best practices

### Notkun
- Kóðauppástungur fyrir CRUD aðgerðir á SQLite
- Útskýringar á Rust ownership, lifetimes og Result villumeðhöndlun
- Hjálp við egui GUI components
- Skjölun og comments
- Regex patterns fyrir staðsetningar validation

### Dæmi um beiðnir
- "Búðu til SQLite töflu fyrir equipment með auto-increment ID"
- "Útfærðu Display trait fyrir Table struct með fallegum íslenskum texta"
- "Hvernig get ég raðað Vec<Equipment> eftir mismunandi eiginleikum?"
- "Búðu til egui dropdown fyrir Building enum með íslenskum nöfnum"

### Staðfesting
- Keyrðum `cargo check` og `cargo build` eftir allar breytingar
- Prófuðum alla virkni handvirkt
- Fórum yfir kóða til að tryggja að hann fylgi Rust best practices

### Takmörk
- Allar AI uppástungur voru yfirfarnar og prófaðar handvirkt
- Engar breytingar voru samþykktar án þess að skilja hvað þær gera
- Kóðinn var endurskrifaður þar sem AI gerði villur eða bjó til óhagkvæman kóða

## Höfundur

Daníel Snær Rodríguez.

Verkefni unnið fyrir Tækniskólann í Rust forritunarverkefni.
