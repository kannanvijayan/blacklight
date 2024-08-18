
pub(crate) enum ShaderWorkgroupSizeRepr {
  Linear(u32),
  Rectangular(u32, u32),
  Cubic(u32, u32, u32),
}
