use bunadarlisti_tekniskoli::models::Location;
#[test]
fn parse_and_format_location() {
    let loc: Location = "H-202".parse().unwrap();
    assert_eq!(loc.floor, 2);
    assert_eq!(loc.room, 2);
    assert_eq!(loc.to_string(), "H-202");
    let loc2: Location = "HA-123".parse().unwrap();
    assert_eq!(loc2.floor, 1);
    assert_eq!(loc2.room, 23);
    assert_eq!(loc2.to_string(), "HA-123");
}