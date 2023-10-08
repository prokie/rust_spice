use std::{
    fs::File,
    io::{self, BufRead},
};

use nom::{
    character::complete::{alphanumeric1, char, digit1, one_of, space1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

use crate::components::{capacitor::Capacitor, resistor::Resistor, voltage_source::VoltageSource};

#[derive(Debug, PartialEq)]
pub struct Spice {
    pub title: String,
}
/// Parse a component name, which is a sequence of alphanumeric characters.
pub fn parse_component_name(input: &str) -> IResult<&str, String> {
    map_res(alphanumeric1, |s: &str| {
        Ok::<String, nom::error::Error<&str>>(s.to_string())
    })(input)
}

/// Parse a capacitor.
pub fn parse_capacitor(input: &str) -> IResult<&str, Capacitor> {
    let (input, _) = char('C')(input)?;

    // Parse a sequence of digits (the resistor identification) and convert it to a String.
    let (input, identification) = parse_component_name(input)?;

    // Match and consume one or more whitespace characters.
    let (input, _) = space1(input)?;

    // Parse two sequences of digits (node_1 and node_2) separated by whitespace,
    // and store them as a tuple.
    let (input, (node_1, node_2)) = separated_pair(alphanumeric1, space1, alphanumeric1)(input)?;

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
    let (input, (node_1, node_2)) = separated_pair(alphanumeric1, space1, alphanumeric1)(input)?;

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

pub fn parse_voltage_source(input: &str) -> IResult<&str, VoltageSource> {
    let (input, _) = char('V')(input)?;

    // Parse a sequence of digits (the resistor identification) and convert it to a String.
    let (input, identification) = parse_component_name(input)?;

    // Match and consume one or more whitespace characters.
    let (input, _) = space1(input)?;

    // Parse two sequences of digits (node_1 and node_2) separated by whitespace,
    // and store them as a tuple.
    let (input, (node_1, node_2)) = separated_pair(alphanumeric1, space1, alphanumeric1)(input)?;

    let node_1 = node_1.to_string();
    let node_2 = node_2.to_string();

    // Match and consume one or more whitespace characters.
    let (input, _) = space1(input)?;

    // Parse a sequence of digits (the resistor value) and convert it to f64.
    let (input, mut value) = map_res(digit1, |s: &str| s.parse::<f64>())(input)?;

    let (input, prefix) =
        one_of::<&str, &str, nom::error::Error<&str>>("kM")(input).unwrap_or((input, ' '));

    value *= match_prefix(prefix);

    let component = VoltageSource {
        node_1,
        node_2,
        value,
        identification,
    };

    Ok((input, component))
}

pub fn match_prefix(prefix: char) -> f64 {
    match prefix {
        'f' => 0.000_000_000_001,
        'u' => 0.000_001, // 'u' => 'µ
        'm' => 0.001,
        'k' => 1000.0,
        'M' => 1_000_000.0,
        'G' => 1_000_000_000.0,
        ' ' => 1.0,
        _ => todo!(),
    }
}

// Read a spice file line by line and parse each line.
pub fn parse_file(input: &str) -> Result<Spice, io::Error> {
    // Open the file
    let file = File::open(input)?;

    // Create a BufReader to efficiently read the file line by line
    let mut reader = io::BufReader::new(file).lines();

    // let mut title = String::new();
    let mut inside_control = false;

    let title = match reader.next() {
        Some(Ok(value)) => value,
        Some(Err(_error)) => return Err(io::Error::new(io::ErrorKind::Other, "Read error")),
        None => return Err(io::Error::new(io::ErrorKind::Other, "Empty file")),
    };

    // Iterate over lines in the file
    for line in reader {
        let line = line.unwrap();

        if line.starts_with(".control") {
            inside_control = true;
            continue;
        } else if line.starts_with(".endc") {
            inside_control = false;
            continue;
        } else if inside_control {
            continue;
        }

        if line.starts_with('R') {
            let (_, resistor) = parse_resistor(&line).unwrap();
            println!("{:?}", resistor);
        } else if line.starts_with('C') {
            let (_, capacitor) = parse_capacitor(&line).unwrap();
            println!("{:?}", capacitor);
        } else if line.starts_with('V') {
            let (_, voltage_source) = parse_voltage_source(&line).unwrap();
            println!("{:?}", voltage_source);
        } else {
            println!("{:?}", &line);
            todo!()
        }
    }
    Ok(Spice { title })
}
