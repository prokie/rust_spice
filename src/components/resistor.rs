/// The Resistor Component.
/// <div style="text-align: center;">
///     <img src="C:/Users/pontu/Documents/rust_spice/assets/components/resistor.png" alt="Image Alt Text" style="width: 50%; height: auto;">
/// </div>
#[derive(Debug, PartialEq)]
pub struct Resistor {
    /// The identification of the Resistor.
    pub identification: String,
    /// The first terminal.
    pub node_1: String,
    /// The second terminal.
    pub node_2: String,
    /// The resistance in ohms.
    pub value: f64,
}
