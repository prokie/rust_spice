use rust_spice::{components::resistor::Resistor, parser::parse_resistor};

#[test]
fn test_resistor_a() {
    let input = "R1 1 2 100";
    assert_eq!(
        parse_resistor(input),
        Ok((
            "",
            Resistor {
                node_1: "1".to_string(),
                node_2: "2".to_string(),
                value: 100.0,
                identification: "1".to_string(),
            }
        ))
    );
}
#[test]
fn test_resistor_b() {
    let input = "RA1 1 2 100";
    assert_eq!(
        parse_resistor(input),
        Ok((
            "",
            Resistor {
                node_1: "1".to_string(),
                node_2: "2".to_string(),
                value: 100.0,
                identification: "A1".to_string(),
            }
        ))
    );
}
#[test]
fn test_resistor_c() {
    let input = "RA1 1 2 10k";
    assert_eq!(
        parse_resistor(input),
        Ok((
            "",
            Resistor {
                node_1: "1".to_string(),
                node_2: "2".to_string(),
                value: 10000.0,
                identification: "A1".to_string(),
            }
        ))
    );
}
#[test]
fn test_resistor_d() {
    let input = "R1 1 2 10M";
    assert_eq!(
        parse_resistor(input),
        Ok((
            "",
            Resistor {
                node_1: "1".to_string(),
                node_2: "2".to_string(),
                value: 10_000_000.0,
                identification: "1".to_string(),
            }
        ))
    );
}
#[test]
fn test_resistor_e() {
    let input = "R1 1 2 10M";
    assert_eq!(
        parse_resistor(input),
        Ok((
            "",
            Resistor {
                node_1: "1".to_string(),
                node_2: "2".to_string(),
                value: 10_000_000.0,
                identification: "1".to_string(),
            }
        ))
    );
}
