use crate::shader_repr::{
  binding::ShaderBindingRepr,
  constant_definition::ShaderConstantDefinitionRepr,
  entry_point::ShaderEntryPointRepr,
  function::ShaderFunctionRepr,
  structure::ShaderStructureRepr,
  type_definition::ShaderTypeDefinitionRepr,
};

/**
 * Representation of a shader module.
 */
pub(crate) struct ShaderModuleRepr {
  // The types to define.
  pub type_definitions: Vec<ShaderTypeDefinitionRepr>,

  // The uniforms type to use.
  pub uniforms: ShaderStructureRepr,

  // The buffers to bind.
  pub bindings: Vec<ShaderBindingRepr>,

  // The constants to define.
  pub constants: Vec<ShaderConstantDefinitionRepr>,

  // Function definitions.
  pub functions: Vec<ShaderFunctionRepr>,

  // Entry points.
  pub entry_points: Vec<ShaderEntryPointRepr>,
}
