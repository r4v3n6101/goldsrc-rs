const MIP_LEVELS: usize = 4;

pub struct MipData<'a> {
    pub indices: [&'a [u8]; MIP_LEVELS],
    pub palette: &'a [u8],
}

pub struct MipTexture<'a> {
    pub name: &'a str,
    pub width: u32,
    pub height: u32,
    pub data: Option<MipData<'a>>,
}
