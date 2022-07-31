pub struct MipData<'a> {
    pub mip0: &'a [u8],
    pub mip2: &'a [u8],
    pub mip4: &'a [u8],
    pub mip8: &'a [u8],
    pub palette: &'a [u8],
}

pub struct MipTexture<'a> {
    pub name: &'a str,
    pub width: u32,
    pub height: u32,
    pub data: Option<MipData<'a>>,
}
