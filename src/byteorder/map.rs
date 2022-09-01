use crate::repr::map::{Entities, Entity};
use std::io::{self, Read};

pub fn map<R: Read>(mut reader: R, size_hint: usize) -> io::Result<Entities> {
    let mut buf = vec![0; size_hint];
    reader.read_exact(&mut buf)?;

    let s = String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut entities: Vec<Entity> = Vec::new();
    for line in s.lines() {
        match line {
            "{" => entities.push(Default::default()),
            "}" => {}
            line => {
                let mut kv = line.split("\"");
                if let Some(key) = kv.nth(1) {
                    if let Some(value) = kv.nth(1) {
                        if let Some(entity) = entities.last_mut() {
                            entity.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(entities)
}
