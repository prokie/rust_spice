use rust_spice::parser::{parse_file, Spice};

#[test]
fn test_parser_voltage_divider() {
    let input = "assets/circuits/voltage_divider.sp";
    assert_eq!(
        parse_file(input).unwrap(),
        Spice {
            title: "voltage divider".to_string(),
        }
    );
}
