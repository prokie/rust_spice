pub struct SpiceParser {}
use nom::{character::complete::alphanumeric1, combinator::map_res, IResult};

pub fn parse_component_name(input: &str) -> IResult<&str, String> {
    map_res(alphanumeric1, |s: &str| {
        Ok::<String, nom::error::Error<&str>>(s.to_string())
    })(input)
}
