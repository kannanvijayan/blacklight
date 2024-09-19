use std::marker::PhantomData;
use crate::{
  api::{
    buffer_attributes::{ BufferDisposition, BufferReadWrite },
    data_type::HostShareableDataType,
    handle::{ ExprHandle, LvalueHandle },
  },
  model::{ BufferReadExprModel, ExpressionModel, IdentifierModel, LvalueModel },
};

/**
 * A handle to a buffer binding declared within a shader module.
 */
#[derive(Clone, Debug)]
pub struct BufferBindingHandle<'sh, DT, DISP>
  where DT: HostShareableDataType, DISP: BufferDisposition
{
  name: IdentifierModel,
  _phantom: PhantomData<&'sh (DT, DISP)>,
}
impl<'sh, DT, DISP> BufferBindingHandle<'sh, DT, DISP>
  where DT: HostShareableDataType, DISP: BufferDisposition
{
  /** Create a new buffer binding handle. */
  pub(crate) fn new(name: IdentifierModel) -> Self {
    BufferBindingHandle { name, _phantom: PhantomData }
  }

  /**
   * Create an lvalue ExprHandle for writing the buffer element at an index.
   *
   * The buffer may be referenced in a sub-lifetime, but the expression must
   * come from the same lifetime as the lvalue being produced.
   */
  pub fn read<'cb>(&self, index: ExprHandle<'cb, u32>)
    -> ExprHandle<'cb, DT>
  where DT: HostShareableDataType,
        'sh: 'cb,
  {
    let buffer_read_expr_model =
      BufferReadExprModel::new(self.name.clone(), index.model, DT::repr());
    let expression_model = ExpressionModel::BufferRead(buffer_read_expr_model);
    ExprHandle::new(Box::new(expression_model))
  }
}

impl<'sh, DT> BufferBindingHandle<'sh, DT, BufferReadWrite>
  where DT: HostShareableDataType
{
  /**
   * Create an lvalue ExprHandle for writing the buffer element at an index.
   *
   * The buffer may be referenced in a sub-lifetime, but the expression must
   * come from the same lifetime as the lvalue being produced.
   */
  pub fn elem<'cb>(&self, index: ExprHandle<'cb, u32>)
    -> LvalueHandle<'cb, DT>
  where DT: HostShareableDataType,
        'sh: 'cb,
  {
    let lvalue_model =
      LvalueModel::new_buffer_element(
        self.name.clone(),
        index.model,
        DT::repr(),
      );
    LvalueHandle::new(lvalue_model)
  }
}
