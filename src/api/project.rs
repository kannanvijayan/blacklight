use wgpu;
use crate::api::{
  Shader,
  builder::ShaderBuilder,
  data_type::StructMappedDataType,
};

/**
 * A project represents a collection of type definitions, constants,
 * and functions that can be used to specify and execute shader modules.
 */
pub struct Project {
  _device: wgpu::Device,
  _queue: wgpu::Queue,
}
impl Project {
  /** Create a new project with the given device and queue. */
  pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
    Project { _device: device, _queue: queue }
  }

  /** Define a new shader module within this project. */
  pub fn define_shader<'pr, UDT, DFN>(&'pr self, definer_fn: DFN)
    -> Shader<UDT>
  where DFN: for <'sh> FnOnce(&mut ShaderBuilder<'sh, 'pr, UDT>),
        UDT: StructMappedDataType
  {
    let mut shader_builder = ShaderBuilder::<UDT>::new(self);
    definer_fn(&mut shader_builder);
    shader_builder.build()
  }
}
