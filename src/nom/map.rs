use crate::repr::map::{Entities, Entity};
use nom::{
    bytes::complete::take_until,
    character::complete::{char as character, multispace0},
    combinator::{map, map_res, rest},
    multi::many0,
    sequence::{delimited, separated_pair},
};

fn property(i: &str) -> nom::IResult<&str, (&str, &str)> {
    separated_pair(
        delimited(character('"'), take_until("\""), character('"')),
        multispace0,
        delimited(character('"'), take_until("\""), character('"')),
    )(i)
}

fn properties(i: &str) -> nom::IResult<&str, Vec<(&str, &str)>> {
    many0(delimited(multispace0, property, multispace0))(i)
}

fn entity(i: &str) -> nom::IResult<&str, Entity> {
    map(
        delimited(
            character('{'),
            delimited(multispace0, properties, multispace0),
            character('}'),
        ),
        |v| {
            v.into_iter()
                .map(|(x, y)| (x.to_string(), y.to_string()))
                .collect()
        },
    )(i)
}

pub fn entities(i: &str) -> nom::IResult<&str, Entities> {
    many0(delimited(multispace0, entity, multispace0))(i)
}

pub fn entities_bin(i: &[u8]) -> nom::IResult<&[u8], Entities> {
    let (_, s) = map_res(rest, std::str::from_utf8)(i)?;
    entities(s)
        .map(|(_, v)| ([].as_slice(), v))
        .map_err(|e| e.map_input(|_| i))
}
