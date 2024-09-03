
/**
 * Represents possible block dimensions.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockDims {
  OneD(u32),
  TwoD([u32; 2]),
  ThreeD([u32; 3]),
}
