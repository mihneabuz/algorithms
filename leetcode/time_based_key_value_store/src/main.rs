use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map.get(&key)
            .map(|v| {
                match v.binary_search_by_key(&timestamp, |item| item.0) {
                    Ok(index) => v[index].1.clone(),
                    Err(ins) => {
                        if let Some(index) = ins.checked_sub(1) {
                            v[index].1.clone()
                        } else {
                            String::from("")
                        }
                    }
                }
            })
            .unwrap_or_default()
    }
}

fn main() {
    let mut time_map = TimeMap::new();
    let (foo, bar) = ("foo".to_string(), "bar".to_string());

    time_map.set(foo.clone(), bar.clone(), 1);
    assert_eq!(time_map.get(foo.clone(), 1), bar.clone());
    assert_eq!(time_map.get(foo.clone(), 3), bar.clone());
}
