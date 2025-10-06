# Búnaðarlisti Tækniskólans (Lokaverkefni B)

Rust forrit með myndrænu viðmóti (`egui/eframe`) og SQLite (`rusqlite`) til að halda utan um búnað skólans og staðsetningu hans.

## Keyrslu‑leiðbeiningar

### 0) Forsendur
- Nýjasta `rustup` og `cargo` uppsett.
- **Windows/macOS/Linux** styðst.
- Engin þjónustulykil eða viðkvæm gildi eru geymd í repo (sjá `.env.example`).

### 1) Afrita repo og undirbúa .env
```bash
cp .env.example .env
# Breyttu ef þarf: DB_PATH=app_data/inventory.sqlite3
```

### 2) Keyra
```bash
cargo run --release
```
Forritið býr til gagnagrunn ef ekki er til, keyrir flutningsskrár (`migrations/001_init.sql`) og ræsir gluggann.

### 3) Notkun
- **Skrá nýjan hlut**: Veldu tegund (Borð/Stóll/Skjávarpi), settu inn verðmæti og staðsetningu (HA/H/S + hæð + herbergi). Dæmi: `H-202` ⇒ Háteigsvegur, 2. hæð, herbergi 02.
- **Eyða / Uppfæra staðsetningu / Prenta út frá `id`** úr vinstri stikunni.
- **Síur** efst: eftir húsi og/eða tegund.
- **JSON inn/út**: Flytja út/inn gagnaafrit (id eru endurskilgreind við innflutning).

## Hönnun / Uppbygging

```
src/
  app.rs                # egui/eframe viðmót
  db.rs                 # SQLite aðgangur og SELECT/INSERT/UPDATE/DELETE
  inventory.rs          # "Listinn" með öllum aðgerðum (CRUD + JSON)
  models/
    mod.rs
    house.rs            # enum House { HA, H, S } + Display/FromStr
    location.rs         # struct Location + Display/FromStr (H-202/HA-123)
    chair_kind.rs       # enum ChairKind + Display/FromStr/TryFrom
    table_item.rs       # struct TableItem + Display
    chair.rs            # struct Chair + Display
    projector.rs        # struct Projector + Display
    equipment.rs        # enum EquipmentKind + EquipmentRecord + TryFrom
migrations/001_init.sql # gagnagrunnsskjema + röðuð sýn (VIEW)
```

- **Display** og **TryFrom** útfært fyrir allan búnað (sjá `models/` skrárnar).
- Hver **struct** og **enum** er í **sér skrá**.
- **Röðun** í lista: eftir húsi, svo hæð, svo herbergi og tegund (`equipment_sorted` view).
- **JSON**: `export_json()` / `import_json()` í `inventory.rs`.
- **SQLite**: `rusqlite` með `bundled` eiginleika til að tryggja `RETURNING id` stuðning.

## Gæðapunktar
- Villu‑meðhöndlun með `thiserror` (sjá `db::DbError`).
- Staðsetningar‑validator (`Location::from_str`) sem styður kóða á borð við `HA-123` og `H-202`.
- `egui_extras::TableBuilder` fyrir snyrtilegan lista.
- File‑dialog með `rfd` (inn/út JSON).

## Prófanir
Keyra prófanir:
```bash
cargo test
```
(Sjá `tests/` ef bætt er við – lágmarkspróf fyrir staðsetningu mætti setja í `tests/location_tests.rs`).

## GitHub repo + samstarf
1. **Búðu til private repo** á GitHub: *New → Repository → Private* (nafn t.d. `bunadarlisti_tekniskoli`).
2. Ýttu kóðanum (`git init`, `git add .`, `git commit -m "Lokaverkefni B"`, `git branch -M main`, `git remote add origin ...`, `git push -u origin main`).
3. **Bjóða `gestskoli` inn**: *Settings → Collaborators → Add people → `gestskoli`* og velja **Write** aðgang.
4. **Skila slóð á Innu** í skilahólfið „Lokaverkefni B“.

## Skil (Hverju á að skila)
- Allur kóði í **GitHub** (private).
- Þessi `README` og leiðbeiningar til keyrslu.
- **AI‑skjöl**: sjá `AI_USAGE.md` og `ai/commands.md` (með dæmum).
- **Myndband**: sýnið helstu virkni (síun, skráningu, eyðingu, uppfærslu og JSON inn/út). Mælt er með OBS eða skjáupptöku í stýrikerfi.

## Öryggi
- Engin leyndarmál í repo – notið `.env` (sjá `.env.example`).
- SQL injection: `rusqlite` `params![]` er notað fyrir bindingu.
- Villumeðhöndlun og boundary tilfelli: birta skilaboð og rangt inntak fellt út.

## Heimildir og gagnlegar tilvísanir
- `eframe`/`egui` – opinbert GUI (sjá docs.rs)  
- `rusqlite` – SQLite fyrir Rust (sjá docs.rs)  
- `rfd` – skráavalgluggar fyrir egui

## Leyfi
MIT.

### Leitar-/birtingaraðgerðir
- Birta **alla**.
- Birta **eftir húsi**.
- Birta **alla af ákveðinni tegund**.
- Birta **alla í ákveðinni stofu** (hús + hæð + herbergi).
- Birta **alla á ákveðinni hæð í tilteknu húsi**.
