use rust_spice::{
    analysis::Analysis,
    components::{resistor::Resistor, voltage_source::VoltageSource},
    parser::{parse_file, Components, Spice},
};

#[test]
fn test_parser_voltage_divider() {
    let input = "assets/circuits/voltage_divider.sp";
    assert_eq!(
        parse_file(input).unwrap(),
        Spice {
            title: "voltage divider".to_string(),
            components: vec![
                Components::VoltageSource(VoltageSource {
                    identification: "1".to_string(),
                    node_1: "in".to_string(),
                    node_2: "0".to_string(),
                    value: 1.0
                }),
                Components::Resistor(Resistor {
                    identification: "1".to_string(),
                    node_1: "in".to_string(),
                    node_2: "out".to_string(),
                    value: 1000.0
                }),
                Components::Resistor(Resistor {
                    identification: "2".to_string(),
                    node_1: "out".to_string(),
                    node_2: "0".to_string(),
                    value: 2000.0
                })
            ],
            analysis: vec![Analysis::OperatingPoint]
        }
    );
}
