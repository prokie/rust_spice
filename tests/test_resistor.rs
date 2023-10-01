use rust_spice::{
    circuit::NodeId,
    components::resistor::{parse_resistor, Resistor},
};

#[test]
fn test_resistor() {
    let input = "R1 1 2 100";
    assert_eq!(
        parse_resistor(input),
        Ok((
            "",
            Resistor {
                node_1: 1 as NodeId,
                node_2: 2 as NodeId,
                value: 100.0,
                identification: "1".to_string(),
            }
        ))
    );
}
