use std::io::{self, Read};

use crate::map::{Entities, Entity};

pub fn map<R: Read>(mut reader: R, size_hint: usize) -> io::Result<Entities> {
    let mut buf = vec![0; size_hint];
    reader.read_exact(&mut buf)?;

    let mut entities: Vec<Entity> = Vec::new();
    for line in buf.split(|x| *x == b'\n') {
        match line {
            b"{" => entities.push(Entity::default()),
            b"}" => {}
            line => {
                let mut kv = line.split(|x| *x == b'"');
                if let Some(key) = kv.nth(1)
                    && let Some(value) = kv.nth(1)
                    && let Some(entity) = entities.last_mut()
                {
                    // TODO : ByteString
                    let key = String::from_utf8_lossy(key);
                    let value = String::from_utf8_lossy(value);
                    entity.insert(key.into_owned(), value.into_owned());
                }
            }
        }
    }

    Ok(entities)
}
