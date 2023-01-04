use std::collections::HashMap;

pub fn rename_key<K, V>(map: &mut HashMap<K, V>, old_key: K, new_key: K)
where
    K: std::cmp::Eq + std::hash::Hash,
{
    if map.contains_key(&old_key) {
        let value = map.remove(&old_key).unwrap();
        map.insert(new_key, value);
    }
}

#[test]
fn test_rename_key() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    rename_key(&mut map, "a", "A");

    assert_eq!(map.get("a"), None);
    assert_eq!(map.get("A"), Some(&1));
}