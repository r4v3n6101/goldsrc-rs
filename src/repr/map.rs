use std::collections::HashMap;

/// A single entity definition from a BSP file.
///
/// NB: Keys and values are currently stored as `String`, but future
/// versions may switch to a byte string type since BSP entity
/// data doesn’t guarantee valid UTF-8.
pub type Entity = HashMap<String, String>;

/// All entities extracted from a BSP file, in load order.
pub type Entities = Vec<Entity>;
