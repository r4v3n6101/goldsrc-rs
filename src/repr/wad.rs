pub struct Entry<'a> {
    pub data: &'a [u8],
    pub full_size: u32,
    pub ty: u8,
    pub compressed: bool,
    pub name: &'a str,
}
