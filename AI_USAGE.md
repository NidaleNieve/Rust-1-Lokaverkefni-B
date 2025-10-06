# AI‑notkunaryfirlýsing

**Verkfæri**: ChatGPT (GPT‑5 Thinking), mögulega GitHub Copilot við smávægilegar lagfæringar í IDE.

**Notkun**: Hugmyndavinna um hönnun lagskiptingar (`models.rs`, `db.rs`, `inventory.rs`, `app.rs`), uppsetning `Cargo.toml` og val á crates (`eframe/egui`, `rusqlite`, `rfd`, `serde`), útfærsla CRUD aðgerða, JSON inn/út, viðmóts‑borð og villumeðhöndlun. Einnig drög að einingaprófum og README/leiðbeiningum.

**Dæmi um skipanir/beiðnir**:
- „Búðu til Rust GUI (egui/eframe) + SQLite (rusqlite) app með CRUD og JSON export/import.“
- „Skrifaðu validator fyrir staðsetningarkóða (HA|H|S)-(floor)(room).“
- „Útfærðu Display og TryFrom fyrir Borð, Stól og Skjávarpa.“
- „Gerðu röðun eftir húsi, hæð, herbergi og tegund.“

**Staðfesting**: Keyra `cargo check/test` eftir breytingar. Yfirfara SQL bindingar og villumeðhöndlun.

**Takmörk**: Útgáfur crates geta breyst; pinnað við útgáfur sem voru stöðugar við gerð verkefnis. Allar breytingar skoðaðar og prófaðar handvirkt áður en samþykkt er.