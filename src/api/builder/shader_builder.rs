use std::{
  collections::HashSet,
  marker::PhantomData,
};
use crate::{
  api::{
    buffer_disposition::{ BufferRead, BufferReadWrite, BufferWrite },
    builder::CodeBlockBuilder,
    data_type::{
      ArgTupleDataType,
      ArgTupleHandleMap,
      EntryPointArgDataType,
      HostShareableDataType,
      StructDataTypeRepr,
      ProcResultType,
    },
    handle::{ BufferBindingHandle, ExprHandle, FunctionHandle },
    EntryPoint,
    Project,
    Shader,
  },
  buffer_disposition::BufferDisposition,
  model::{
    BufferBindingModel,
    CodeBlockModel,
    DataTypeCollector,
    EntryPointModel,
    ExpressionModel,
    FunctionModel,
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
  functions: Vec<FunctionModel>,
  buffer_bindings: Vec<BufferBindingModel>,
  entrypoints: Vec<EntryPointModel>,
  used_buffer_bindings: HashSet<(u32, u32)>,
  _phantom: PhantomData<& 'sh ()>
}
impl<'sh, 'pr: 'sh> ShaderBuilder<'sh, 'pr> {
  /** Create a new shader builder for the given project builder. */
  pub(crate) fn new(project: &'pr Project) -> Self {
    ShaderBuilder {
      _project: project,
      buffer_bindings: Vec::new(),
      functions: Vec::new(),
      entrypoints: Vec::new(),
      used_buffer_bindings: HashSet::new(),
      _phantom: PhantomData,
    }
  }

  fn build_sub_code_block<B, RET>(&mut self, builder_func: B) -> CodeBlockModel
    where B: FnOnce(&mut CodeBlockBuilder<'sh, 'sh, RET>),
          RET: ProcResultType
  {
    let mut code_block_builder = CodeBlockBuilder::new();
    builder_func(&mut code_block_builder);
    code_block_builder.build()
  }

  /** Define a new shader function. */
  pub fn define_function<ARG, RET, FB>(&mut self,
    func_name: &'static str,
    arg_names: ARG::NameTuple,
    builder_func: FB,
  ) -> FunctionHandle<'sh, ARG, RET>
    where ARG: ArgTupleDataType + ArgTupleHandleMap<'sh>,
          RET: ProcResultType,
          FB: FnOnce(&mut CodeBlockBuilder<'sh, 'sh, RET>, ARG::HandleTuple)
  {
    let arg_handles = ARG::make_handle_tuple(&arg_names);
    let code_block_model = self.build_sub_code_block(move |builder| {
      builder_func(builder, arg_handles);
    });
    let identifier_model = IdentifierModel::new(func_name);

    let function_model = FunctionModel::new(
      identifier_model.clone(),
      ARG::make_names_vector(&arg_names),
      ARG::make_types_vector(),
      RET::proc_result_repr(),
      code_block_model
    );
    self.functions.push(function_model);

    FunctionHandle::new(identifier_model)
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
    let ident_model = IdentifierModel::new("global_id");
    let ident_expr_model = IdentifierExprModel::new(ident_model, ARG::repr());
    let arg_expr =
      ExprHandle::new(Box::new(ExpressionModel::Identifier(ident_expr_model)));
    let code_block_model = self.build_sub_code_block(move |builder| {
      builder_func(builder, arg_expr);
    });
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
      panic!("Buffer binding with group {} and index {} already defined.",
             group, index);
    }
    let dt_repr = DT::repr();
    let identifier_model = IdentifierModel::new(name);
    // Ensure that the 
    let buffer_binding_model = BufferBindingModel::new(
      identifier_model.clone(),
      group,
      index,
      dt_repr,
      DISP::REPR,
    );
    self.buffer_bindings.push(buffer_binding_model);
    self.used_buffer_bindings.insert((group, index));
    BufferBindingHandle::new(identifier_model)
  }

  /** Build the shader from the definitions provided. */
  pub(crate) fn build(self) -> Shader {
    let shader_model =
      ShaderModel::new(
        self.collect_struct_data_types(),
        self.buffer_bindings,
        self.functions,
        self.entrypoints
      );
    Shader::new(shader_model)
  }

  /**
   * Collect a vector of all struct data types used this shader definition.
   * This includes transitively referenced struct data types.
   */
  fn collect_struct_data_types(&self) -> Vec<StructDataTypeRepr> {
    let mut collector = DataTypeCollector::new();
    self.collect_struct_data_types_into(&mut collector);
    collector.take_data_types()
  }

  fn collect_struct_data_types_into(&self, collector: &mut DataTypeCollector) {
    for function in &self.functions {
      function.collect_struct_data_types_into(collector);
    }
    for buffer_binding in &self.buffer_bindings {
      buffer_binding.collect_struct_data_types_into(collector);
    }
  }
}
