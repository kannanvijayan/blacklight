use crate::api::data_type::{
  DataTypeRepr,
  ExprDataType,
  Struct,
  StructMappedDataType,
};

/**
 * Host-shareable data types.
 * See https://www.w3.org/TR/WGSL/#host-shareable-types.
 * | Host-shareable types are used to describe the contents of buffers which are
 * | shared between the host and the GPU, or copied between host and GPU without
 * | format translation. When used for this purpose, the type may additionally
 * | have layout attributes applied as described in § 13.4 Memory Layout. As
 * | described in § 7.3 var Declarations, the store type of uniform buffer and
 * | storage buffer variables must be host-shareable.
 */
pub trait HostShareableDataType: ExprDataType {}

impl HostShareableDataType for i32 {}
impl HostShareableDataType for [i32; 2] {}
impl HostShareableDataType for [i32; 3] {}
impl HostShareableDataType for [i32; 4] {}

impl HostShareableDataType for u32 {}
impl HostShareableDataType for [u32; 2] {}
impl HostShareableDataType for [u32; 3] {}
impl HostShareableDataType for [u32; 4] {}

impl HostShareableDataType for f32 {}
impl HostShareableDataType for [f32; 2] {}
impl HostShareableDataType for [f32; 3] {}
impl HostShareableDataType for [f32; 4] {}

impl<T> HostShareableDataType for Struct<T>
  where T: Copy + StructMappedDataType
{}

/**
 * A type erasure from static and incorporation into runtime for a literal data type.
 */
#[derive(Clone, Copy, Debug)]
pub enum BufferDataValue {
  I32(i32),
  Vec2I32([i32; 2]),
  Vec3I32([i32; 3]),
  Vec4I32([i32; 4]),
  U32(u32),
  Vec2U32([u32; 2]),
  Vec3U32([u32; 3]),
  Vec4U32([u32; 4]),
  F32(f32),
  Vec2F32([f32; 2]),
  Vec3F32([f32; 3]),
  Vec4F32([f32; 4]),
}
