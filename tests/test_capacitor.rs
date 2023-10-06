use rust_spice::{components::capacitor::Capacitor, parser::parse_capacitor};

#[test]
fn test_capacitor_a() {
    let input = "C1 1 2 100";
    assert_eq!(
        parse_capacitor(input),
        Ok((
            "",
            Capacitor {
                node_1: "1".to_string(),
                node_2: "2".to_string(),
                value: 100.0,
                identification: "1".to_string(),
            }
        ))
    );
}
