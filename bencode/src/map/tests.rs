use super::*;

#[test]
fn map_insert_adds_value() {
    let mut map = InsertOrderMap::new();

    assert_eq!(0, map.len());
    map.insert("test", 5);
    assert_eq!(1, map.len());
}

#[test]
fn map_insert_replaces_value() {
    let mut map = InsertOrderMap::new();

    map.insert("test", 5);
    map.insert("test", 10);
    assert_eq!(1, map.len());
}

#[test]
fn map_get_returns_correct() {
    let mut map = InsertOrderMap::new();

    map.insert("test", 5);
    assert_eq!(Some(&5), map.get(&"test"));

    map.insert("test", 10);
    assert_eq!(Some(&10), map.get(&"test"));
}

#[test]
fn map_index_returns_correct() {
    let mut map = InsertOrderMap::new();

    map.insert("test", 5);
    assert_eq!(5, map[&"test"]);
}

#[should_panic]
#[test]
fn map_index_panics_on_missing() {
    let map: InsertOrderMap<&str, _> = InsertOrderMap::new();
    map[&"test"]
}
