use std::collections::HashMap;

fn rename_object_key<'a>(obj: &'a HashMap<&'a str, &'a str>, new_keys: &'a HashMap<&'a str, &'a str>) -> HashMap<&'a str, &'a str> {
    let mut key_values = Vec::new();

    for (key, value) in obj {
        let new_key = match new_keys.get(key) {
            Some(new_key) => new_key,
            None => key,
        };
        let new_pair = (new_key, value);
        key_values.push(new_pair);
    }

    key_values.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename_object_key() {
        let obj = [("key1", "value1"), ("key2", "value2")]
            .iter()
            .cloned()
            .collect::<HashMap<&str, &str>>();
        let new_keys = [("key1", "new_key1"), ("key2", "new_key2")]
            .iter()
            .cloned()
            .collect::<HashMap<&str, &str>>();
        let expected_output = [("new_key1", "value1"), ("new_key2", "value2")]
            .iter()
            .cloned()
            .collect::<HashMap<&str, &str>>();

        assert_eq!(rename_object_key(&obj, &new_keys), expected_output);
    }
}
