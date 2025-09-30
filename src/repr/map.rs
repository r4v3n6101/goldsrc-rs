use std::collections::HashMap;

// TODO : will be ByteString, because have no guaranties about UTF-8 of keys
pub type Entity = HashMap<String, String>;
pub type Entities = Vec<Entity>;
