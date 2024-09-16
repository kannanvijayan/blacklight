use std::marker::PhantomData;
use crate::{
  api::{
    data_type::{
      ArgTupleDataType,
      ArgTupleHandleMap,
      ArgTupleHandleVisitor,
      ExprDataType,
      ProcResultType,
    },
    handle::ExprHandle,
  },
  model::{ IdentifierModel, ExpressionModel, FunctionCallExprModel },
};

/**
 * A handle to a function declared within a shader module.
 */
pub struct FunctionHandle<'sh, ARG, RET>
  where ARG: ArgTupleDataType + ArgTupleHandleMap<'sh>,
        RET: ProcResultType,
{
  name: IdentifierModel,
  _phantom: PhantomData<&'sh (ARG, RET)>,
}
impl<'sh, ARG, RET> FunctionHandle<'sh, ARG, RET>
  where ARG: ArgTupleDataType + ArgTupleHandleMap<'sh>,
        RET: ProcResultType,
{
  /** Create a new function handle. */
  pub(crate) fn new(name: IdentifierModel) -> Self {
    FunctionHandle { name, _phantom: PhantomData }
  }

  /** Create a call expression from this function and argument expressions.
   * A void call is a statement and added from the codeblock builder api.
   */
  pub fn call<'cb>(&self, args: ARG::HandleTuple) -> ExprHandle<'cb, RET>
    where 'sh: 'cb,
          RET: ExprDataType,
  {
    let mut visitor = MakeArgumentExprVectorVisitor::new();
    ARG::visit_argument_handles(args, &mut visitor);
    let function_call_model =
      FunctionCallExprModel::new(self.name.clone(), visitor.args, RET::repr());
    let expression_model = ExpressionModel::FunctionCall(function_call_model);
    ExprHandle::new(Box::new(expression_model))
  }
}

struct MakeArgumentExprVectorVisitor {
  args: Vec<Box<ExpressionModel>>,
}
impl MakeArgumentExprVectorVisitor {
  fn new() -> Self {
    MakeArgumentExprVectorVisitor { args: Vec::new() }
  }
}
impl<'a> ArgTupleHandleVisitor<'a> for MakeArgumentExprVectorVisitor {
  fn visit_arg<ARG>(&mut self, handles: ExprHandle<'a, ARG>)
      where ARG: ExprDataType
  {
    self.args.push(handles.model);
  }
}
