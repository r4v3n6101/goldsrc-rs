pub struct ColourData<'a, const N: usize> {
    pub indices: [&'a [u8]; N],
    pub palette: &'a [u8],
}

pub struct MipTexture<'a> {
    pub name: &'a str,
    pub width: u32,
    pub height: u32,
    pub data: Option<ColourData<'a, 4>>,
}

pub struct Picture<'a> {
    pub width: u32,
    pub height: u32,
    pub data: ColourData<'a, 1>,
}
