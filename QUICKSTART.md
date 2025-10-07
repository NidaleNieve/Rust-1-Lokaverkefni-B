# Quick Start Guide

## Fyrsta keyrsla

### 1. Staðfesta Rust uppsetningu
```bash
rustc --version
cargo --version
```

Ef ekki uppsett, sækja á: https://rustup.rs/

### 2. Fara í verkefnamöppu
```bash
cd /Users/nidale/Documents/Rust/CLAUDE-Lokaverkefni-B
```

### 3. Keyra forritið
```bash
cargo run --release
```

Fyrsta keyrsla mun:
- Downloada öllum dependencies (~2 mínútur)
- Compile verkefnið (~2 mínútur)
- Búa til `equipment.db` gagnagrunn
- Opna GUI gluggann

### 4. Prufa virkni

**Skrá fyrsta búnaðinn:**
1. Velja "📝 Skrá"
2. Velja "🪑 Borð"
3. Hús: Háteigsvegur (dropdown)
4. Hæð: 2 (slider)
5. Herbergi: 5 (slider)
6. Verðmæti: 25000
7. Fjöldi sæta: 6
8. Smella "✅ Skrá búnað"

**Birta búnaðinn:**
1. Fara í "📋 Birta"
2. Velja "Allur búnaður"
3. Smella "🔍 Birta"
4. Sjá fallega formatted lýsingu!

## Næstu skref

Sjá `VIDEO_GUIDE.md` fyrir fullt yfirlit yfir alla virkni.
Sjá `README.md` fyrir ítarlegar leiðbeiningar.

## Algengar spurningar

**Q: Forritið opnast ekki?**
A: Keyra `cargo build --release` fyrst og sjá hvort eru compilation errors.

**Q: Hvar er gagnagrunnurinn?**
A: `equipment.db` í rót verkefnamöppu (auto-created).

**Q: Hvernig hreinsa gögnin?**
A: Eyða `equipment.db` skrá og keyra aftur.

**Q: JSON skrár?**
A: `equipment.json` er búin til þegar þú velur "💾 Vista í JSON".

## Build fyrir dreifingu

```bash
cargo build --release
```

Executable verður í:
```
target/release/CLAUDE-Lokaverkefni-B
```

## Loka forriti

Loka bara glugganum eða `Ctrl+C` í terminal.

Allir gögn eru vistuð í gagnagrunn sjálfvirkt!
