use crate::types::{AttributeRef, Field};
use nom::{
    bytes::complete::{take, take_till}, 
    multi::many0,
    IResult,
};

fn parse_field(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, field) = take_till(|c| c == b' ')(input)?;
    let (input, _) = take(1usize)(input)?;
    Ok((input, field))
}

fn parse_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, value) = take_till(|c| c == b';')(input)?;
    let (input, _) = take(1usize)(input)?;
    let value = &value[1..value.len() - 1]; // strip the quotations
    Ok((input, value))
}

fn tuple(input: &[u8]) -> IResult<&[u8], (Option<Field>, &[u8])> {
    let (input, field) = parse_field(input)?;
    let field = Field::from_bytes(field);
    let (input, value) = parse_value(input)?;
    let (input, _) = take(1usize)(input)?;
    Ok((input, (field, value)))
}

pub fn parse_attributes(input: &[u8]) -> IResult<&[u8], AttributeRef> {
    let mut attributes = AttributeRef::default();
    let (input, field_attributes) = many0(tuple)(input)?;
    field_attributes
        .iter()
        .for_each(|(field, value)| {
            field.map(|f| attributes.update_field(f, value));
        });
    Ok((input, attributes))
}

