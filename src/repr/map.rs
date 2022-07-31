use std::collections::HashMap;

pub struct Entity<'a> {
    pub properties: HashMap<&'a str, &'a str>,
}
