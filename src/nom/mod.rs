pub mod bsp;
pub mod texture;
pub mod wad;

const NAME_LEN: usize = 16;

fn cstr16(i: &[u8]) -> nom::IResult<&[u8], &str> {
    let (i, cstr) = nom::bytes::complete::take(NAME_LEN)(i)?;
    let (_, cstr) = nom::combinator::map_res(
        nom::bytes::complete::take_until("\0"),
        std::str::from_utf8,
    )(cstr)?;
    Ok((i, cstr))
}

trait SliceExt {
    fn off(&self, shift: u32) -> Self;
    fn off_size(&self, shift: u32, size: u32) -> Self;
}

impl<T> SliceExt for &'_ [T] {
    fn off(&self, shift: u32) -> Self {
        self.get(shift as usize..).unwrap_or(&[])
    }

    fn off_size(&self, shift: u32, size: u32) -> Self {
        self.get(shift as usize..shift as usize + size as usize)
            .unwrap_or(&[])
    }
}
