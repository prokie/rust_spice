use nom::{
    character::complete::{char, digit1, one_of, space1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

use crate::parser::parse_component_name;
/// The Resistor Component.
/// <div style="text-align: center;">
///     <img src="C:/Users/pontu/Documents/rust_spice/assets/components/resistor.png" alt="Image Alt Text" style="width: 50%; height: auto;">
/// </div>
#[derive(Debug, PartialEq)]
pub struct Resistor {
    /// The identification of the Resistor.
    pub identification: String,
    /// The first terminal.
    pub node_1: String,
    /// The second terminal.
    pub node_2: String,
    /// The resistance in ohms.
    pub value: f64,
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
