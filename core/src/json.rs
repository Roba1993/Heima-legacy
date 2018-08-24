use serde_json::{Value, Map};

pub trait SimpleJson {    
    fn get_str<'a, 'b, P: Into<&'b str>>(&'a self, pointer: P) -> &'a str;

    fn set_str<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V);
}

impl SimpleJson for Value {
    fn get_str<'a, 'b, P: Into<&'b str>>(&'a self, pointer: P) -> &'a str {
        self.pointer(pointer.into()).and_then(|v| v.as_str()).unwrap_or("")
    }

    fn set_str<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.as_object_mut().unwrap_or(&mut Map::new()).insert(key.into(), Value::String(value.into()));
    }
}