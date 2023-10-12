use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::analysis::Analysis;
use crate::components::{capacitor::Capacitor, resistor::Resistor, voltage_source::VoltageSource};

use nom::{
    character::complete::{alphanumeric1, char, digit1, one_of, space1},
    combinator::map_res,
    error::ErrorKind,
    sequence::separated_pair,
    IResult,
};
#[derive(Debug, PartialEq)]
pub enum Components {
    VoltageSource(VoltageSource),
    Resistor(Resistor),
    Capacitor(Capacitor),
}

#[derive(Debug, PartialEq)]
pub struct Spice {
    pub title: String,
    pub components: Vec<Components>,
    pub analysis: Vec<Analysis>,
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
pub fn parse_file(input: &str) -> Result<Spice, ErrorKind> {
    let file = File::open(input).unwrap();
    let mut components = Vec::new();
    let mut analysis = Vec::new();
    let mut reader = io::BufReader::new(file).lines();
    let mut inside_control = false;

    let title = match reader.next() {
        Some(Ok(value)) => value,
        Some(Err(_error)) => return Err(ErrorKind::Fail),
        None => return Err(ErrorKind::Fail),
    };

    for line in reader {
        let line = line.unwrap();
        let line = line.trim();

        if line.starts_with(".control") {
            inside_control = true;
            continue;
        } else if line.starts_with(".endc") {
            inside_control = false;
            continue;
        }

        if inside_control {
            if line == "op" {
                analysis.push(Analysis::OperatingPoint);
            } else if line == "print all" {
                continue;
            } else {
                todo!()
            }
        } else if line.starts_with('R') {
            let (_, resistor) = parse_resistor(line).unwrap();
            components.push(Components::Resistor(resistor));
        } else if line.starts_with('C') {
            let (_, capacitor) = parse_capacitor(line).unwrap();
            components.push(Components::Capacitor(capacitor));
        } else if line.starts_with('V') {
            let (_, voltage_source) = parse_voltage_source(line).unwrap();
            components.push(Components::VoltageSource(voltage_source));
        } else if line == ".end" {
            break;
        } else {
            todo!()
        }
    }

    Ok(Spice {
        title,
        components,
        analysis,
    })
}
