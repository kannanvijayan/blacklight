use wgpu;
use crate::api::{
  Function,
  Shader,
  data_type::{ShArgTupleDataType, ProcResultType},
  builder::ShaderBuilder,
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
  pub fn define_shader<'pr, DFN>(&'pr self, definer_fn: DFN)
    -> Shader
  where DFN: for <'sh> FnOnce(&mut ShaderBuilder<'sh, 'pr>)
  {
    let mut shader_builder = ShaderBuilder::new(self);
    definer_fn(&mut shader_builder);
    shader_builder.build()
  }

  /** Define a function within the project. */
  pub fn define_function<ReturnT, ArgsT>(&self) -> Function<ReturnT, ArgsT>
    where ReturnT: ProcResultType,
          ArgsT: ShArgTupleDataType,
  {
    unimplemented!("Project::define_function")
  }
}
