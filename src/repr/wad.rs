pub type Archive<'file> = Vec<Entry<'file>>;

pub struct Entry<'file> {
    pub full_size: u32,
    pub ty: u8,
    pub compressed: bool,

    pub name: &'file str,
    pub data: &'file [u8],
}
