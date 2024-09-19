use std::marker::PhantomData;
use crate::{
  api::{
    data_type::ExprDataType,
    handle::{ ExprHandle, LvalueHandle },
    variable_attributes::{ VariableMutability, VariableReadWrite },
  },
  model::{ ExpressionModel, IdentifierExprModel, IdentifierModel, LvalueModel },
};

/**
 * A handle to a buffer binding declared within a shader module.
 */
#[derive(Clone, Debug)]
pub struct VariableBindingHandle<'cb, DT, MUT>
  where DT: ExprDataType, MUT: VariableMutability
{
  name: IdentifierModel,
  _phantom: PhantomData<&'cb (DT, MUT)>,
}
impl<'cb, DT, MUT> VariableBindingHandle<'cb, DT, MUT>
  where DT: ExprDataType, MUT: VariableMutability
{
  /** Create a new buffer binding handle. */
  pub(crate) fn new(name: IdentifierModel) -> Self {
    VariableBindingHandle { name, _phantom: PhantomData }
  }

  /**
   * Create an assignable lvalue handle.
   */
  pub fn read<'cb2>(&self) -> ExprHandle<'cb2, DT>
  where DT: ExprDataType,
        'cb: 'cb2,
  {
    let ident_expr_model =
      IdentifierExprModel::new(self.name.clone(), DT::repr());
    let expr_model = ExpressionModel::Identifier(ident_expr_model);
    ExprHandle::new(Box::new(expr_model))
  }
}

impl<'cb, DT> VariableBindingHandle<'cb, DT, VariableReadWrite>
  where DT: ExprDataType
{
  /**
   * Create an assignable lvalue handle.
   */
  pub fn lvalue<'cb2>(&self) -> LvalueHandle<'cb2, DT>
  where DT: ExprDataType,
        'cb: 'cb2,
  {
    let model = LvalueModel::new_variable(self.name.clone(), DT::repr());
    LvalueHandle::new(model)
  }
}
