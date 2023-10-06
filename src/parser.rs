use nom::{character::complete::alphanumeric1, combinator::map_res, IResult};
use nom::{
    character::complete::{char, digit1, one_of, space1},
    sequence::separated_pair,
};

use crate::components::capacitor::Capacitor;
use crate::components::resistor::Resistor;
pub struct SpiceParser {}

pub fn parse_component_name(input: &str) -> IResult<&str, String> {
    map_res(alphanumeric1, |s: &str| {
        Ok::<String, nom::error::Error<&str>>(s.to_string())
    })(input)
}

pub fn parse_capacitor(input: &str) -> IResult<&str, Capacitor> {
    let (input, _) = char('C')(input)?;

    // Parse a sequence of digits (the resistor identification) and convert it to a String.
    let (input, identification) = parse_component_name(input)?;

    // Match and consume one or more whitespace characters.
    let (input, _) = space1(input)?;

    // Parse two sequences of digits (node_1 and node_2) separated by whitespace,
    // and store them as a tuple.
    let (input, (node_1, node_2)) = separated_pair(digit1, space1, digit1)(input)?;

    let node_1 = node_1.to_string();
    let node_2 = node_2.to_string();

    // Match and consume one or more whitespace characters.
    let (input, _) = space1(input)?;

    // Parse a sequence of digits (the resistor value) and convert it to f64.
    let (input, mut value) = map_res(digit1, |s: &str| s.parse::<f64>())(input)?;

    let (input, prefix) =
        one_of::<&str, &str, nom::error::Error<&str>>("kM")(input).unwrap_or((input, ' '));

    value *= match_prefix(prefix);

    let components = Capacitor {
        node_1,
        node_2,
        value,
        identification,
    };

    Ok((input, components))
}

pub fn parse_resistor(input: &str) -> IResult<&str, Resistor> {
    let (input, _) = char('R')(input)?;

    // Parse a sequence of digits (the resistor identification) and convert it to a String.
    let (input, identification) = parse_component_name(input)?;

    // Match and consume one or more whitespace characters.
    let (input, _) = space1(input)?;

    // Parse two sequences of digits (node_1 and node_2) separated by whitespace,
    // and store them as a tuple.
    let (input, (node_1, node_2)) = separated_pair(digit1, space1, digit1)(input)?;

    let node_1 = node_1.to_string();
    let node_2 = node_2.to_string();

    // Match and consume one or more whitespace characters.
    let (input, _) = space1(input)?;

    // Parse a sequence of digits (the resistor value) and convert it to f64.
    let (input, mut value) = map_res(digit1, |s: &str| s.parse::<f64>())(input)?;

    let (input, prefix) =
        one_of::<&str, &str, nom::error::Error<&str>>("kM")(input).unwrap_or((input, ' '));

    value *= match_prefix(prefix);

    let components = Resistor {
        node_1,
        node_2,
        value,
        identification,
    };

    Ok((input, components))
}

pub fn match_prefix(prefix: char) -> f64 {
    dbg!(prefix);
    match prefix {
        'k' => 1000.0,
        'M' => 1_000_000.0,
        ' ' => 1.0,
        _ => todo!(),
    }
}
