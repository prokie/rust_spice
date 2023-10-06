#[derive(Debug, PartialEq)]
pub struct Capacitor {
    /// The identification of the Capacitor.
    pub identification: String,
    /// The first terminal.
    pub node_1: String,
    /// The second terminal.
    pub node_2: String,
    /// The capacitance in farads.
    pub value: f64,
}
