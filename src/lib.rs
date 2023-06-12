#![allow(dead_code)]

use nom::IResult;
#[derive(Debug, PartialEq)]
pub enum ComponentType {
    VoltageSource,
    CurrentSource,
    Resistor,
    Capacitor,
    Inductor,
    Diode,
}

#[derive(Debug, PartialEq)]
pub struct Component {
    pub node_1: String,
    pub node_2: String,
    pub parameters: String,
    pub _type: ComponentType,
    pub name: String,
}

fn components(input: &str) -> IResult<&str, Component> {
    let components = Component {
        _type: ComponentType::VoltageSource,
        node_1: "in".to_string(),
        node_2: "0".to_string(),
        parameters: "1".to_string(),
        name: "1".to_string(),
    };

    Ok(("", components))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_components() {
        assert_eq!(
            components("V1 in 0 1"),
            Ok((
                "",
                Component {
                    _type: ComponentType::VoltageSource,
                    node_1: "in".to_string(),
                    node_2: "0".to_string(),
                    parameters: "1".to_string(),
                    name: "1".to_string(),
                }
            ))
        );
    }
}
