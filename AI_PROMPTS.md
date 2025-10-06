# AI Skipanir og Beiðnir (AI Prompts Used)

Þessi skjal inniheldur yfirlit yfir allar AI skipanir og beiðnir sem notaðar voru við gerð þessa verkefnis.

## Verkefnaskipulag og Arkitektúr

### 1. Verkefnisuppbygging
**Skipun:**
```
Create a Rust project structure for an equipment management system with:
- Separate files for each struct and enum
- Location struct with Building enum
- Table, Chair, and Projector structs
- Equipment enum to wrap all types
- Database layer with SQLite
- GUI with eframe/egui
```

**Niðurstaða:** Grunnuppbygging verkefnisins með öllum nauðsynlegum skrám.

### 2. Location og Building
**Skipun:**
```
Create a Location struct with:
- Building enum (Hafnarfjörður=HA, Háteigsvegur=H, Skólavörðuholt=S)
- floor: u8
- room: u8 (max 99)
- Implement Display trait to show format like "H-202"
- Implement TryFrom<&str> with regex validation for format "{BUILDING}-{FLOOR}{ROOM}"
```

**Niðurstaða:** location.rs með fullri validation og Display útfærslu.

## Data Models

### 3. ChairType Enum
**Skipun:**
```
Create ChairType enum with variants:
- Hægindastóll (comfort chair)
- Skólastóll (school chair)  
- Skrifstofustóll (office chair)
- Annað (other)
Implement Display trait in Icelandic and TryFrom<&str>
```

**Niðurstaða:** chair_type.rs með enum og traits.

### 4. Equipment Structs
**Skipun:**
```
Create three equipment structs:
1. Table with: id (Option<i64>), location, value (u32), seats (u8)
2. Chair with: id, location, value, chair_type (ChairType)
3. Projector with: id, location, value, lumens (u32)

Each should:
- Implement Display trait with nice Icelandic descriptions like:
  "Borð með ID: {id}, kostar {value} kr., fyrir {seats} manns og er staðsett í {location}"
- Have constructor methods
- Have with_id() method
```

**Niðurstaða:** table.rs, chair.rs, projector.rs með fallega formatted Display útfærslur.

### 5. Equipment Wrapper
**Skipun:**
```
Create Equipment enum that wraps Table, Chair, Projector
Add methods:
- get_id() -> Option<i64>
- set_id(i64)
- get_type_name() -> &str
Implement Display to delegate to inner type
```

**Niðurstaða:** equipment.rs með wrapper enum.

## Database Layer

### 6. SQLite Integration
**Skipun:**
```
Create database.rs with:
- Database struct wrapping rusqlite Connection
- Create table with columns: id (auto-increment), type, building, floor, room, value, extra_data
- CRUD operations:
  - insert_equipment(&Equipment) -> Result<i64>
  - get_all_equipment() -> Result<Vec<Equipment>>
  - update_location(id, Location) -> Result<()>
  - delete_equipment(id) -> Result<()>
- Query methods:
  - get_equipment_by_building(Building)
  - get_equipment_by_type(&str)
  - get_equipment_by_room(Building, floor, room)
  - get_equipment_by_floor(Building, floor)
All queries should order by: building, floor, room, type
```

**Niðurstaða:** database.rs með fullri CRUD virkni og síunaraðferðum.

## GUI Implementation

### 7. Basic GUI Structure
**Skipun:**
```
Create main.rs with eframe application:
- Window size 1400x900, min 1200x700
- App sections: Register, Update, Delete, Display
- Use tab navigation
- Show success/error messages prominently
```

**Niðurstaða:** Grunnviðmót með navigation.

### 8. Location Input Widget
**Skipun:**
```
Create a reusable location input function that takes:
- ui: &mut egui::Ui
- building: &mut Building
- floor: &mut u8
- room: &mut u8

Should display:
- ComboBox dropdown for building selection with Icelandic names
- Slider for floor (0-9)
- Slider for room (0-99)
- Preview label showing "Staðsetning: {code}"

Make it a static function to avoid borrow checker issues
```

**Niðurstaða:** Endurnýtanlegt location input widget.

### 9. Registration Section
**Skipun:**
```
Create registration UI with:
- Radio buttons for equipment type with emojis (🪑 Borð, 💺 Stóll, 📽️ Skjávarpi)
- Unified location input using the location widget
- Value text input
- Conditional inputs:
  - Table: slider for seats (1-20)
  - Chair: ComboBox for chair type
  - Projector: text input for lumens
- Submit button that validates and inserts into database
- Clear messages for success/errors
```

**Niðurstaða:** Skráningarhluti með validation.

### 10. Update Section
**Skipun:**
```
Create update UI that:
- Takes ID as text input
- Shows same location input widget as registration
- Updates only the location of equipment
- Shows success/error messages
```

**Niðurstaða:** Uppfærsluhluti með sama location input og skráning.

### 11. Display Section with Filters
**Skipun:**
```
Create display section with:
- Radio button filters:
  - "Allur búnaður" (all equipment)
  - "Eftir húsi" (by building) - show building dropdown
  - "Eftir tegund" (by type) - show type radio buttons
  - "Eftir stofu" (by room) - show building dropdown + floor/room sliders
  - "Eftir hæð" (by floor) - show building dropdown + floor slider
- Search text input to filter displayed results
- Button to trigger display
- JSON export/import buttons
```

