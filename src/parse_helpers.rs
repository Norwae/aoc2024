use std::ops::Neg;
use std::str::FromStr;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, opt};
use nom::IResult;
use nom::sequence::tuple;

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

pub fn parse_i64(input: &str) -> IResult<&str, i64> {
    parse_signed_nr(input)
}
