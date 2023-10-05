use rust_spice::components::resistor::{parse_resistor, Resistor};

#[test]
fn test_resistor() {
    let mut input = "R1 1 2 100";
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

    input = "RA1 1 2 100";
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

    input = "RA1 1 2 10k";
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

    input = "R1 1 2 10M";
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

    input = "R1 1 2 10M";
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