**Niðurstaða:** Birting með öllum umbeðnum síum.

### 12. Sortable Table
**Skipun:**
```
Add sortable grid display with:
- Clickable column headers: "ID ▲▼", "Tegund ▲▼", "Staðsetning ▲▼", "Verðmæti ▲▼"
- Toggle between ascending/descending on click
- Reset button that appears when sorted
- Show row count
- Use egui::Grid with striped rows
- Display full formatted equipment string in last column
```

**Niðurstaða:** Raðanleg tafla með reset virkni.

### 13. Search Functionality
**Skipun:**
```
Add search that:
- Filters displayed_equipment by search_query
- Searches in the formatted equipment string (case-insensitive)
- Updates in real-time
```

**Niðurstaða:** Leitarvirkni í birtingarhluta.

## WOW Factor Features

### 14. Statistics Panel
**Skipun:**
```
Add statistics display showing:
- Total equipment count
- Total value (sum of all equipment values)
- Count and percentage by type (Table, Chair, Projector)
- Count and percentage by building (HA, H, S)
Display in a nice grid with emojis and formatting
Add toggle button "Sýna/Fela tölfræði"
```

**Niðurstaða:** Tölfræðipanel með ítarlega sundurliðun.

### 15. Enhanced Visual Design
**Skipun:**
```
Improve visual design:
- Add custom spacing and padding to egui style
- Use RichText for messages with larger icons and bold text
- Green checkmark (✓) for success messages
- Red X (✗) for error messages
- Better button styling with emojis
- Group statistics in egui::group
- Add database name in header
```

**Niðurstaða:** Fallegra og notendavænna viðmót.

### 16. JSON Import/Export
**Skipun:**
```
Implement JSON functionality:
- Save all equipment to equipment.json with pretty formatting
- Load equipment from equipment.json
- Reset IDs on import for auto-increment
- Show count of imported items
- Handle errors gracefully
```

**Niðurstaða:** Virk JSON import/export virkni.

## Error Handling og Bug Fixes

### 17. Borrow Checker Issues
**Skipun:**
```
Fix borrow checker errors in render_location_input by:
- Making it a static method (no &mut self)
- Calling with Self::render_location_input()
- Passing only needed parameters
```

**Niðurstaða:** Borrow checker villur lagfærðar.

### 18. MutexGuard Lifetime Issues
**Skipun:**
```
Fix database access issues by:
- Creating scope for MutexGuard with {}
- Storing result before dropping guard
- Then using result after guard is dropped
```

**Niðurstaða:** Lifetime villur í load_equipment() lagfærðar.

### 19. egui API Updates
**Skipun:**
```
Fix ComboBox API:
- Replace from_id_salt() with from_id_source()
- Use string literals instead of ui.next_auto_id()
```

**Niðurstaða:** Samhæfni við nýjustu egui útgáfu.

## Testing og Validation

### 20. Input Validation
**Skipun:**
```
Add validation for:
- Value must be valid u32
- Lumens must be valid u32
- Location format must match regex pattern
- Room number max 99
- Show clear error messages in Icelandic
```

**Niðurstaða:** Öll validation virk.

### 21. Compilation Testing
**Skipun:**
```
cargo check
cargo build --release
```

**Niðurstaða:** Verkefni compiles án villna (1 warning um ónotaða aðferð).

## Documentation

### 22. README Creation
**Skipun:**
```
Create comprehensive README.md with:
- Feature list in Icelandic
- Technical details
- Installation instructions
- Usage guide for each section
- Display/TryFrom examples
- Location format explanation
- Database info
- AI usage documentation
```

**Niðurstaða:** Ítarleg README skjöl.

## Samantekt á AI Notkun

**AI Verkfæri:**
- GitHub Copilot Agent (GPT-4)
- Fyrir: kóðauppástungur, útskýringar, debugging

**Helstu Notkunarsvið:**
1. Rust ownership og borrow checker útskýringar
2. egui widget dæmi og API notkun
3. SQLite query mynstur
4. Regex fyrir validation
5. Serde serialization
6. Error handling patterns
7. Display trait útfærslur
8. GUI layout og hönnun

**Validering:**
- Allar uppástungur prófaðar með `cargo check`
- Handvirk yfirferð á öllum kóða
- Testaður allur functionality í GUI

**Takmörk:**
- Engar blindar samþykktir á AI kóða
- Allur kóðinn skilinn og yfirfarinn
- Endurskrifað þar sem AI gerði villur

## Viðbótar Eiginleikar Útfærðir

Til viðbótar við grunnkröfur:
- ✅ Search functionality með real-time filtering
- ✅ Sortable columns (ascending/descending)
- ✅ Statistics panel með sundurliðun
- ✅ Enhanced visual design með emojis og colors
- ✅ Better error messages í íslensku
- ✅ Unified location input widget
- ✅ JSON import/export
- ✅ Toggle filters í Display section
- ✅ Comprehensive README documentation
