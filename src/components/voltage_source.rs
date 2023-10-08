#[derive(Debug, PartialEq)]
pub struct VoltageSource {
    /// The identification of the voltage source.
    pub identification: String,
    /// The first terminal.
    pub node_1: String,
    /// The second terminal.
    pub node_2: String,
    /// The voltage in volts.
    pub value: f64,
}
