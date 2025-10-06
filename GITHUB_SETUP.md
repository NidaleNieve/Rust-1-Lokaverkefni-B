# GitHub Setup Leiðbeiningar

## 1. Búa til Private Repository

### Á GitHub:
1. Fara á https://github.com/new
2. Repository name: `CLAUDE-Lokaverkefni-B` (eða eitthvað lýsandi)
3. Description: "Búnaðarlisti Tækniskólans - Equipment Management System í Rust"
4. Velja **Private** ✓
5. EKKI bæta við README, .gitignore eða license (við höfum þau nú þegar)
6. Smella "Create repository"

### Bæta við Samstarfsaðila:
1. Fara í Settings > Collaborators
2. Smella "Add people"
3. Leita að `gestskoli` (eða rétt notandanafn kennara)
4. Senda boð

## 2. Push Existing Code

### Í terminal:

```bash
# Fara í verkefnamöppuna
cd /Users/nidale/Documents/Rust/CLAUDE-Lokaverkefni-B

# Byrja Git repo (ef ekki þegar gert)
git init

# Bæta við öllum skrám
git add .

# Fyrsta commit
git commit -m "Initial commit: Búnaðarlisti Tækniskólans með fullri virkni

Features:
- SQLite database með CRUD operations
- eframe/egui GUI með dropdown og sliders
- Formatted display output með íslenskum textum
- Sortable columns með ascending/descending
- 5 filter options í einum hluta
- Statistics panel með live data
- Real-time search functionality
- JSON import/export
- Professional error handling
- All requirements met + WOW factor"

# Tengja við GitHub repo (skipta út USERNAME og REPO)
git remote add origin https://github.com/USERNAME/REPO.git

# Rename branch til main (ef það er master)
git branch -M main

# Push til GitHub
git push -u origin main
```

## 3. Staðfesta á GitHub

Fara á repo síðuna og athuga:
- ✅ Allar skrár eru þar
- ✅ README.md sýnist rétt
- ✅ Collaborator (gestskoli) hefur aðgang

## 4. Skila á Innu

1. Copy GitHub repo URL: `https://github.com/USERNAME/REPO`
2. Fara á Innu
3. Finna "Lokaverkefni B" skilahólf
4. Líma slóðina
5. Skila

## 5. Skrár sem eru á GitHub

### Source Code:
- `src/main.rs` - GUI application
- `src/equipment.rs` - Equipment wrapper
- `src/location.rs` - Location og Building
- `src/chair_type.rs` - ChairType enum
- `src/table.rs` - Table struct
- `src/chair.rs` - Chair struct
- `src/projector.rs` - Projector struct
- `src/database.rs` - SQLite layer

### Configuration:
- `Cargo.toml` - Dependencies
- `Cargo.lock` - Dependency lock
- `.gitignore` - Git ignore rules

### Documentation:
- `README.md` - Aðal leiðbeiningar
- `PROJECT_SUMMARY.md` - Verkefnayfirlit
- `AI_PROMPTS.md` - Allar AI skipanir
- `VIDEO_GUIDE.md` - Myndbandsupptöku leiðbeiningar
- `QUICKSTART.md` - Fljótlegar leiðbeiningar
- `GITHUB_SETUP.md` - Þessi skjal

### Ekki á GitHub (.gitignore):
- `target/` - Build outputs
- `equipment.db` - Database (local only)
- `equipment.json` - Exports (local only)

## 6. Næstu Skref

1. ✅ Push á GitHub
2. ✅ Bæta við samstarfsaðila
3. ✅ Skila á Innu
4. 📹 Taka upp myndband (sjá VIDEO_GUIDE.md)
5. 📤 Skila myndbandi á Innu

## Athugasemdir

**Repository ætti að innihalda:**
- Allur source code
- All documentation
- Working Cargo.toml
- .gitignore

**Repository ætti EKKI að innihalda:**
- Compiled binaries (target/)
- Database files
- IDE settings
- OS-specific files

## Hjálp

**Ef git er ekki uppsett:**
```bash
# macOS
xcode-select --install

# Eða með Homebrew
brew install git
```

**Ef git remote er vitlaust:**
```bash
git remote remove origin
git remote add origin https://github.com/USERNAME/REPO.git
```

**Ef þú þarft að update repo:**
```bash
git add .
git commit -m "Update: [lýsing]"
git push
```
