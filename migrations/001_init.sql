PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS equipment (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kind TEXT NOT NULL CHECK(kind IN ('Table','Chair','Projector')),
    value_isk INTEGER NOT NULL CHECK(value_isk >= 0),
    house TEXT NOT NULL CHECK(house IN ('HA','H','S')),
    floor INTEGER NOT NULL CHECK(floor >= 0),
    room INTEGER NOT NULL CHECK(room >= 0),
    -- extra fields (only one applies depending on kind)
    seats INTEGER,
    chair_kind TEXT CHECK(chair_kind IN ('Haegindastoll','Skolastoll','Skrifstofustoll','Annad')),
    lumens INTEGER,
    created_at TEXT DEFAULT (datetime('now'))
);

-- Useful view for sorted ordering
CREATE VIEW IF NOT EXISTS equipment_sorted AS
SELECT *
FROM equipment
ORDER BY
    CASE house WHEN 'HA' THEN 1 WHEN 'H' THEN 2 WHEN 'S' THEN 3 ELSE 99 END,
    floor ASC,
    room ASC,
    kind ASC,
    id ASC;