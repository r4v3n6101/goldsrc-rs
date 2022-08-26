use crate::repr::texture::{Palette, Rgb};
use nom::error::{Error as NomErr, ErrorKind as NomErrKind};
use std::mem::MaybeUninit;

pub mod bsp;
pub mod map;
pub mod texture;
pub mod wad;

const NAME_LEN: usize = 16;
const PALETTE_SIZE: usize = 256;

fn cstr16(i: &[u8]) -> nom::IResult<&[u8], &str> {
    let (i, cstr) = nom::bytes::complete::take(NAME_LEN)(i)?;
    let (_, cstr) = nom::combinator::map_res(
        nom::bytes::complete::take_until("\0"),
        std::str::from_utf8,
    )(cstr)?;
    Ok((i, cstr))
}

fn palette(i: &[u8]) -> nom::IResult<&[u8], Box<Palette>> {
    let (i, palette) = nom::bytes::complete::take(PALETTE_SIZE * 3)(i)?;

    let mut boxed_palette = Box::<Palette>::new_zeroed_slice(PALETTE_SIZE);
    unsafe {
        let ptr = palette.as_ptr() as *const MaybeUninit<Rgb>;
        boxed_palette
            .as_mut_ptr()
            .copy_from_nonoverlapping(ptr, PALETTE_SIZE);
    }

    Ok((i, unsafe { boxed_palette.assume_init() }))
}

#[inline]
fn nom_eof() -> nom::Err<NomErr<&'static [u8]>> {
    nom::Err::Error(NomErr::new([].as_slice(), NomErrKind::Eof))
}

trait SliceExt<'a, T>: Sized {
    fn off(self, shift: usize, size: usize) -> Result<Self, nom::Err<NomErr<&'static [u8]>>>;
    fn off_all(self, shift: usize) -> Result<Self, nom::Err<NomErr<&'static [u8]>>>;
}

impl<'a, T> SliceExt<'a, T> for &'a [T] {
    fn off(self, shift: usize, size: usize) -> Result<Self, nom::Err<NomErr<&'static [u8]>>> {
        self.get(shift..shift + size).ok_or_else(nom_eof)
    }

    fn off_all(self, shift: usize) -> Result<Self, nom::Err<NomErr<&'static [u8]>>> {
        self.get(shift..).ok_or_else(nom_eof)
    }
}
