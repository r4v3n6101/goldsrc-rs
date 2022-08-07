use nom::{
    bytes::complete::take_until,
    character::complete::{char as character, multispace0},
    sequence::{delimited, separated_pair},
};

use crate::repr::map::{Entities, Entity};

fn property(i: &str) -> nom::IResult<&str, (&str, &str)> {
    separated_pair(
        delimited(character('"'), take_until("\""), character('"')),
        multispace0,
        delimited(character('"'), take_until("\""), character('"')),
    )(i)
}

pub fn tmp_entities(i: &[u8]) -> nom::IResult<&[u8], Entities> {
    Ok((&[], Entities::new()))
}

pub fn entities(i: &str) -> nom::IResult<&str, Entities> {
    Ok(("", Entities::new()))
}
