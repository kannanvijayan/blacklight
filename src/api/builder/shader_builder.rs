use std::{
  collections::{ HashMap, HashSet },
  marker::PhantomData,
};
use crate::{
  api::{
    buffer_disposition::{ BufferRead, BufferReadWrite, BufferWrite },
    builder::CodeBlockBuilder,
    data_type::{
      DataTypeRepr,
      StructDataTypeRepr,
      HostShareableDataType,
      EntryPointArgDataType,
    },
    handle::{ BufferBindingHandle, ExprHandle },
    EntryPoint,
    Project,
    Shader,
  },
  buffer_disposition::BufferDisposition,
  model::{
    BufferBindingModel,
    EntryPointModel,
    ExpressionModel,
    IdentifierExprModel,
    IdentifierModel,
    ShaderModel,
  }
};

/**
 * A builder helper for defining shaders.
 */
pub struct ShaderBuilder<'sh, 'pr: 'sh> {
  _project: &'pr Project,
  defined_names: HashMap<String, DefinedNameKind>,
  struct_data_types: Vec<StructDataTypeRepr>,
  entrypoints: Vec<EntryPointModel>,
  buffer_bindings: Vec<BufferBindingModel>,
  used_buffer_bindings: HashSet<(u32, u32)>,
  _phantom: PhantomData<& 'sh ()>
}
impl<'sh, 'pr: 'sh> ShaderBuilder<'sh, 'pr> {
  /** Create a new shader builder for the given project builder. */
  pub(crate) fn new(project: &'pr Project) -> Self {
    ShaderBuilder {
      _project: project,
      defined_names: HashMap::new(),
      struct_data_types: Vec::new(),
      entrypoints: Vec::new(),
      buffer_bindings: Vec::new(),
      used_buffer_bindings: HashSet::new(),
      _phantom: PhantomData,
    }
  }

  /** Define a new linear shader entrypoint. */
  pub fn define_entrypoint<ARG, EPB>(&mut self,
    name: &'static str,
    block_dims: ARG,
    builder_func: EPB
  ) -> EntryPoint<ARG>
  where
    EPB: FnOnce(&mut CodeBlockBuilder<'sh, 'sh, ()>, ExprHandle<'sh, ARG>),
    ARG: EntryPointArgDataType
  {
    let mut code_block_builder = CodeBlockBuilder::new();
    let ident_model = IdentifierModel::new("global_id");
    let ident_expr_model = IdentifierExprModel::new(ident_model, ARG::repr());
    let arg_expr =
      ExprHandle::new(ExpressionModel::Identifier(ident_expr_model));
    builder_func(&mut code_block_builder, arg_expr);
    let code_block_model = code_block_builder.build();
    let entry_point_model =
      EntryPointModel::new(
        name.into(),
        block_dims.to_block_dims(),
        code_block_model
      );
    self.entrypoints.push(entry_point_model.clone());
    EntryPoint::new(entry_point_model)
  }

  /** Define a read-only buffer binding. */
  pub fn define_read_buffer_binding<DT>(&mut self,
    name: &str,
    group: u32,
    index: u32
  ) -> BufferBindingHandle<'sh, DT, BufferRead>
    where DT: HostShareableDataType,
  {
    self.define_buffer_binding(name, group, index)
  }

  /** Define a write-only buffer binding. */
  pub fn define_write_buffer_binding<DT>(&mut self,
    name: &str,
    group: u32,
    index: u32
  ) -> BufferBindingHandle<'sh, DT, BufferWrite>
    where DT: HostShareableDataType,
  {
    self.define_buffer_binding(name, group, index)
  }

  /** Define a read-write buffer binding. */
  pub fn define_read_write_buffer_binding<DT>(&mut self,
    name: &str,
    group: u32,
    index: u32
  ) -> BufferBindingHandle<'sh, DT, BufferReadWrite>
    where DT: HostShareableDataType,
  {
    self.define_buffer_binding(name, group, index)
  }

  /** Define a buffer binding. */
  fn define_buffer_binding<DT, DISP>(&mut self,
    name: &str,
    group: u32,
    index: u32
  ) -> BufferBindingHandle<'sh, DT, DISP>
  where DT: HostShareableDataType,
        DISP: BufferDisposition
  {
    if self.used_buffer_bindings.contains(&(group, index)) {
      panic!("Buffer binding with group {} and index {} already defined.", group, index);
    }
    let dt_repr = DT::repr();
    self.ensure_data_type(&dt_repr);
    // Ensure that the 
    let buffer_binding_model = BufferBindingModel::new(
      name.to_string(),
      group,
      index,
      dt_repr,
      DISP::REPR,
    );
    self.buffer_bindings.push(buffer_binding_model);
    self.used_buffer_bindings.insert((group, index));
    BufferBindingHandle::new(name.to_string())
  }

  /** Ensure that a data type is added to this shader's definition, if needed */
  fn ensure_data_type(&mut self, data_type: &DataTypeRepr) {
    match data_type {
      DataTypeRepr::Struct(struct_data_type) => {
        self.ensure_struct_data_type(struct_data_type);
      },
      _ => {}
    }
  }

  /** Ensure that a struct data type is added to this shader's definition. */
  fn ensure_struct_data_type(&mut self, struct_data_type: &StructDataTypeRepr) {
    let exists = self.ensure_name(
      struct_data_type.name(),
      DefinedNameKind::StructDataType
    );
    if !exists {
      // Add any of the struct's embedded struct types to the shader first.
      for field in struct_data_type.fields() {
        self.ensure_data_type(field.data_type());
      }
      self.struct_data_types.push(struct_data_type.clone());
    }
  }

  /** Ensure that a name is not already defined. */
  fn ensure_name(&mut self, name: &str, kind: DefinedNameKind) -> bool {
    let entry = self.defined_names.get(name);
    if let Some(entry_kind) = entry {
      if entry_kind != &kind {
        panic!("Name '{}' is already defined as a different kind.", name);
      }
      true
    } else {
      self.defined_names.insert(name.to_string(), kind);
      false
    }
  }

  /** Build the shader from the definitions provided. */
  pub(crate) fn build(self) -> Shader {
    let shader_model =
      ShaderModel::new(
        self.struct_data_types,
        self.buffer_bindings,
        self.entrypoints
      );
    Shader::new(shader_model)
  }
}


// Defined name kinds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DefinedNameKind {
  BufferBinding,
  StructDataType,
}
