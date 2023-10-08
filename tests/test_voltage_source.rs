use rust_spice::{components::voltage_source::VoltageSource, parser::parse_voltage_source};

#[test]
fn test_voltage_source_a() {
    let input = "V1 1 2 100";
    assert_eq!(
        parse_voltage_source(input),
        Ok((
            "",
            VoltageSource {
                node_1: "1".to_string(),
                node_2: "2".to_string(),
                value: 100.0,
                identification: "1".to_string(),
            }
        ))
    );
}
