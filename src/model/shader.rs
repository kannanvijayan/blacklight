use crate::{
  api::data_type::{ DataTypeRepr,StructDataTypeRepr, StructFieldRepr },
  model::{
    BufferBindingModel,
    EntryPointModel,
    FunctionModel,
    IdentifierModel,
    VariableBindingModel,
  },
};

/**
 * Model of a shader.
 */
#[derive(Clone, Debug)]
pub(crate) struct ShaderModel {
  struct_data_types: Vec<StructDataTypeRepr>,
  uniform_data_type: StructDataTypeRepr,
  buffer_bindings: Vec<BufferBindingModel>,
  const_definitions: Vec<VariableBindingModel>,
  functions: Vec<FunctionModel>,
  entrypoints: Vec<EntryPointModel>,
}
impl ShaderModel {
  /** Create a new shader model. */
  pub(crate) fn new(
    struct_data_types: Vec<StructDataTypeRepr>,
    uniform_data_type: StructDataTypeRepr,
    buffer_bindings: Vec<BufferBindingModel>,
    const_definitions: Vec<VariableBindingModel>,
    functions: Vec<FunctionModel>,
    entrypoints: Vec<EntryPointModel>,
  ) -> Self {
    ShaderModel {
      uniform_data_type,
      struct_data_types,
      buffer_bindings,
      const_definitions,
      functions,
      entrypoints,
    }
  }

  /** Get the struct data types. */
  pub(crate) fn struct_data_types(&self) -> &[StructDataTypeRepr] {
    &self.struct_data_types
  }

  /** Get the buffer bindings. */
  pub(crate) fn buffer_bindings(&self) -> &[BufferBindingModel] {
    &self.buffer_bindings
  }

  /** Get the functions */
  pub(crate) fn functions(&self) -> &[FunctionModel] {
    &self.functions
  }

  /** Get the entrypoints. */
  pub(crate) fn entrypoints(&self) -> &[EntryPointModel] {
    &self.entrypoints
  }

  /** Get the const definitions. */
  pub(crate) fn const_definitions(&self) -> &[VariableBindingModel] {
    &self.const_definitions
  }

  /** Generate the additional buffer-length fields to add to the uniforms. */
  pub(crate) fn get_length_fields(&self) -> impl Iterator<Item = &IdentifierModel> {
    self.buffer_bindings.iter().map(|binding| binding.name())
  }

  /** Generate the full uniforms struct definition. */
  pub(crate) fn full_uniform_structs(&self) -> Vec<StructDataTypeRepr> {
    let mut result = Vec::new();

    let length_fields = self.get_length_fields().collect::<Vec<_>>();
    let maybe_lengths_struct = if length_fields.len() > 0 {
      let lengths_struct = StructDataTypeRepr::new(
        IdentifierModel::new("BlacklightBufferLengths"),
        length_fields
          .iter()
          .map(|name| StructFieldRepr::new(
            (*name).clone(),
            DataTypeRepr::new_u32()
          ))
          .collect(),
      );
      result.push(lengths_struct.clone());
      Some(lengths_struct)
    } else {
      None
    };

    let mut fields = vec![
      StructFieldRepr::new(
        IdentifierModel::new("uniforms"),
        DataTypeRepr::Struct(self.uniform_data_type.clone()),
      )
    ];
    if let Some(lengths_struct) = maybe_lengths_struct {
      fields.push(StructFieldRepr::new(
        IdentifierModel::new("lengths"),
        DataTypeRepr::Struct(lengths_struct)
      ));
    }

    let full_uniforms_struct =
      StructDataTypeRepr::new(IdentifierModel::new("BlacklightUniforms"), fields);
    result.push(full_uniforms_struct);
    
    result
  }
}
