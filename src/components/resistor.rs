use nom::{
    character::complete::{alphanumeric1, char, digit1, space1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

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
    let (input, identification) = map_res(alphanumeric1, |s: &str| {
        Ok::<String, nom::error::Error<&str>>(s.to_string())
    })(input)?;

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
    let (input, value) = map_res(digit1, |s: &str| s.parse::<f64>())(input)?;

    let components = Resistor {
        node_1,
        node_2,
        value,
        identification,
    };

    Ok((input, components))
}
