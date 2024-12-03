use std::ops::{Add, Mul, Neg};
use std::str::FromStr;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, opt};
use nom::IResult;
use nom::sequence::tuple;

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
pub fn parse_unsigned_nr<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse::<T>)(input)
}

pub fn parse_signed_nr<T: FromStr + Neg<Output=T>>(input: &str) -> IResult<&str, T> {
    map(tuple((
        opt(tag("-")),
        parse_unsigned_nr::<T>
    )), |(neg, v)| {
        let neg: Option<&str> = neg;
        if neg.is_some() {
            -v
        } else {
            v
        }
    })(input)
}
