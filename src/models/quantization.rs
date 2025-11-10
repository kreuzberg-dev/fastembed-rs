/// Enum for quantization mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuantizationMode {
    #[default]
    None,
    Static,
    Dynamic,
}
