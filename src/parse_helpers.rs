use std::ops::{Add, Mul, Neg};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt};
use nom::IResult;
use nom::sequence::tuple;

pub fn infallible_parse<T>(mut p: impl FnMut(&[u8]) -> T) -> impl FnMut(&[u8]) -> Result<T, !> {
    move |input|Ok(p(input))
}

pub fn parse_signed_nr_bytes<T: Mul<Output=T> + Add<Output=T> + From<u8> + Neg<Output=T> + Copy>(input: &[u8]) -> IResult<&[u8], T> {
    let (rest, (minus, mut value)) = tuple((opt(tag(b"-")), parse_unsigned_nr_bytes::<T>))(input)?;

    if minus.is_some() {
        value = -value;
    }

    Ok((rest, value))
}
pub fn parse_unsigned_nr_bytes<T: Mul<Output=T> + Add<Output=T> + From<u8> + Copy>(input: &[u8]) -> IResult<&[u8], T> {
    map(digit1, move |bytes|{
        let mut acc = <T as From<u8>>::from(0);
        let ten = <T as From<u8>>::from(10);
        for byte in bytes {
            let off = byte - b'0';
            let off = <T as From<u8>>::from(off);
            acc = acc * ten + off
        }
        acc
    })(input)
}